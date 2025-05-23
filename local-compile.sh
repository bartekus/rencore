#!/usr/bin/env bash

#cargo install cargo-zigbuild
#brew install openssl
#brew install zig
#
#set -euo pipefail
#
## Build encore dependencies
#echo "Building encore local binary..."
#go run ./pkg/encorebuild/cmd/build-local-binary/ all
#echo "✅ Encore local binary successfully built."
#
#
#echo "Creating encore js runtimes..."
#(cd runtimes/js && cargo zigbuild --release --jobs 6)
#echo "✅ Encore js runtimes successfully built."
#
## Build TypeScript parser
#echo "Building TypeScript parser..."
#(cd tsparser && cargo zigbuild --release --jobs 6)
#echo "✅ TypeScript parser successfully built."
#
#mkdir ./target/release/bin || true &&
#mkdir ./target/release/runtimes || true &&
#mkdir ./target/release/runtimes/core || true &&
#mkdir ./target/release/runtimes/go || true &&
#mkdir ./target/release/runtimes/js || true &&
#mkdir ./target/release/runtimes/js/encore.dev || true &&
#
## Create encore binary and other Go binaries
#echo "Creating encore binary..."
#go build ./cli/cmd/encore &&
#mv ./encore ./target/release/bin &&
#chmod +x ./target/release/encore &&
#echo "✅ encore binary successfully built."
#
#echo "Creating git-remote-encore binary..."
#go build ./cli/cmd/git-remote-encore &&
#mv ./git-remote-encore ./target/release &&
#chmod +x ./target/release/git-remote-encore &&
#echo "✅ git-remote-encore binary successfully built."

echo "Creating tsbundler-encore binary..."
go build ./cli/cmd/tsbundler-encore &&
mv ./tsbundler-encore ./target/release
chmod +x ./target/release/tsbundler-encore &&
echo "✅ tsbundler-encore binary successfully built."

#echo "Copying runtimes..."
#cp -r "./runtimes/core" "./target/release/runtimes/" &&
#cp -r "./runtimes/go" "./target/release/runtimes/go/" &&
#cp -r "./runtimes/js/encore.dev" "./target/release/runtimes/js/encore.dev/" &&
#cp "./runtimes/js/encore-runtime.node" "./target/release/runtimes/js/" &&
#echo "✅ Runtimes successfully copied."
