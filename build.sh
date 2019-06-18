#!/bin/bash
#
clear
echo "Building " $1$2
pandoc -t html5 $1$2md -s --toc -c css/styles.css  --highlight-style monochrome -o $1$2html 
echo "Done:"
ls -lh $1$2*
