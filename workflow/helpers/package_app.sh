#!/bin/bash
script_dir=$(dirname "$0")

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


case ${platform} in
	"apple-darwin")
		echo "apple-darwin"
		${script_dir}/apple-darwin/package_app_for_apple-darwin_app.sh ${platform} ${target} ${package_type} ${parts_folder} ${output_folder}
		;;
	*)
		echo "Unsupported platform: ${platform}"
		exit -1
		;;
esac