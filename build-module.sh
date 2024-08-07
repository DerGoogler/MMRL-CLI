#!/bin/bash

# run checks
if ! command -v dos2unix >/dev/null; then
    echo "Please install dos2unix (sudo apt-get install dos2unix)"
    exit 1
fi

# Go to work dir
cd module

get_prop() { cat ./../Cargo.toml | grep -Po "(?<=^$1 = \")[^\"]*(?=\".*)"; }

NAME=$(get_prop name)
VER=$(get_prop version)
AUT=$(get_prop author)
LIC=$(get_prop license)
VER_CODE="${VER//./}"
BUILD_DATE=$(date "+%Y-%m-%d")
RUST_VER=$(rustc --version | grep -oP '\d+\.\d+\.\d+')

# generate module.prop
cat <<EOF >module.prop
id=$NAME
name=MMRL CLI
version=$VER
versionCode=$VER_CODE
author=$AUT
description=MMRL Command Line Interface is a free tool to install Magisk/KernelSU modules. Build on $BUILD_DATE with Rust $RUST_VER.
updateJson=https://raw.githubusercontent.com/DerGoogler/MMRL-CLI/master/module/update.json
license=$LIC
EOF

# generate update.json
cat <<EOF >update.json
{
    "version": "$VER",
    "versionCode": "$VER_CODE",
    "zipUrl": "https://github.com/DerGoogler/MMRL-CLI/releases/download/v$VER/mmrl-$VER-module-aarch64.zip",
    "changelog": "https://raw.githubusercontent.com/DerGoogler/MMRL-CLI/master/CHANGELOG.md"
}
EOF

cat <<EOF >system/usr/share/mmrl/config/mmrl/info.json
{
    "version": "$VER",
    "versionCode": "$VER_CODE",
    "author": "$AUT",
    "rustVersion": "$RUST_VER",
    "buildDate": "$BUILD_DATE"
}
EOF

cp ../CHANGELOG.md .
cp ../target/aarch64-linux-android/release/mmrl system/bin/mmrl

FILE_NAME="$NAME-$VER-module-aarch64.zip"

# convert files before zipping
dos2unix \
    META-INF/com/google/android/update-binary \
    META-INF/com/google/android/updater-script \
    *.sh

zip -r "../target/$FILE_NAME" ./* -x "system/bin/placeholder"

rm -rf "system/bin/mmrl" "CHANGELOG.md"
