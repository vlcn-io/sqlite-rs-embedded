#![no_std]
#![feature(vec_into_raw_parts)]

extern crate alloc;

use alloc::string::String;
use core::ffi::c_char;
pub use sqlite3_allocator::*;
pub use sqlite3_capi::*;

/// Pass and give ownership of the string.
/// This method will correctly drop the string when SQLite is finished
/// using it.
pub fn result_text_owned(ctx: *mut context, text: String) {
    let (ptr, len, _) = text.into_raw_parts();
    result_text(
        ctx,
        ptr as *mut c_char,
        len as i32,
        Destructor::CUSTOM(droprust),
    );
}

pub fn result_text_shared(ctx: *mut context, text: &str) {
    result_text(
        ctx,
        text.as_ptr() as *mut c_char,
        text.len() as i32,
        Destructor::TRANSIENT,
    );
}

pub fn result_text_static(ctx: *mut context, text: &'static str) {
    result_text(
        ctx,
        text.as_ptr() as *mut c_char,
        text.len() as i32,
        Destructor::STATIC,
    );
}
