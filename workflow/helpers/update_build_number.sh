#!/bin/sh
dirname "$0"
bn_file=$(dirname "$0")/../../build_number.txt

echo $bn_file

git rev-list --count HEAD >${bn_file}

