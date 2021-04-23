#!/bin/sh


## copy individual images
## cp ../fiiish-content/fish/fish_*.png ../fiiish-data/

# no need to put background into atlas since it uses it's own shader anyway
cp ../fiiish-content/background/*.png ../fiiish-data/

## combine images ... needs omt-atlas in path
omt-atlas combine --border 1 --size 2048 \
	--output ../fiiish-data/game-atlas-%d \
	--input \
		../fiiish-content/fish/fish_*.png \
		../fiiish-content/pickups/coin/coin_*.png \
		../fiiish-content/pickups/coin_green/coin_green_*.png \
		../fiiish-content/pickups/coin_blue/coin_blue_*.png \
		../fiiish-content/pickups/magnet/magnet_*.png \
		../fiiish-content/obstacles/rocks/rock-?.png \
		../fiiish-content/obstacles/seaweed/seaweed-*.png \
		../fiiish-content/decorations/*.png \
		../fiiish-content/obstacles/blocks/block-*.png

omt-atlas combine --border 1 --size 2048 \
	--output ../fiiish-data/gui-atlas-%d \
	--input \
		../fiiish-content/gui/buttons/*.png			\
		../fiiish-content/gui/screen_frame_2.png

## now create the archives
# :TODO: fix for non unix systems
cd ../fiiish-data
ls -1 |grep -v paklist.txt >paklist.txt
cd -
omt-packer pack --basepath ../fiiish-data --output fiiish-data.omar --paklist ../fiiish-data/paklist.txt

cd ../dummy-data
ls -1 |grep -v paklist.txt >paklist.txt
cd -
omt-packer pack --basepath ../dummy-data --output dummy-data.omar --paklist ../dummy-data/paklist.txt

## preview
# omt-atlas preview --input ../fiiish-data/game-atlas-%d

## info
# omt-atlas info --input ../fiiish-data/game-atlas-%d
