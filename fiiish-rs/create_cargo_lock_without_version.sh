#!/bin/sh

sed -r -E '/^name = "fiiish-rs"$/,/^dependencies = \[$/s/^version = ".*"$/version = ""/' Cargo.lock > Cargo.lock-no_version
