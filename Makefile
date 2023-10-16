build:
	cross build --target aarch64-linux-android --release

install: build
	adb push target/aarch64-linux-android/release/mmrl /data/mkuser/usr/bin