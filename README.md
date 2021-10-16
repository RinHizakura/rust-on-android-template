# rust-on-android template
## flow

We'll need toolchain to compile our codes for the android target.
```
$ rustup target add armv7-linux-androideabi  # For 32-bit ARM.
$ rustup target add aarch64-linux-android    # For 64-bit ARM.
```

We'll need [android studio](https://developer.android.com/studio) to manage the interface
of APPs. It also helps to install NDK (Native Development Kit) and its associated tool.
From *Tools -> SDK manager -> SDK tools*, download the following tools:

![](image/SDK_manager.png)
