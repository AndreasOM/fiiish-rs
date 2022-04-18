
APP=fiiish-rs
BIN=fiiish

argc=$#
if [ $argc -lt 5 ]
then
	echo "Usage: $0 [platform] [target] [package_type] [parts_folder] [output_folder] [:TODO: data_variant]"
	exit -1
fi
platform=$1
shift 1

target=$1
shift 1

package_type=$1
shift 1

parts_folder=$1
shift 1

output_folder=$1
shift 1

if [ "${platform}" != "apple-darwin" ]
then
	echo "This script can only package for apple-darwin, and not for ${platform}"
	exit -1
fi

if [ "${package_type}" != ".app" ]
then
	echo "This script can only package for .app, and not for ${package_type}"
	exit -1
fi

echo ":TODO:"
ls -R ${parts_folder}

# :TODO: get version from version.txt
version="v0.0.0-test"
# :TODO decide between macos and apple-darwin
#app_dir=${output_folder}/${app}-macos-${version}/${app}.app
app_dir=${output_folder}/${APP}.app

echo "Creating folder structure in ${app_dir}"
mkdir -p ${app_dir}

mkdir -p ${app_dir}/Contents/MacOS	# save a few mkdir calls by starting with a deep directory
mkdir -p ${app_dir}/Contents/Resources

echo "Copying in runtime"
cp ${parts_folder}/runtime-${target}/${BIN} ${app_dir}/Contents/MacOS/

echo "Copying in Info.plist"
cp ${parts_folder}/runtime-${target}/Info.plist ${app_dir}/Contents/

echo "Copying in AppIcon"
cp ${parts_folder}/runtime-${target}/AppIcon.icns ${app_dir}/Contents/Resources/
cp ${parts_folder}/runtime-${target}/Assets.car ${app_dir}/Contents/Resources/

echo "Copying in 'extras'"
cp ${parts_folder}/runtime-${target}/build_number.txt ${app_dir}/Contents/Resources/
cp ${parts_folder}/runtime-${target}/version.txt ${app_dir}/Contents/Resources/

echo "Adding data"
cp ${parts_folder}/fiiish-data/fiiish-data.omar ${app_dir}/Contents/Resources/
cp ${parts_folder}/dummy-data/dummy-data.omar ${app_dir}/Contents/Resources/

