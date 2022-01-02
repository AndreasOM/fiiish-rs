#!/bin/sh

app=${APP_NAME}
binary=fiiish
binary_dir=fiiish-rs

platform=macos


# TODO: make this check if cargo get is available
# faster than installing cargo-get:
version=$(grep version ${binary_dir}/Cargo.toml|cut -d"\"" -f2)
# version=$(cd ${binary_dir} && cargo get version)
echo "Version: ${version}"

build_number=$(git rev-list ${version} --count)
echo "Build number: ${build_number}"


app_dir=package/${app}-${platform}-${version}/fiiish-rs.app

echo "Creating folder structure in ${app_dir}"
mkdir -p ${app_dir}

mkdir -p ${app_dir}/Contents/MacOS	# save a few mkdir calls by starting with a deep directory
mkdir -p ${app_dir}/Contents/Resources

echo "Combining binaries into fat binaries"
lipo -create -output ${app_dir}/Contents/MacOS/${binary} \
	${binary_dir}/target/aarch64-apple-darwin/release/${binary} \
	${binary_dir}/target/x86_64-apple-darwin/release/${binary}

echo "Patching up Info.plist"

exp1="s/<key>CFBundleVersion<\\/key><string>.*<\\/string>/<key>CFBundleVersion<\\/key><string>${build_number}<\\/string>/g"
exp2="s/<key>CFBundleShortVersionString<\\/key><string>.*<\\/string>/<key>CFBundleShortVersionString<\\/key><string>${version}<\\/string>/g"
cat ${binary_dir}/Info.plist|sed ${exp1} |sed ${exp2} > ${app_dir}/Contents/Info.plist


echo "Compiling AppIcon"
temp=$(mktemp -d)
# TODO: hard coded path is probably a bad idea
/Applications/Xcode.app/Contents/Developer/usr/bin/actool \
	--compile ${app_dir}/Contents/Resources/ \
	--app-icon AppIcon \
	--platform macosx \
	--minimum-deployment-target 10.0 \
	${binary_dir}/Assets.xcassets \
	--output-partial-info-plist ${temp}/Info.plist		# :TODO: this should be merged with Info.plist above

cat ${temp}/Info.plist

rm -r ${temp}

echo "Packing data"
cd fiiish-data
ls -1 |grep -v paklist.txt >paklist.txt
cd -
omt-packer pack --basepath fiiish-data --output fiiish-data.omar --paklist fiiish-data/paklist.txt

cd dummy-data
ls -1 |grep -v paklist.txt >paklist.txt
cd -
omt-packer pack --basepath dummy-data --output dummy-data.omar --paklist dummy-data/paklist.txt


echo "Adding resources"
cp fiiish-data.omar ${app_dir}/Contents/Resources/
cp dummy-data.omar ${app_dir}/Contents/Resources/


