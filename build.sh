#!/bin/bash
#
pandoc -t html5 $1.md -s --toc -c css/styles.css --highlight-style pygments -o $1.html 
echo ""
echo "ctrl-c to stop server:"
python3 -m http.server
