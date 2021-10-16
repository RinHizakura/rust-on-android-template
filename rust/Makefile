NDK_PREBUILT_HOME = '..\/..\/..\/Android\/Sdk\/ndk\/22.0.7026061\/toolchains\/llvm\/prebuilt'

build:
	cp scripts/config .cargo/.
	sed -i s/NDK_PREBUILT_HOME/${NDK_PREBUILT_HOME}/g .cargo/config
	cargo b

clean:
	rm -rf .config/config
	cargo c
