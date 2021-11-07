#!/bin/sh

set -ex

wasm-pack build --target web
#python3 -m http.server
http -a 127.0.0.1 -p 8080
