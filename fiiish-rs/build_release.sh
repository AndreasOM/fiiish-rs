#!/bin/sh

# :TODO: get real script path
base_dir="."

platform=$1
suffix=${2:-dev}
## version=$2
version=$(cargo get version)
fullversion=v${version}-${suffix}

if [ "x${suffix}" == "xdev" ]
then
	echo "Building DEV"
elif [ "x${suffix}" == "xalpha" ]
then
	echo "Building ALPHA"
elif [ "x${suffix}" == "xbeta" ]
then
	echo "Building BETA"
elif [ "x${suffix}" == "xrelease" ]
then
	echo "Building RELEASE"
else
	echo "Unsupported suffix ${suffix}"
	exit -1
fi

git diff --quiet
rc=$?
if [ $rc -ne 0 ]
then
	echo "Not building release from dirty repository, either commit or stash your changes"
	exit -1
fi

echo Building ${platform} v${version}-${suffix}
read -p "Press enter to continue"


release_dir=${base_dir}/../release/${platform}-${suffix}/${fullversion}

echo ${release_dir}
mkdir -p ${release_dir}

function build_osx {

	release_dir=$1

	# build M1
	cargo build --release --target aarch64-apple-darwin

	# build Intel
	cargo build --release --target x86_64-apple-darwin

	lipo -create -output ${release_dir}/fiiish target/aarch64-apple-darwin/release/fiiish target/x86_64-apple-darwin/release/fiiish

}

# :TODO: fix for all platforms

if [ "x${platform}" == "xosx" ]
then
	build_osx ${release_dir}
else
	echo "Unsupported platform ${platform}"
	exit -1
fi


git tag -f -a ${fullversion} -m "+ Tag ${fullversion}"
git push -f origin ${fullversion}
