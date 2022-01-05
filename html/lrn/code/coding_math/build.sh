#!/bin/sh

# USAGE:
# ./build.sh <target_name>
# ex:
# ./build.sh ep002
TARGET=$1
set -ex

wasm-pack build --target web

if [ -d "$TARGET" ]; then
    printf '%s\n' "Removing previous ($TARGET)"
    rm -rf "$TARGET"
fi
mkdir $TARGET
cp pkg/* $TARGET

printf '%s\n' "serving page at: http://127.0.0.1:8080"
#python3 -m http.server
http -a 127.0.0.1 -p 8080
