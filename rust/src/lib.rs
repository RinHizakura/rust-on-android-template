use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jint, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_android_RustUtils_echostring(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        let world = rust_greeting(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
        let output = env
            .new_string(CStr::from_ptr(world).to_str().unwrap())
            .expect("Couldn't create java string!");
        rust_greeting_free(world);
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_android_RustUtils_echoint(
        env: JNIEnv,
        _: JClass,
        x: jint,
    ) -> jstring {
        let mut bytes = x.to_string().into_bytes();
        /* pending a null terminator for c string */
        bytes.push(0);
        /* does it a bad idea to turn vector into c_char array directly? */
        let world = rust_greeting(bytes.as_ptr());
        let output = env
            .new_string(CStr::from_ptr(world).to_str().unwrap())
            .expect("Couldn't create java string!");
        rust_greeting_free(world);
        output.into_inner()
    }
}
