#!/bin/bash
#
clear
echo "Building " $1
t=$(basename $1 .md)
echo "Title: " ${t}
TITLE="--metadata pagetitle="${t}
pandoc -t html5 $1.md -s -c css/styles.css --highlight-style monochrome -o $1.html ${TITLE}
echo "Done:"
ls -lh $1$2*
