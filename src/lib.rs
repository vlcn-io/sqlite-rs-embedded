#![no_std]
#![feature(vec_into_raw_parts)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
pub use sqlite3_allocator::*;
pub use sqlite3_capi::*;

/// Pass and give ownership of the string to SQLite.
/// SQLite will not copy the string.
/// This method will correctly drop the string when SQLite is finished
/// using it.
pub fn result_text_owned(ctx: *mut context, text: String) {
    let (ptr, len, _) = text.into_raw_parts();
    result_text(
        ctx,
        ptr as *const c_char,
        len as i32,
        Destructor::CUSTOM(droprust),
    );
}

/// Takes a reference to a string, has SQLite copy the contents
/// and take ownership of the copy.
pub fn result_text_shared(ctx: *mut context, text: &str) {
    result_text(
        ctx,
        text.as_ptr() as *mut c_char,
        text.len() as i32,
        Destructor::TRANSIENT,
    );
}

/// Takes a reference to a string that is statically allocated.
/// SQLite will not copy this string.
pub fn result_text_static(ctx: *mut context, text: &'static str) {
    result_text(
        ctx,
        text.as_ptr() as *mut c_char,
        text.len() as i32,
        Destructor::STATIC,
    );
}

pub fn result_blob_owned(ctx: *mut context, blob: Vec<u8>) {
    let (ptr, len, _) = blob.into_raw_parts();
    result_blob(ctx, ptr, len as i32, Destructor::CUSTOM(droprust));
}

pub fn result_blob_shared(ctx: *mut context, blob: &[u8]) {
    result_blob(ctx, blob.as_ptr(), blob.len() as i32, Destructor::TRANSIENT);
}

pub fn result_blob_static(ctx: *mut context, blob: &'static [u8]) {
    result_blob(ctx, blob.as_ptr(), blob.len() as i32, Destructor::STATIC);
}
