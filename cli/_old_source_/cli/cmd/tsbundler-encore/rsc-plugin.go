package main

import (
	"bytes"
	"github.com/evanw/esbuild/pkg/api"
	"os"
)

type RSCPlugin struct {

}

plugin := api.Plugin{
Name: "rsc",
Setup: func(build api.PluginBuild) {
// build.OnResolve, build.OnLoad, etc.
},
}
result := api.Build(api.BuildOptions{
EntryPoints: []string{"src/index.tsx"},
Bundle:      true,
Plugins:     []api.Plugin{plugin},
// … other options …
})

build.OnResolve(api.OnResolveOptions{
Filter: `\.(js|jsx|ts|tsx)$`,
}, func(args api.OnResolveArgs) (api.OnResolveResult, error) {
// Peek at file contents to detect `"use server"` vs `"use client"`
src, _ := os.ReadFile(args.Path)
if bytes.HasPrefix(bytes.TrimSpace(src), []byte(`"use server"`)) {
return api.OnResolveResult{Path: args.Path, Namespace: "rsc-server"}, nil
}
if bytes.HasPrefix(bytes.TrimSpace(src), []byte(`"use client"`)) {
return api.OnResolveResult{Path: args.Path, Namespace: "rsc-client"}, nil
}
// Fallback to default resolution
return api.OnResolveResult{}, nil
})

// Server components → serialize for RSC
build.OnLoad(api.OnLoadOptions{
Filter:    ".*",
Namespace: "rsc-server",
}, func(args api.OnLoadArgs) (api.OnLoadResult, error) {
src, _ := os.ReadFile(args.Path)
// Here you’d invoke React’s server transform (e.g. via
// react-server-dom-webpack’s transform API) to produce
// a module manifest and JS entrypoint.
transformed := runServerTransform(string(src), args.Path)
return api.OnLoadResult{
Contents: &transformed,
Loader:   api.LoaderJS,
}, nil
})

// Client components → strip server-only code
build.OnLoad(api.OnLoadOptions{
Filter:    ".*",
Namespace: "rsc-client",
}, func(args api.OnLoadArgs) (api.OnLoadResult, error) {
src, _ := os.ReadFile(args.Path)
// You can either leave TSX alone (client loader) or
// strip out `"use server"` blocks via a simple regex.
return api.OnLoadResult{
Contents: &srcString,
Loader:   api.LoaderTSX,
}, nil
})
