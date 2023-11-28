#!/bin/sh

set -ex

# compile for plain vanilla no javascript framework 
wasm-pack build --target web --out-name index --no-typescript --out-dir www/pkg

# display link for easy access
echo "Serving at: http://127.0.0.1:8080/html/"

# run the web server
http -a 127.0.0.1 -p 8080 www

