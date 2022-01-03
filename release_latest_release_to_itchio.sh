#!/bin/sh

gh release list # -R andreasOM/fiiish-rs

version=$(gh release list|grep Latest| awk '{print $1}')

zip_name="Fiiish-RS-macos-app-${version}.zip"
if [ ! -f releases/${zip_name} ]
then
	echo "Downloading ${version}"

	mkdir -p releases
	gh release download $version -D releases -p "Fiiish-RS-macos-app-${version}.zip"

	echo "Unpacking ${version} (macos-app)"
	cd releases
	unzip Fiiish-RS-macos-app-${version}.zip


	suffix=$(echo ${version}|cut -d'-' -f2)
	if [ x${suffix} == xtest ]
	then
		echo "Releasing -test to itch.io"
		~/bin/butler/butler push Fiiish-RS-macos-${version}/fiiish-rs.app omni-mad/fiiish:osx-test --userversion=${version}
	else
		echo "Unsupported suffix ${suffix}"
	fi
	exit
	~/bin/butler/butler push Fiiish-RS-macos-${version}/fiiish-rs.app omni-mad/fiiish:osx-alpha --userversion=${version}
else
	echo "File already exists. Skipping!"
fi
