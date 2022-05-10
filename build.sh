TARGET=$PWD/$1
shift;
TITLE=$*

STYLE=https://razafy.com/css/styling.css

pandoc -s -V "pagetitle:$TITLE" --css=$STYLE -f markdown+smart $TARGET.md -o $TARGET.html
