#![no_std]
#![feature(vec_into_raw_parts)]
#![allow(non_camel_case_types)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
#[macro_use]
extern crate num_derive;

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

pub trait DB<'a> {
    fn prepare_v2(&self, sql: &str) -> Result<Box<dyn Stmt<'a>>, Error>;
}

#[derive(FromPrimitive)]
enum ResultCode {
    OK = 0,
    ERROR = 1,
    INTERNAL = 2,
    PERM = 3,
    ABORT = 4,
    BUSY = 5,
    LOCKED = 6,
    NOMEM = 7,
    READONLY = 8,
    INTERRUPT = 9,
    IOERR = 10,
    CORRUPT = 11,
    NOTFOUND = 12,
    FULL = 13,
    CANTOPEN = 14,
    PROTOCOL = 15,
    EMPTY = 16,
    SCHEMA = 17,
    TOOBIG = 18,
    CONSTRAINT = 19,
    MISMATCH = 20,
    MISUSE = 21,
    NOLFS = 22,
    AUTH = 23,
    FORMAT = 24,
    RANGE = 25,
    NOTADB = 26,
    NOTICE = 27,
    WARNING = 28,
    ROW = 100,
    DONE = 101,
    NULL = 5000,
}

#[derive(FromPrimitive)]
enum ColumnType {
    Integer,
    Float,
    Text,
    Blob,
    Null,
}

pub trait Stmt<'a> {
    fn step(&self) -> ResultCode;
    fn column_count(&self) -> i32;
    fn column_name(&self, i: i32) -> Result<&'a str, ResultCode>;
    fn column_type(&self, i: i32) -> ColumnType;
    fn column_text(&self, i: i32) -> Result<&'a str, ResultCode>;
    fn column_blob(&self, i: i32) -> Result<&'a [u8], ResultCode>;
    fn column_double(&self, i: i32) -> Result<f64, ResultCode>;
    fn column_int(&self, i: i32) -> Result<i32, ResultCode>;
    fn column_int64(&self, i: i32) -> Result<i64, ResultCode>;
}

impl<'a> Stmt<'a> for *mut stmt {
    fn step(&self) -> ResultCode {
        ResultCode::from_i32(step(*self))
    }

    fn column_count(&self) -> i32 {
        column_count(*self)
    }

    fn column_name(&self, i: i32) -> Result<&'a str, ResultCode> {
        let ptr = column_name(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(
                unsafe {
                    core::str::from_utf8_unchecked(core::ffi::CStr::from_ptr(ptr).to_bytes())
                },
            )
        }
    }

    fn column_type(&self, i: i32) -> ColumnType {
        ColumnType::from_i32(column_type(*self, i))
    }

    fn column_text(&self, i: i32) -> Result<&'a str, ResultCode> {
        let ptr = column_text(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(
                unsafe {
                    core::str::from_utf8_unchecked(core::ffi::CStr::from_ptr(ptr).to_bytes())
                },
            )
        }
    }

    fn column_blob(&self, i: i32) -> Result<&'a [u8], ResultCode> {
        let len = column_bytes(*self, i);
        let ptr = column_blob(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) })
        }
    }

    fn column_double(&self, i: i32) -> Result<f64, Error> {
        Ok(column_double(*self, i))
    }

    fn column_int(&self, i: i32) -> Result<i32, Error> {
        Ok(column_int(*self, i))
    }

    fn column_int64(&self, i: i32) -> Result<i64, Error> {
        Ok(column_int64(*self, i))
    }
}

impl Drop for *mut stmt {
    fn drop(&mut self) {
        finalize(*self);
    }
}

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
