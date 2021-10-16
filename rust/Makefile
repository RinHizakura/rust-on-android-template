NDK_PREBUILT_HOME = '..\/..\/..\/Android\/Sdk\/ndk\/23.0.7599858\/toolchains\/llvm\/prebuilt'

build:
	cp scripts/config .cargo/.
	sed -i s/NDK_PREBUILT_HOME/${NDK_PREBUILT_HOME}/g .cargo/config
	cargo b

clean:
	rm -rf .config/config
	cargo c
