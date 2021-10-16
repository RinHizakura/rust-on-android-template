NDK_PREBUILT_HOME = '..\/..\/..\/Android\/Sdk\/ndk\/22.0.7026061\/toolchains\/llvm\/prebuilt'

JNILIB_ARM32_DIR = ../android/app/src/main/jniLibs/armeabi
ARM32_TARGET_DIR = target/armv7-linux-androideabi/release

JNILIB_ARM64_DIR = ../android/app/src/main/jniLibs/arm64
ARM64_TARGET_DIR = target/aarch64-linux-android/release

build32:
	cp scripts/config .cargo/.
	sed -i s/NDK_PREBUILT_HOME/${NDK_PREBUILT_HOME}/g .cargo/config
	cargo b --release --target armv7-linux-androideabi
	mkdir -p ${JNILIB_ARM32_DIR}
	cp ${ARM32_TARGET_DIR}/libgreetings.so ${JNILIB_ARM32_DIR}/libgreetings.so

build64:
	cp scripts/config .cargo/.
	sed -i s/NDK_PREBUILT_HOME/${NDK_PREBUILT_HOME}/g .cargo/config
	cargo b --release --target aarch64-linux-android
	mkdir -p ${JNILIB_ARM64_DIR}
	cp ${ARM64_TARGET_DIR}/libgreetings.so ${JNILIB_ARM64_DIR}/libgreetings.so

clean:
	rm -rf .config/config
	cargo c
