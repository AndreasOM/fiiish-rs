#!/bin/sh


## copy individual images
## cp ../fiiish-content/fish/fish_*.png ../fiiish-data/

# no need to put background into atlas since it uses it's own shader anyway
cp ../fiiish-content/background/*.png ../fiiish-data/

## combine images ... needs omt-atlas in path
omt-atlas combine --border 1 --size 2048 --output ../fiiish-data/game-atlas-%d --input ../fiiish-content/fish/fish_*.png ../fiiish-content/pickups/coin/coin_*.png


## preview
# omt-atlas preview --input ../fiiish-data/game-atlas-%d

## info
# omt-atlas info --input ../fiiish-data/game-atlas-%d
