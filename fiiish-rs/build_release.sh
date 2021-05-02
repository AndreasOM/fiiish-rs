#!/bin/sh

# :TODO: get real script path
base_dir="."

platform=$1
suffix=$2
## version=$2
version=$(cargo get version)
fullversion=v${version}-${suffix}


echo Building ${platform} v${version}-${suffix}
read -p "Press enter to continue"


release_dir=${base_dir}/../release/${platform}-${suffix}/${fullversion}

echo ${release_dir}
mkdir -p ${release_dir}


# :TODO: fix for all platforms

# build M1
cargo build --release --target aarch64-apple-darwin

# build Intel
cargo build --release --target x86_64-apple-darwin

lipo -create -output ${release_dir}/fiiish target/aarch64-apple-darwin/release/fiiish target/x86_64-apple-darwin/release/fiiish


git tag -a ${fullversion} -m "+ Tag ${fullversion}"
git push origin ${fullversion}
