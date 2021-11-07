pandoc --css=styling.css -s -f markdown+smart --metadata pagetitle=$1 --to=html5 $1.md -o $1.html
