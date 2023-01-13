#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::{
    sqlite3, sqlite3_api_routines as api_routines, sqlite3_context as context,
    sqlite3_value as value, SQLITE_INNOCUOUS as INNOCUOUS, SQLITE_OK as OK, SQLITE_UTF8 as UTF8,
};

extern crate alloc;

use alloc::ffi::{CString, NulError};
use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::ptr;

pub enum Destructor {
    TRANSIENT,
    STATIC,
    CUSTOM(unsafe extern "C" fn(*mut c_void)),
}

#[macro_export]
macro_rules! strlit {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

pub fn strdyn(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}

#[macro_export]
macro_rules! args {
    ($argc:expr, $argv:expr) => {
        unsafe { slice::from_raw_parts($argv, $argc as usize) }
    };
}

// macro emulation: https://github.com/asg017/sqlite-loadable-rs/blob/main/src/ext.rs
static mut SQLITE3_API: *mut api_routines = ptr::null_mut();

pub fn EXTENSION_INIT2(api: *mut api_routines) {
    unsafe {
        SQLITE3_API = api;
    }
}

static EXPECT_MESSAGE: &str =
    "sqlite-loadable error: expected method on SQLITE3_API. Please file an issue";

pub fn malloc(size: usize) -> *mut u8 {
    unsafe {
        if usize::BITS == 64 {
            let ptr =
                ((*SQLITE3_API).malloc64.expect(EXPECT_MESSAGE))(size as bindings::sqlite3_uint64);
            ptr as *mut u8
        } else {
            let ptr = ((*SQLITE3_API).malloc.expect(EXPECT_MESSAGE))(size as i32);
            ptr as *mut u8
        }
    }
}

pub fn free(ptr: *mut u8) {
    unsafe {
        ((*SQLITE3_API).free.expect(EXPECT_MESSAGE))(ptr as *mut c_void);
    }
}

pub fn value_text<'a>(arg1: *mut value) -> &'a str {
    unsafe {
        let len = value_bytes(arg1);
        let bytes = ((*SQLITE3_API).value_text.expect(EXPECT_MESSAGE))(arg1);
        let slice = core::slice::from_raw_parts(bytes as *const u8, len as usize);
        core::str::from_utf8_unchecked(slice)
    }
}

pub fn value_type(value: *mut value) -> i32 {
    unsafe { ((*SQLITE3_API).value_type.expect(EXPECT_MESSAGE))(value) }
}

pub fn value_bytes(arg1: *mut value) -> i32 {
    unsafe { ((*SQLITE3_API).value_bytes.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_blob<'a>(value: *mut value) -> &'a [u8] {
    unsafe {
        let n = value_bytes(value);
        let b = ((*SQLITE3_API).value_blob.expect(EXPECT_MESSAGE))(value);
        core::slice::from_raw_parts(b.cast::<u8>(), n as usize)
    }
}

pub fn bind_pointer(
    db: *mut bindings::sqlite3_stmt,
    i: i32,
    p: *mut c_void,
    t: *const c_char,
) -> i32 {
    unsafe { ((*SQLITE3_API).bind_pointer.expect(EXPECT_MESSAGE))(db, i, p, t, None) }
}
pub fn step(stmt: *mut bindings::sqlite3_stmt) -> c_int {
    unsafe { ((*SQLITE3_API).step.expect(EXPECT_MESSAGE))(stmt) }
}

pub fn finalize(stmt: *mut bindings::sqlite3_stmt) -> c_int {
    unsafe { ((*SQLITE3_API).finalize.expect(EXPECT_MESSAGE))(stmt) }
}

pub fn column_text(stmt: *mut bindings::sqlite3_stmt, c: c_int) -> *const c_uchar {
    unsafe { ((*SQLITE3_API).column_text.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_value(stmt: *mut bindings::sqlite3_stmt, c: c_int) -> *mut value {
    unsafe { ((*SQLITE3_API).column_value.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn bind_text(stmt: *mut bindings::sqlite3_stmt, c: c_int, s: *const c_char, n: c_int) -> i32 {
    unsafe { ((*SQLITE3_API).bind_text.expect(EXPECT_MESSAGE))(stmt, c, s, n, None) }
}

pub fn prepare_v2(
    db: *mut bindings::sqlite3,
    sql: *const c_char,
    n: i32,
    stmt: *mut *mut bindings::sqlite3_stmt,
    leftover: *mut *const c_char,
) -> i32 {
    unsafe { ((*SQLITE3_API).prepare_v2.expect(EXPECT_MESSAGE))(db, sql, n, stmt, leftover) }
}

pub fn value_int(arg1: *mut value) -> i32 {
    unsafe { ((*SQLITE3_API).value_int.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_int64(arg1: *mut value) -> i64 {
    unsafe { ((*SQLITE3_API).value_int64.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_double(arg1: *mut value) -> f64 {
    unsafe { ((*SQLITE3_API).value_double.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_pointer(arg1: *mut value, p: *mut c_char) -> *mut c_void {
    unsafe { ((*SQLITE3_API).value_pointer.expect(EXPECT_MESSAGE))(arg1, p) }
}

pub fn result_int(context: *mut context, v: c_int) {
    unsafe {
        ((*SQLITE3_API).result_int.expect(EXPECT_MESSAGE))(context, v);
    }
}

// TODO: expose a version that doesn't always require copying the blob.
// I.e., a method that can take a destructor function for SQLite to call.
pub fn result_blob(context: *mut context, blob: &[u8], d: Destructor) {
    let len = blob.len() as c_int;
    unsafe {
        ((*SQLITE3_API).result_blob.expect(EXPECT_MESSAGE))(
            context,
            blob.as_ptr().cast::<c_void>(),
            len,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            },
        );
    }
}
pub fn result_int64(context: *mut context, v: i64) {
    unsafe {
        ((*SQLITE3_API).result_int64.expect(EXPECT_MESSAGE))(context, v);
    }
}

pub fn result_double(context: *mut context, f: f64) {
    unsafe {
        ((*SQLITE3_API).result_double.expect(EXPECT_MESSAGE))(context, f);
    }
}

pub fn result_null(context: *mut context) {
    unsafe {
        ((*SQLITE3_API).result_null.expect(EXPECT_MESSAGE))(context);
    }
}
pub fn result_pointer(
    context: *mut context,
    pointer: *mut c_void,
    name: *mut c_char,
    destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe {
        ((*SQLITE3_API).result_pointer.expect(EXPECT_MESSAGE))(context, pointer, name, destructor);
    }
}

pub fn result_error(context: *mut context, text: &str) -> Result<(), NulError> {
    CString::new(text.as_bytes()).map(|s| {
        let n = text.len() as i32;
        let ptr = s.as_ptr();
        unsafe {
            ((*SQLITE3_API).result_error.expect(EXPECT_MESSAGE))(context, ptr, n);
        }
    })
}

pub fn result_error_code(context: *mut context, code: i32) {
    unsafe {
        ((*SQLITE3_API).result_error_code.expect(EXPECT_MESSAGE))(context, code);
    }
}

// d is our destructor function.
// -- https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb
pub fn result_text(context: *mut context, s: *const i8, n: i32, d: Destructor) {
    unsafe {
        ((*SQLITE3_API).result_text.expect(EXPECT_MESSAGE))(
            context,
            s,
            n,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            },
        );
    }
}

pub fn result_subtype(context: *mut context, subtype: u32) {
    unsafe {
        ((*SQLITE3_API).result_subtype.expect(EXPECT_MESSAGE))(context, subtype);
    }
}

pub fn set_auxdata(
    context: *mut context,
    n: c_int,
    p: *mut c_void,
    d: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe {
        ((*SQLITE3_API).set_auxdata.expect(EXPECT_MESSAGE))(context, n, p, d);
    }
}

pub fn get_auxdata(context: *mut context, n: c_int) -> *mut c_void {
    unsafe { ((*SQLITE3_API).get_auxdata.expect(EXPECT_MESSAGE))(context, n) }
}

pub fn create_function_v2(
    db: *mut bindings::sqlite3,
    s: *const c_char,
    argc: i32,
    flags: u32,
    p_app: *mut c_void,
    x_func: Option<unsafe extern "C" fn(*mut context, i32, *mut *mut value)>,
    x_step: Option<unsafe extern "C" fn(*mut context, i32, *mut *mut value)>,
    x_final: Option<unsafe extern "C" fn(*mut context)>,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> c_int {
    unsafe {
        // SQLITE3_API is null when -DOMIT_LOAD_EXTENSION flag is set
        // in that case we're statically linked directly and can reference
        // the function directly
        // match (*SQLITE3_API).create_function_v2 {
        //     None => bindings::sqlite3_create_function_v2(
        //         db,
        //         s,
        //         argc,
        //         i32::try_from(flags).expect("Invalid flags"),
        //         p_app,
        //         x_func,
        //         x_step,
        //         x_final,
        //         destroy,
        //     ),
        //     Some(f) => f(
        //         db,
        //         s,
        //         argc,
        //         i32::try_from(flags).expect("Invalid flags"),
        //         p_app,
        //         x_func,
        //         x_step,
        //         x_final,
        //         destroy,
        //     ),
        // }
        ((*SQLITE3_API).create_function_v2.expect(EXPECT_MESSAGE))(
            db,
            s,
            argc,
            i32::try_from(flags).expect("Invalid flags"),
            p_app,
            x_func,
            x_step,
            x_final,
            destroy,
        )
    }
}

pub fn create_module_v2(
    db: *mut bindings::sqlite3,
    s: *const i8,
    module: *const bindings::sqlite3_module,
    p_app: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> i32 {
    unsafe {
        ((*SQLITE3_API).create_module_v2.expect(EXPECT_MESSAGE))(db, s, module, p_app, destroy)
    }
}

pub fn vtab_distinct(index_info: *mut bindings::sqlite3_index_info) -> i32 {
    unsafe { ((*SQLITE3_API).vtab_distinct.expect(EXPECT_MESSAGE))(index_info) }
}

pub fn sqlitex_declare_vtab(db: *mut bindings::sqlite3, s: *const i8) -> i32 {
    unsafe { ((*SQLITE3_API).declare_vtab.expect(EXPECT_MESSAGE))(db, s) }
}

// we don't need this... right? It's just overcomplicating what only need to be a call to `SQLITE_EXTENSION_INIT2`
// pub fn start_extension<F>(
//     db: *mut bindings::sqlite3,
//     _pz_err_msg: *mut *mut c_char,
//     p_api: *mut bindings::api_routines,
//     callback: F,
// ) -> c_uint
// where
//     F: Fn(*mut bindings::sqlite3) -> Result<(), Error>,
// {
//     unsafe {
//         faux_sqlite_extension_init2(p_api);
//     }
//     match callback(db) {
//         Ok(()) => SQLITE_OK,
//         Err(err) => err.code(),
//     }
// }
