use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{
    generate,
    shells::{Bash, Fish, Zsh},
};
use std::path::PathBuf;

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;
use utils::types::LogLevel;

#[derive(Parser, Debug)]
#[command(
    name = "rencore",
    author,
    about,
    long_about = "Rust Starter CLI",
    version
)]
//TODO: #[clap(setting = AppSettings::SubcommandRequired)]
//TODO: #[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    /// Set a custom config file
    /// TODO: parse(from_os_str)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Set a custom config file
    #[arg(name = "debug", short, long = "debug", value_name = "DEBUG")]
    pub debug: Option<bool>,

    /// Set Log Level
    #[arg(
        name = "log_level",
        short,
        long = "log-level",
        value_name = "LOG_LEVEL"
    )]
    pub log_level: Option<LogLevel>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(
        name = "check",
        about = "Check the application using the daemon",
        long_about = None,
    )]
    Check {
        #[clap(long, help = "Enable codegen debug")]
        codegen_debug: bool,
        #[clap(long, help = "Parse tests")]
        parse_tests: bool,
    },
    #[clap(
        name = "bundle",
        about = "Bundle TypeScript/JavaScript entrypoints",
        long_about = None,
    )]
    Bundle {
        #[clap(short, long, value_name = "ENTRYPOINT")]
        entrypoint: PathBuf,
        #[clap(short, long, value_name = "OUTDIR")]
        outdir: PathBuf,
        // ... any other options ...
    },
    #[clap(
        name = "hazard",
        about = "Generate a hazardous occurance",
        long_about = None, 
    )]
    Hazard,
    #[clap(
        name = "error",
        about = "Simulate an error",
        long_about = None, 
    )]
    Error,
    #[clap(
        name = "completion",
        about = "Generate completion scripts",
        long_about = None,
        )]
    Completion {
        #[clap(subcommand)]
        subcommand: CompletionSubcommand,
    },
    #[clap(
        name = "config",
        about = "Show Configuration",
        long_about = None,
    )]
    Config,
    #[clap(
        name = "run",
        about = "Run the application",
        long_about = None,
    )]
    Run {
        #[clap(short, long, value_name = "WATCH")]
        watch: bool,
        #[clap(short, long, value_name = "PORT")]
        port: Option<u16>,
        // ... other options ...
    },
}

#[derive(Subcommand, PartialEq, Debug)]
enum CompletionSubcommand {
    #[clap(about = "generate the autocompletion script for bash")]
    Bash,
    #[clap(about = "generate the autocompletion script for zsh")]
    Zsh,
    #[clap(about = "generate the autocompletion script for fish")]
    Fish,
}

pub fn cli_match() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli.config.as_deref())?;

    let app = Cli::command();
    let matches = app.get_matches();

    AppConfig::merge_args(matches)?;

    // Execute the subcommand
    match &cli.command {
        Commands::Check { codegen_debug, parse_tests } => {
            // Run the async check function using a runtime
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(commands::check(*codegen_debug, *parse_tests))?;
        },
        
        Commands::Bundle { entrypoint, outdir } => commands::bundle(entrypoint, outdir)?,
        // Commands::Run { watch, listen, port, json, namespace, color, debug, browser } => commands::run(watch, listen, port, json, namespace, color, debug, browser)?,
        Commands::Hazard => commands::hazard()?,
        Commands::Error => commands::simulate_error()?,
        Commands::Completion { subcommand } => {
            let mut app = Cli::command();
            match subcommand {
                CompletionSubcommand::Bash => {
                    generate(Bash, &mut app, "rencore", &mut std::io::stdout());
                }
                CompletionSubcommand::Zsh => {
                    generate(Zsh, &mut app, "rencore", &mut std::io::stdout());
                }
                CompletionSubcommand::Fish => {
                    generate(Fish, &mut app, "rencore", &mut std::io::stdout());
                }
            }
        }
        Commands::Config => commands::config()?,
        Commands::Run { watch, port } => commands::run(watch, port)?,
    }

    Ok(())
}

pub fn run(watch: bool, port: Option<u16>) -> Result<()> {
    // Your logic here
    Ok(())
}
