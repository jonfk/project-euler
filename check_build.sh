#!/usr/bin/env bash

set -e
set -x

rm -r build || true
mkdir build
cd build

# Go
mkdir go
cd go
find ../.. -name "*.go" -print0 | xargs -0L1 go build
cd ..

# Haskell
mkdir haskell
cd haskell
find ../.. -name "*.hs" -print0 | xargs -0L1 ghc
cd ..

cd ..
rm -r build
