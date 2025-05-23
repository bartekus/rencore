use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use swc_common::comments::{Comments, NoopComments, SingleThreadedComments};
use swc_common::errors::Handler;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, Span, Spanned};
use swc_ecma_ast as ast;
use swc_ecma_ast::EsVersion;
use swc_ecma_loader::resolve::Resolve;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, Syntax};
use swc_ecma_visit::FoldWith;
use thiserror::Error;

use crate::parser::fileset::SourceFile;
use crate::parser::{FilePath, FileSet, Pos};

// File extensions that should be parsed as modules
const MODULE_EXTENSIONS: &[&str] = &["js", "ts", "mjs", "mts", "cjs", "cts", "jsx", "tsx"];

/// A unique id for a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(pub usize);

pub struct ModuleLoader {
    errs: Lrc<Handler>,
    file_set: Lrc<FileSet>,
    resolver: Box<dyn Resolve>,
    encore_gen_root: PathBuf,
    by_path: RefCell<HashMap<FilePath, Lrc<Module>>>,

    // The universe module, if it's been loaded.
    universe: OnceCell<Lrc<Module>>,

    /// The generated encore.gen/clients module.
    encore_app_clients: OnceCell<Lrc<Module>>,
    /// The generated encore.gen/auth module.
    encore_auth: OnceCell<Lrc<Module>>,
}

