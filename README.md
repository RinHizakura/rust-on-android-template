# rust-on-android template
## How to write your APP in Rust

1. We'll need toolchain to compile our codes for the android target.
```
$ rustup target add armv7-linux-androideabi  # For 32-bit ARM.
$ rustup target add aarch64-linux-android    # For 64-bit ARM.
```

2. We'll need [android studio](https://developer.android.com/studio) to manage the interface
of APPs(There are other alternatives, but I think using android studio is relatively simple).
It also helps to install NDK (Native Development Kit) and its associated tool.
From *Tools -> SDK manager -> SDK tools*, download the following tools:

![](image/SDK_manager.png)

note: The ndk version over r23 could not work. You should try the older one.

3. In the `rust` directory, open the `Makefile` file and fix the path youself:
* `NDK_PREBUILT_HOME`: the path of the prebuilt llvm toolchains
* `JNILIB_ARM*_DIR`: the path to put your shared object for your android project
* `ARM*_TARGET_DIR`: the path of your shared object which is built by rust

4. Then you can run `make build32` or `make build64` to compile the rust codes(see the
`rust` directory or the following references to understand how to write these codes) into
shared object, and copy it in the correct directory automatically.

5. Finally, build your android project for th APK to run application on your phone(see the
`android` or the following references to khow how to link the shared object in android codes)!

## Reference

* [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
* [Running Rust on Android](https://blog.svgames.pl/article/running-rust-on-android)
* [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs)
* [rust-on-android](https://github.com/suve/rust-on-android/)
* [cross-platform-rust](https://github.com/fluffyemily/cross-platform-rust)
