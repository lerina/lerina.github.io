#!/bin/bash
#
pandoc -t html5 $1$2md -s --toc -c css/styles.css --highlight-style pygments -o $1$2html 
echo ""
echo "ctrl-c to stop server:"
python3 -m http.server --bind localhost  8000
