# USAGE: build -d myFile myTitle can be multi words
#      : build ../../ myFile myTitle can be multi words
#TODO: make it work with multiwords dir names     
LOCAL=$1
TARGET=$PWD/$2
shift;
TITLE=$*

if [ $LOCAL == "-d" ]; then
    STYLE=https://razafy.com/css/styling.css
else
    STYLE=${LOCAL}css/styling.css
fi

pandoc -s -V "pagetitle:$TITLE" --css=$STYLE -f markdown+smart ${TARGET}md -o ${TARGET}html --toc --toc-depth=2

echo -e $TITLE "\n@" $TARGET "\n---- DONE!"