impl std::fmt::Debug for ModuleLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleLoader")
            .field("file_set", &self.file_set)
            .field("mods", &self.by_path)
            .finish()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to resolve module {0}")]
    UnableToResolve(String, #[source] anyhow::Error),
    #[error("invalid filename {0}")]
    InvalidFilename(FileName),
    #[error("unable to load file from filesystem")]
    LoadFile(#[source] io::Error),
    #[error("error when parsing module")]
    ParseError(swc_ecma_parser::error::Error),
}

impl Error {
    pub fn span(&self) -> Option<Span> {
        match self {
            Error::UnableToResolve(..) | Error::InvalidFilename(_) | Error::LoadFile(_) => None,
            Error::ParseError(e) => Some(e.span()),
        }
    }

    pub fn msg(&self) -> String {
        match self {
            Error::UnableToResolve(s, source) => {
                format!("unable to resolve module {}: {:?}", s, source)
            }
            Error::InvalidFilename(_) | Error::LoadFile(_) => self.to_string(),
            Error::ParseError(e) => e.clone().into_kind().msg().to_string(),
        }
    }
}

impl ModuleLoader {
    pub fn new(
        errs: Lrc<Handler>,
        file_set: Lrc<FileSet>,
        resolver: Box<dyn Resolve>,
        app_root: PathBuf,
    ) -> Self {
        let encore_gen_root = app_root.join("encore.gen");
        Self {
            errs,
            file_set,
            resolver,
            encore_gen_root,
            by_path: RefCell::new(HashMap::new()),
            universe: OnceCell::new(),
            encore_app_clients: OnceCell::new(),
            encore_auth: OnceCell::new(),
        }
    }

    pub fn modules(&self) -> Vec<Lrc<Module>> {
        self.by_path.borrow().values().cloned().collect::<Vec<_>>()
    }

    pub fn module_containing_pos(&self, pos: Pos) -> Option<Lrc<Module>> {
        let file = self.file_set.lookup_file(pos)?;
        let path = file.name();
        self.by_path.borrow().get(&path).cloned()
    }

    pub fn resolve_import_from_module(
        &self,
        module: &Module,
        import_path: &str,
    ) -> Result<Option<Lrc<Module>>, Error> {
        self.resolve_import(&module.swc_file_path, import_path)
    }

    pub fn resolve_import(
        &self,
        from_file: &swc_common::FileName,
        import_path: &str,
    ) -> Result<Option<Lrc<Module>>, Error> {
        // 1) Handle the special ~encore/* aliases
        if import_path == "~encore/clients" {
            return Ok(Some(self.encore_app_clients()));
        }
        if import_path == "~encore/auth" {
            return Ok(Some(self.encore_auth()));
        }

        // 2) Perform resolution (now returns Resolution)
        let resolution = self
            .resolver
            .resolve(from_file, import_path)
            .map_err(|e| Error::UnableToResolve(import_path.to_string(), e))?;

        // 3) Extract the FileName
        let file_path = match &resolution.filename {
            FileName::Real(buf) => {
                // only parse module extensions
                if let Some(ext) = buf.extension().and_then(|s| s.to_str()) {
                    if !MODULE_EXTENSIONS.contains(&ext) {
                        return Ok(None);
                    }
                }
                // check for encore.gen prefix
                if let Ok(suffix) = buf.strip_prefix(&self.encore_gen_root) {
                    if suffix.starts_with("clients/") {
                        return Ok(Some(self.encore_app_clients()));
                    }
                    if suffix.starts_with("auth/") {
                        return Ok(Some(self.encore_auth()));
                    }
                }
                FilePath::Real(buf.clone())
            }
            FileName::Custom(s) => FilePath::Custom(s.clone()),
            other => {
                // any other FileName variant is invalid for us
                return Err(Error::InvalidFilename(other.clone()));
            }
        };

        // 4) Dedupe: if already loaded, return it
        if let Some(m) = self.by_path.borrow().get(&file_path) {
            return Ok(Some(m.clone()));
        }

        // 5) Figure out if it was a relative vs. bare import
        let module_path = if import_path.starts_with("./")
            || import_path.starts_with("../")
            || import_path.starts_with('/')
        {
            None
        } else {
            Some(import_path.to_owned())
        };

        // 6) Load it
        let module = match &file_path {
            FilePath::Real(path) => self.load_fs_file(path, module_path)?,
            FilePath::Custom(_) => self.load_custom_file(file_path.clone(), "", module_path)?,
        };

        Ok(Some(module))
    }

    /// Load a file from the filesystem into the module loader.
    pub fn load_fs_file(
        &self,
        path: &Path,
        module_path: Option<String>,
    ) -> Result<Lrc<Module>, Error> {
        // Is it already stored?
        let file_name = FilePath::from(path.to_owned());
        if let Some(module) = self.by_path.borrow().get(&file_name) {
            return Ok(module.clone());
        }

        let file = self.file_set.load_file(path).map_err(Error::LoadFile)?;
        let module = self.parse_and_store(file, module_path)?;
        Ok(module)
    }

    /// Load a file from the filesystem into the module loader.
    fn load_custom_file<S: Into<String>>(
        &self,
        file_name: FilePath,
        src: S,
        module_path: Option<String>,
    ) -> Result<Lrc<Module>, Error> {
        // Is it already stored?
        if let Some(module) = self.by_path.borrow().get(&file_name) {
            return Ok(module.clone());
        }
        let file = self
            .file_set
            .new_source_file(file_name.to_owned(), src.into());
        let module = self.parse_and_store(file, module_path)?;
        Ok(module)
    }

    pub fn universe(&self) -> Lrc<Module> {
        self.universe
            .get_or_init(|| {
                let file = self
                    .file_set
                    .new_source_file(FilePath::Real("universe.ts".into()), UNIVERSE_TS.into());
                self.parse_and_store(file, Some("__universe__".into()))
                    .unwrap()
            })
            .to_owned()
    }

    pub fn encore_app_clients(&self) -> Lrc<Module> {
        self.encore_app_clients
            .get_or_init(|| {
                let file = self
                    .file_set
                    .new_source_file(FilePath::Real("encore.gen/clients".into()), "".into());
                self.parse_and_store(file, Some("encore.gen/clients".into()))
                    .unwrap()
            })
            .to_owned()
    }

    pub fn encore_auth(&self) -> Lrc<Module> {
        self.encore_auth
            .get_or_init(|| {
                let file = self
                    .file_set
                    .new_source_file(FilePath::Real("encore.gen/auth".into()), "".into());
                self.parse_and_store(file, Some("encore.gen/auth".into()))
                    .unwrap()
            })
            .to_owned()
    }

    /// Parse and store a file.
    fn parse_and_store(
        &self,
        file: Lrc<SourceFile>,
        module_path: Option<String>,
    ) -> Result<Lrc<Module>, Error> {
        let (ast, comments) = self.parse_file(file.clone())?;

        let mut mods = self.by_path.borrow_mut();
        let id = ModuleId(mods.len() + 1);

        let module = Module::new(
            self.file_set.clone(),
            id,
            file.name(),
            module_path,
            ast,
            Some(comments),
        );
        mods.insert(module.file_path.clone(), module.clone());
        Ok(module)
    }

    /// Parse a file.
    fn parse_file(
        &self,
        file: Lrc<SourceFile>,
    ) -> Result<(ast::Module, Box<SingleThreadedComments>), Error> {
        let comments: Box<SingleThreadedComments> = Box::default();

        let syntax = Syntax::Typescript(swc_ecma_parser::TsConfig {
            tsx: file.name().is_tsx(),
            dts: file.name().is_dts(),
            decorators: true,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        });

        let lexer = Lexer::new(
            syntax,
            EsVersion::Es2022,
            StringInput::from(file.as_ref()),
            Some(&comments),
        );
        let mut parser = Parser::new_from(lexer);
        for e in parser.take_errors() {
            e.into_diagnostic(&self.errs).emit();
        }

        let ast = parser.parse_module().map_err(Error::ParseError)?;

        // Resolve identifiers.
        let mut resolver = swc_ecma_transforms_base::resolver(Mark::new(), Mark::new(), true);
        let ast_module = ast.fold_with(&mut resolver);

        Ok((ast_module, comments))
    }
}

pub struct Module {
    file_set: Lrc<FileSet>,
    pub id: ModuleId,
    pub ast: swc_ecma_ast::Module,
    pub file_path: FilePath,
    pub swc_file_path: swc_common::FileName,
    /// How the module was imported, if it's an external module.
    pub module_path: Option<String>,
    pub comments: Box<dyn Comments>,
    cached_imports: OnceCell<Vec<ast::ImportDecl>>,
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("id", &self.id)
            .field("path", &self.file_path)
            .finish()
    }
}

