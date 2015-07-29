#!/usr/bin/env bash

set -e
set -x

rm -r build
mkdir build
cd build

mkdir go
cd go
# Go
find ../.. -name "*.go" -print0 | xargs -0L1 go build
cd ..

cd ..
rm -r build
