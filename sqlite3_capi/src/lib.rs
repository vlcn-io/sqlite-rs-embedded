#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::ptr;

// macro emulation: https://github.com/asg017/sqlite-loadable-rs/blob/main/src/ext.rs
static mut SQLITE3_API: *mut bindings::sqlite3_api_routines = ptr::null_mut();

pub fn EXTENSION_INIT2(api: *mut bindings::sqlite3_api_routines) {
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

pub fn value_text(arg1: *mut bindings::sqlite3_value) -> *const c_uchar {
    unsafe { ((*SQLITE3_API).value_text.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_type(value: *mut bindings::sqlite3_value) -> i32 {
    unsafe { ((*SQLITE3_API).value_type.expect(EXPECT_MESSAGE))(value) }
}

pub fn value_bytes(arg1: *mut bindings::sqlite3_value) -> i32 {
    unsafe { ((*SQLITE3_API).value_bytes.expect(EXPECT_MESSAGE))(arg1) }
}

pub unsafe fn value_blob(arg1: *mut bindings::sqlite3_value) -> *const c_void {
    ((*SQLITE3_API).value_blob.expect(EXPECT_MESSAGE))(arg1)
}

pub unsafe fn bind_pointer(
    db: *mut bindings::sqlite3_stmt,
    i: i32,
    p: *mut c_void,
    t: *const c_char,
) -> i32 {
    ((*SQLITE3_API).bind_pointer.expect(EXPECT_MESSAGE))(db, i, p, t, None)
}
pub unsafe fn step(stmt: *mut bindings::sqlite3_stmt) -> c_int {
    ((*SQLITE3_API).step.expect(EXPECT_MESSAGE))(stmt)
}

pub unsafe fn finalize(stmt: *mut bindings::sqlite3_stmt) -> c_int {
    ((*SQLITE3_API).finalize.expect(EXPECT_MESSAGE))(stmt)
}

pub unsafe fn column_text(stmt: *mut bindings::sqlite3_stmt, c: c_int) -> *const c_uchar {
    ((*SQLITE3_API).column_text.expect(EXPECT_MESSAGE))(stmt, c)
}

pub unsafe fn column_value(
    stmt: *mut bindings::sqlite3_stmt,
    c: c_int,
) -> *mut bindings::sqlite3_value {
    ((*SQLITE3_API).column_value.expect(EXPECT_MESSAGE))(stmt, c)
}

pub unsafe fn bind_text(
    stmt: *mut bindings::sqlite3_stmt,
    c: c_int,
    s: *const c_char,
    n: c_int,
) -> i32 {
    ((*SQLITE3_API).bind_text.expect(EXPECT_MESSAGE))(stmt, c, s, n, None)
}

pub unsafe fn prepare_v2(
    db: *mut bindings::sqlite3,
    sql: *const c_char,
    n: i32,
    stmt: *mut *mut bindings::sqlite3_stmt,
    leftover: *mut *const c_char,
) -> i32 {
    ((*SQLITE3_API).prepare_v2.expect(EXPECT_MESSAGE))(db, sql, n, stmt, leftover)
}

pub unsafe fn value_int(arg1: *mut bindings::sqlite3_value) -> i32 {
    ((*SQLITE3_API).value_int.expect(EXPECT_MESSAGE))(arg1)
}

pub unsafe fn value_int64(arg1: *mut bindings::sqlite3_value) -> i64 {
    ((*SQLITE3_API).value_int64.expect(EXPECT_MESSAGE))(arg1)
}

pub unsafe fn value_double(arg1: *mut bindings::sqlite3_value) -> f64 {
    ((*SQLITE3_API).value_double.expect(EXPECT_MESSAGE))(arg1)
}

pub unsafe fn value_pointer(arg1: *mut bindings::sqlite3_value, p: *mut c_char) -> *mut c_void {
    ((*SQLITE3_API).value_pointer.expect(EXPECT_MESSAGE))(arg1, p)
}

pub unsafe fn result_int(context: *mut bindings::sqlite3_context, v: c_int) {
    ((*SQLITE3_API).result_int.expect(EXPECT_MESSAGE))(context, v);
}

// TODO: expose a version that doesn't always require copying the blob.
// I.e., a method that can take a destructor function for SQLite to call.
pub unsafe fn result_blob(context: *mut bindings::sqlite3_context, p: *const c_void, n: i32) {
    ((*SQLITE3_API).result_blob.expect(EXPECT_MESSAGE))(
        context,
        p,
        n,
        Some(core::mem::transmute(-1_isize)),
    );
}
pub unsafe fn result_int64(context: *mut bindings::sqlite3_context, v: i64) {
    ((*SQLITE3_API).result_int64.expect(EXPECT_MESSAGE))(context, v);
}

pub unsafe fn result_double(context: *mut bindings::sqlite3_context, f: f64) {
    ((*SQLITE3_API).result_double.expect(EXPECT_MESSAGE))(context, f);
}

pub unsafe fn result_null(context: *mut bindings::sqlite3_context) {
    ((*SQLITE3_API).result_null.expect(EXPECT_MESSAGE))(context);
}
pub unsafe fn result_pointer(
    context: *mut bindings::sqlite3_context,
    pointer: *mut c_void,
    name: *mut c_char,
    destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    ((*SQLITE3_API).result_pointer.expect(EXPECT_MESSAGE))(context, pointer, name, destructor);
}

pub unsafe fn result_error(context: *mut bindings::sqlite3_context, s: *const i8, n: i32) {
    ((*SQLITE3_API).result_error.expect(EXPECT_MESSAGE))(context, s, n);
}

pub unsafe fn result_error_code(context: *mut bindings::sqlite3_context, code: i32) {
    ((*SQLITE3_API).result_error_code.expect(EXPECT_MESSAGE))(context, code);
}

// d is our destructor function.
// -- https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb
pub unsafe fn result_text(
    context: *mut bindings::sqlite3_context,
    s: *const i8,
    n: i32,
    d: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    ((*SQLITE3_API).result_text.expect(EXPECT_MESSAGE))(context, s, n, d);
}

pub unsafe fn result_subtype(context: *mut bindings::sqlite3_context, subtype: u32) {
    ((*SQLITE3_API).result_subtype.expect(EXPECT_MESSAGE))(context, subtype);
}

pub unsafe fn set_auxdata(
    context: *mut bindings::sqlite3_context,
    n: c_int,
    p: *mut c_void,
    d: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    ((*SQLITE3_API).set_auxdata.expect(EXPECT_MESSAGE))(context, n, p, d);
}

pub unsafe fn get_auxdata(context: *mut bindings::sqlite3_context, n: c_int) -> *mut c_void {
    ((*SQLITE3_API).get_auxdata.expect(EXPECT_MESSAGE))(context, n)
}

pub unsafe fn create_function_v2(
    db: *mut bindings::sqlite3,
    s: *const c_char,
    argc: i32,
    flags: u32,
    p_app: *mut c_void,
    x_func: Option<
        unsafe extern "C" fn(
            *mut bindings::sqlite3_context,
            i32,
            *mut *mut bindings::sqlite3_value,
        ),
    >,
    x_step: Option<
        unsafe extern "C" fn(
            *mut bindings::sqlite3_context,
            i32,
            *mut *mut bindings::sqlite3_value,
        ),
    >,
    x_final: Option<unsafe extern "C" fn(*mut bindings::sqlite3_context)>,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> c_int {
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

pub unsafe fn create_module_v2(
    db: *mut bindings::sqlite3,
    s: *const i8,
    module: *const bindings::sqlite3_module,
    p_app: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> i32 {
    ((*SQLITE3_API).create_module_v2.expect(EXPECT_MESSAGE))(db, s, module, p_app, destroy)
}

pub unsafe fn vtab_distinct(index_info: *mut bindings::sqlite3_index_info) -> i32 {
    ((*SQLITE3_API).vtab_distinct.expect(EXPECT_MESSAGE))(index_info)
}

pub unsafe fn sqlitex_declare_vtab(db: *mut bindings::sqlite3, s: *const i8) -> i32 {
    ((*SQLITE3_API).declare_vtab.expect(EXPECT_MESSAGE))(db, s)
}

// we don't need this... right? It's just overcomplicating what only need to be a call to `SQLITE_EXTENSION_INIT2`
// pub fn start_extension<F>(
//     db: *mut bindings::sqlite3,
//     _pz_err_msg: *mut *mut c_char,
//     p_api: *mut bindings::sqlite3_api_routines,
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
