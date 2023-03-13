#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use crate::hello;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::CString;
use chrono::DateTime;
use chrono::Utc;
// NOTE: RustKt references the name rusty.kt, which will be the kotlin file exposing the functions below.
// Remember the JNI naming conventions.

//static mut end_time: DateTime<Utc>;
#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_helloDirect(
  env: JNIEnv,
  _: JClass,
  input: JString,
) -> jstring {
  let input: String = env
    .get_string(input)
    .expect("Couldn't get Java string!")
    .into();
  let output = env
    .new_string(format!("Hello from Rust1: {}", input))
    .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_oncreate(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_create"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_onstart(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_start"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_onresume(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_resume"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_onpause(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_pause"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_onrestart(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_restart"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_onstop(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_stop"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_ondestroy(
  env: JNIEnv,
  _: JClass
) -> jstring{
  let output = env
  .new_string(format!("Rust: on_destroy"))
  .expect("Couldn't create a Java string!");
  output.into_inner()
}

#[allow(clippy::similar_names)]
#[no_mangle]
pub extern "system" fn Java_com_example_rusty_1android_1lib_RustyKt_hello(
  env: JNIEnv,
  _: JClass,
  input: JString,
) -> jstring {
  let java_str = env.get_string(input).expect("Couldn't get Java string!");
  // we call our generic func for iOS
  let java_str_ptr = java_str.as_ptr();
  let result = unsafe { hello(java_str_ptr) };
  // freeing memory from CString in ios function
  // if we call hello_release we won't have access to the result
  let result_ptr = unsafe { CString::from_raw(result) };
  let result_str = result_ptr.to_str().unwrap();
  let output = env
    .new_string(result_str)
    .expect("Couldn't create a Java string!");
  output.into_inner()
}
