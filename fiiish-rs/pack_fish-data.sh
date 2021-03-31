#!/bin/sh


## copy individual images
cp ../fiiish-content/fish/fish_*.png ../fiiish-data/

## combine images ... needs omt-atlas in path
omt-atlas combine --border 1 --size 2048 --output ../fiiish-data/game-atlas-%d --input ../fiiish-content/fish/fish_*.png


## preview
# omt-atlas preview --input ../fiiish-data/game-atlas-%d

## info
# omt-atlas info --input ../fiiish-data/game-atlas-%d
