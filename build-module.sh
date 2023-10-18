#!/bin/bash

# Go to work dir
cd module 

get_prop() { cat ./../Cargo.toml | grep -Po "(?<=^$1 = \")[^\"]*(?=\".*)"; }

NAME=$(get_prop name)
VER=$(get_prop version)
VER_CODE="${VER//./}"
BUILD_DATE=`date "+%Y-%m-%d"`
RUST_VER=$(rustc --version | grep -oP '\d+\.\d+\.\d+')

arr=(
    "id=$NAME"
    "name=MMRL CLI"
    "version=$VER"
    "versionCode=$VER_CODE"
    "author=Der_Googler"
    "description=MMRL Command Line Interface is a free tool to install Magisk/KernelSU modules. Build on $BUILD_DATE with Rust $RUST_VER."
);
printf '%s\n' "${arr[@]}" > module.prop

cp ./../target/aarch64-linux-android/release/mmrl system/bin/mmrl

FILE_NAME="$NAME-$VER-module-aarch64.zip"

zip -r "./../target/$FILE_NAME" ./*

rm -rf system/bin/mmrl