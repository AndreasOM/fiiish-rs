#!/bin/sh

echo ":WIP: :IDEA: don't use yet"

ver="v$(cargo get version)"

sed '/## DONE/,$d'  TODO.md > BEFORE_DONE.md
sed -n '/## Released/,$p' TODO.md|sed '/## Released/,1d' > AFTER_RELEASED.md
# sed -n '/## DONE/,/## Released/p' TODO.md|sed '/## DONE/,1d'|sed '/## Released/,1d' > MIDDLE.md
sed -n '/## DONE/,/## Released/p' TODO.md|sed '/## DONE/,1d'|sed '/## Released/,1d' > MIDDLE.md

cat BEFORE_DONE.md > TODO_NEW.md
echo "## DONE" >> TODO_NEW.md
echo "## Released" >> TODO_NEW.md
echo "## ${ver}" >> TODO_NEW.md
cat MIDDLE.md >> TODO_NEW.md
cat AFTER_RELEASED.md >> TODO_NEW.md

rm BEFORE_DONE.md
rm MIDDLE.md
rm AFTER_RELEASED.md
