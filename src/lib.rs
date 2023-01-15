#![no_std]

extern crate alloc;

use alloc::ffi::CString;
use core::ffi::c_char;
pub use sqlite3_allocator::*;
pub use sqlite3_capi::*;

/*
 * Wrappers for the various versions of lifetimes
 */

/// Pass and give ownership of the string.
/// This method will correctly drop the string
/// when SQLite is finished using it.
pub fn result_text_owned(ctx: *mut context, text: CString, len: i32) {
    result_text(ctx, text.into_raw(), len, Destructor::CUSTOM(dropstr));
}

pub fn result_text_shared(ctx: *mut context, text: &str) {
    result_text(
        ctx,
        text.as_ptr() as *const c_char,
        text.len() as i32,
        Destructor::TRANSIENT,
    );
}

pub fn result_text_static(ctx: *mut context, text: &'static str) {
    result_text(
        ctx,
        text.as_ptr() as *const c_char,
        text.len() as i32,
        Destructor::STATIC,
    );
}
