#![no_std]
#![allow(non_snake_case)]

// Users of the bindings should include alloc and configure it with sqlite3_allocator
extern crate alloc;

use alloc::ffi::{CString, NulError};
use core::ffi::{c_int, c_void};
use sqlite3_capi::bindings;

pub use sqlite3_capi;
pub use sqlite3_capi::bindings::{
    sqlite3, sqlite3_api_routines, sqlite3_context, sqlite3_value, SQLITE_OK, SQLITE_UTF8,
};

pub struct Error(ErrorKind);

pub enum ErrorKind {
    ERROR(u32),
}

impl Error {
    pub fn code(&self) -> u32 {
        match self.0 {
            ErrorKind::ERROR(code) => code,
        }
    }
}

pub fn init(api: *mut bindings::sqlite3_api_routines) {
    sqlite3_capi::EXTENSION_INIT2(api);
}

pub fn value_blob<'a>(value: *mut bindings::sqlite3_value) -> &'a [u8] {
    let n = sqlite3_capi::value_bytes(value);
    let b = unsafe { sqlite3_capi::value_blob(value) };
    return unsafe { core::slice::from_raw_parts(b.cast::<u8>(), n as usize) };
}

pub fn value_text<'a>(value: *mut bindings::sqlite3_value) -> &'a str {
    let len = sqlite3_capi::value_bytes(value);
    let bytes = sqlite3_capi::value_text(value);
    let slice = unsafe { core::slice::from_raw_parts(bytes as *const u8, len as usize) };
    unsafe { core::str::from_utf8_unchecked(slice) }
}

pub fn value_int(value: *mut bindings::sqlite3_value) -> i32 {
    unsafe { sqlite3_capi::value_int(value) }
}

pub fn value_int64(value: *mut bindings::sqlite3_value) -> i64 {
    unsafe { sqlite3_capi::value_int64(value) }
}

pub fn value_double(value: *mut bindings::sqlite3_value) -> f64 {
    unsafe { sqlite3_capi::value_double(value) }
}

pub fn result_int(context: *mut bindings::sqlite3_context, i: i32) {
    unsafe { sqlite3_capi::result_int(context, i) };
}

pub fn result_int64(context: *mut bindings::sqlite3_context, i: i64) {
    unsafe { sqlite3_capi::result_int64(context, i) };
}

pub fn result_double(context: *mut bindings::sqlite3_context, i: f64) {
    unsafe { sqlite3_capi::result_double(context, i) };
}

pub fn result_blob(context: *mut bindings::sqlite3_context, blob: &[u8]) {
    let len = blob.len() as c_int;
    unsafe { sqlite3_capi::result_blob(context, blob.as_ptr().cast::<c_void>(), len) };
}

pub fn result_null(context: *mut bindings::sqlite3_context) {
    unsafe { sqlite3_capi::result_null(context) };
}

pub fn result_error(context: *mut bindings::sqlite3_context, text: &str) -> Result<(), NulError> {
    CString::new(text.as_bytes()).map(|s| {
        let n = text.len() as i32;
        let ptr = s.as_ptr();
        unsafe {
            sqlite3_capi::result_error(context, ptr, n);
        };
    })
}

pub fn result_error_code(context: *mut bindings::sqlite3_context, code: i32) {
    unsafe { sqlite3_capi::result_error_code(context, code) };
}

pub fn result_bool(context: *mut bindings::sqlite3_context, value: bool) {
    if value {
        result_int(context, 1)
    } else {
        result_int(context, 0)
    }
}

// https://github.com/asg017/sqlite-loadable-rs/blob/main/src/scalar.rs#L94
// type Function =
//     unsafe extern "C" fn(*mut bindings::sqlite3_context, c_int, *mut *mut bindings::sqlite3_value);

// // compiler generated strings -- https://stackoverflow.com/questions/53611161/how-do-i-expose-a-compile-time-generated-static-c-string-through-ffi
// pub fn create_function_v2(
//     db: *mut bindings::sqlite3,
//     name: *const c_char,
//     argc: i32,
//     flags: i32,
//     p_app: *mut c_void,
//     x_func: F,
//     x_step: Option<S>,
//     x_final: Option<Final>,
//     destroy: Option<Destroy>,
// ) {
//     // Cstring.as_ptr?
//     // sqlite3_capi::create_function_v2(
//     //     db, name, argc, text_rep, p_app, x_func, x_step, x_final, destroy,
//     // );
// }
