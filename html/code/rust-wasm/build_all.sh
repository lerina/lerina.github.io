for file in *.md; do
  #MK_RAZ -d $file $file
  MK_RAZ -d ${file%.*}. ${file%.*}.
done
