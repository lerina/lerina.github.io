#!/bin/bash
#
echo ""
echo "ctrl-c to stop server:"
#python3 -m http.server --bind localhost  8000
http -p 8080 $1