impl Module {
    fn new(
        file_set: Lrc<FileSet>,
        id: ModuleId,
        file_path: FilePath,
        module_path: Option<String>,
        ast: ast::Module,
        comments: Option<Box<dyn Comments>>,
    ) -> Lrc<Self> {
        let comments: Box<dyn Comments> = comments.unwrap_or_else(|| Box::new(NoopComments {}));
        let swc_file_path = file_path.clone().into();
        Lrc::new(Self {
            file_set,
            id,
            ast,
            file_path,
            swc_file_path,
            module_path,
            comments,
            cached_imports: OnceCell::new(),
        })
    }

    pub fn imports(&self) -> &Vec<ast::ImportDecl> {
        self.cached_imports
            .get_or_init(move || imports_from_mod(&self.ast))
    }

    pub fn preceding_comments(&self, pos: Pos) -> Option<String> {
        self.file_set.preceding_comments(&self.comments, pos)
    }
}

impl Spanned for Module {
    fn span(&self) -> Span {
        self.ast.span
    }
}

/// imports_from_mod returns the import declarations in the given module.
fn imports_from_mod(ast: &ast::Module) -> Vec<ast::ImportDecl> {
    (ast.body)
        .iter()
        .filter_map(|it| match &it {
            ast::ModuleItem::ModuleDecl(ast::ModuleDecl::Import(imp)) => Some(imp.clone()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
impl ModuleLoader {
    /// Injects a new file into the module loader.
    /// If a file with that name has already been added it does nothing.
    pub fn inject_file(&self, path: FilePath, src: &str) -> anyhow::Result<Lrc<Module>> {
        // Check if the file has already been added if the file has a unique identity.
        // For other file types (like anonymous files) don't check for this so that we can inject
        //  multiple anonymous files for testing purposes.
        match path {
            FilePath::Real(..) => {
                if let Some(module) = self.by_path.borrow().get(&path) {
                    return Ok(module.clone());
                }
            }
            FilePath::Custom(_) => {}
        }

        use swc_common::{Globals, GLOBALS};
        let globals = Globals::new();
        GLOBALS.set(&globals, || {
            let file = self.file_set.new_source_file(path, src.into());
            let module = self.parse_and_store(file, None)?;
            Ok(module)
        })
    }

    pub fn load_archive(
        &self,
        base: &Path,
        ar: &txtar::Archive,
    ) -> anyhow::Result<HashMap<FilePath, Lrc<Module>>> {
        let mut result = HashMap::new();
        for file in &ar.files {
            if file.name.extension().is_none_or(|ext| ext != "ts") {
                continue;
            }

            let file_name = FilePath::Real(base.join(&file.name));
            let file = self.file_set.new_source_file(file_name, file.data.clone());
            let module = self.parse_and_store(file, None)?;
            result.insert(module.file_path.clone(), module);
        }

        if self.errs.has_errors() {
            Err(anyhow::anyhow!("parse error"))
        } else {
            Ok(result)
        }
    }
}

const UNIVERSE_TS: &str = include_str!("./universe.ts");
