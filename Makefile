build:
	cross build --target aarch64-linux-android --release

install: build
	adb push target/aarch64-linux-android/release/mmrl /data/mkuser/usr/bin
# rustup target add aarch64-linux-android

module: build
	bash build-module.sh