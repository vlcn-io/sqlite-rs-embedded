#![no_std]
#![feature(vec_into_raw_parts)]
#![allow(non_camel_case_types)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
pub use sqlite3_allocator::*;
pub use sqlite3_capi::*;

// Make a struct Stmt {}
// This stmt will:
// - have a lifetime tied to the db
// - have all extracted column value types lifetimes tied to the stmt
// - have a drop impl that finalizes the stmt
// https://stackoverflow.com/questions/27219258/in-rust-how-do-you-explicitly-tie-the-lifetimes-of-two-objects-together-withou

// pub struct Stmt {
//     /// Internal pointer to the C stmt
//     stmt: *mut stmt,
// }

// pub struct Context {
//     ctx: *mut context,
// }

// impl Context {
//     pub fn new(ctx: *mut context) -> Self {
//         Self { ctx }
//     }
// }

pub trait Context {
    /// Pass and give ownership of the string to SQLite.
    /// SQLite will not copy the string.
    /// This method will correctly drop the string when SQLite is finished
    /// using it.
    fn result_text_owned(&self, text: String);
    fn result_text_shared(&self, text: &str);
    fn result_text_static(&self, text: &'static str);
    fn result_blob_owned(&self, blob: Vec<u8>);
    fn result_blob_shared(&self, blob: &[u8]);
    fn result_blob_static(&self, blob: &'static [u8]);
}

impl Context for *mut context {
    fn result_text_owned(&self, text: String) {
        let (ptr, len, _) = text.into_raw_parts();
        result_text(
            *self,
            ptr as *const c_char,
            len as i32,
            Destructor::CUSTOM(droprust),
        );
    }

    /// Takes a reference to a string, has SQLite copy the contents
    /// and take ownership of the copy.
    fn result_text_shared(&self, text: &str) {
        result_text(
            *self,
            text.as_ptr() as *mut c_char,
            text.len() as i32,
            Destructor::TRANSIENT,
        );
    }

    /// Takes a reference to a string that is statically allocated.
    /// SQLite will not copy this string.
    fn result_text_static(&self, text: &'static str) {
        result_text(
            *self,
            text.as_ptr() as *mut c_char,
            text.len() as i32,
            Destructor::STATIC,
        );
    }

    fn result_blob_owned(&self, blob: Vec<u8>) {
        let (ptr, len, _) = blob.into_raw_parts();
        result_blob(*self, ptr, len as i32, Destructor::CUSTOM(droprust));
    }

    fn result_blob_shared(&self, blob: &[u8]) {
        result_blob(
            *self,
            blob.as_ptr(),
            blob.len() as i32,
            Destructor::TRANSIENT,
        );
    }

    fn result_blob_static(&self, blob: &'static [u8]) {
        result_blob(*self, blob.as_ptr(), blob.len() as i32, Destructor::STATIC);
    }
}
