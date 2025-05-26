#!/usr/bin/env bash

#cargo install cargo-zigbuild
#brew install openssl
#brew install zig
#
set -euo pipefail
#
## Build encore dependencies
#echo "Building encore local binary..."
#go run ./pkg/encorebuild/cmd/build-local-binary/ all
#echo "✅ Encore local binary successfully built."
#rm -rf ./target

echo "Creating encore js runtimes..."
(cd runtimes/js && cargo zigbuild --release --jobs 6)
echo "✅ Encore js runtimes successfully built."

# Build TypeScript parser
echo "Building TypeScript parser..."
(cd tsparser && cargo zigbuild --release --jobs 6)
echo "✅ TypeScript parser successfully built."

