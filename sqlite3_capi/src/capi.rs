extern crate alloc;

use alloc::ffi::{CString, NulError};
use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::ptr;

#[cfg(feature = "omit_load_extension")]
use crate::bindings;

pub use crate::bindings::{
    sqlite3, sqlite3_api_routines as api_routines, sqlite3_context as context,
    sqlite3_index_info as index_info, sqlite3_module as module, sqlite3_stmt as stmt,
    sqlite3_uint64 as uint64, sqlite3_value as value, sqlite_int64 as int64, SQLITE_UTF8 as UTF8,
};

mod aliased {
    pub use crate::bindings::{
        sqlite3_close as close, sqlite3_commit_hook as commit_hook,
        sqlite3_create_function_v2 as create_function_v2,
        sqlite3_create_module_v2 as create_module_v2, sqlite3_declare_vtab as declare_vtab,
        sqlite3_free as free, sqlite3_get_auxdata as get_auxdata, sqlite3_malloc as malloc,
        sqlite3_malloc64 as malloc64, sqlite3_result_blob as result_blob,
        sqlite3_result_error as result_error, sqlite3_result_error_code as result_error_code,
        sqlite3_result_int as result_int, sqlite3_result_int64 as result_int64,
        sqlite3_result_null as result_null, sqlite3_result_pointer as result_pointer,
        sqlite3_result_subtype as result_subtype, sqlite3_result_text as result_text,
        sqlite3_set_auxdata as set_auxdata, sqlite3_value_text as value_text,
        sqlite3_value_type as value_type, sqlite3_vtab_distinct as vtab_distinct,
    };
}

pub enum Destructor {
    TRANSIENT,
    STATIC,
    CUSTOM(xDestroy),
}

#[macro_export]
macro_rules! strlit {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[cfg(feature = "omit_load_extension")]
macro_rules! invoke_sqlite {
    ($name:ident, $($arg:expr),*) => {
      aliased::$name($($arg),*)
    };
}

#[cfg(not(feature = "omit_load_extension"))]
macro_rules! invoke_sqlite {
  ($name:ident, $($arg:expr),*) => {
    ((*SQLITE3_API).$name.unwrap())($($arg),*)
  }
}

pub extern "C" fn droprust(ptr: *mut c_void) {
    unsafe {
        ptr.drop_in_place();
    }
}

#[macro_export]
macro_rules! args {
    ($argc:expr, $argv:expr) => {
        unsafe { slice::from_raw_parts($argv, $argc as usize) }
    };
}

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
            invoke_sqlite!(malloc64, size as uint64) as *mut u8
        } else {
            invoke_sqlite!(malloc, size as i32) as *mut u8
        }
    }
}

pub fn free(ptr: *mut u8) {
    unsafe { invoke_sqlite!(free, ptr as *mut c_void) }
}

pub type xCommitHook = unsafe extern "C" fn(*mut c_void) -> i32;
pub fn commit_hook(
    db: *mut sqlite3,
    callback: Option<xCommitHook>,
    user_data: *mut c_void,
) -> Option<xCommitHook> {
    unsafe {
        invoke_sqlite!(commit_hook, db, callback, user_data)
            .as_ref()
            .map(|p| core::mem::transmute(p))
    }
}

// pub fn mprintf(format: *const i8, ...) -> *mut c_char {
//     unsafe { ((*SQLITE3_API).mprintf.expect(EXPECT_MESSAGE))(format, args) }
// }

pub fn value_text<'a>(arg1: *mut value) -> &'a str {
    unsafe {
        let len = value_bytes(arg1);
        let bytes = invoke_sqlite!(value_text, arg1);
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

pub fn bind_pointer(db: *mut stmt, i: i32, p: *mut c_void, t: *const c_char) -> i32 {
    unsafe { ((*SQLITE3_API).bind_pointer.expect(EXPECT_MESSAGE))(db, i, p, t, None) }
}
pub fn step(stmt: *mut stmt) -> c_int {
    unsafe { ((*SQLITE3_API).step.expect(EXPECT_MESSAGE))(stmt) }
}

pub fn finalize(stmt: *mut stmt) -> c_int {
    unsafe { ((*SQLITE3_API).finalize.expect(EXPECT_MESSAGE))(stmt) }
}

pub fn column_type(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { ((*SQLITE3_API).column_type.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_count(stmt: *mut stmt) -> i32 {
    unsafe { ((*SQLITE3_API).column_count.expect(EXPECT_MESSAGE))(stmt) }
}

pub fn column_text(stmt: *mut stmt, c: c_int) -> *const c_uchar {
    unsafe { ((*SQLITE3_API).column_text.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_blob(stmt: *mut stmt, c: c_int) -> *const c_void {
    unsafe { ((*SQLITE3_API).column_blob.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_bytes(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { ((*SQLITE3_API).column_bytes.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_value(stmt: *mut stmt, c: c_int) -> *mut value {
    unsafe { ((*SQLITE3_API).column_value.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_double(stmt: *mut stmt, c: c_int) -> f64 {
    unsafe { ((*SQLITE3_API).column_double.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_int(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { ((*SQLITE3_API).column_int.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_int64(stmt: *mut stmt, c: c_int) -> int64 {
    unsafe { ((*SQLITE3_API).column_int64.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn column_name(stmt: *mut stmt, c: c_int) -> *const c_char {
    unsafe { ((*SQLITE3_API).column_name.expect(EXPECT_MESSAGE))(stmt, c) }
}

pub fn bind_text(stmt: *mut stmt, c: c_int, s: *const c_char, n: c_int) -> i32 {
    unsafe { ((*SQLITE3_API).bind_text.expect(EXPECT_MESSAGE))(stmt, c, s, n, None) }
}

pub fn prepare_v2(
    db: *mut sqlite3,
    sql: *const c_char,
    n: i32,
    stmt: *mut *mut stmt,
    leftover: *mut *const c_char,
) -> i32 {
    unsafe { ((*SQLITE3_API).prepare_v2.expect(EXPECT_MESSAGE))(db, sql, n, stmt, leftover) }
}

pub fn value_int(arg1: *mut value) -> i32 {
    unsafe { ((*SQLITE3_API).value_int.expect(EXPECT_MESSAGE))(arg1) }
}

pub fn value_int64(arg1: *mut value) -> int64 {
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

pub fn result_blob(context: *mut context, b: *const u8, n: i32, d: Destructor) {
    unsafe {
        ((*SQLITE3_API).result_blob.expect(EXPECT_MESSAGE))(
            context,
            b as *const c_void,
            n,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            },
        );
    }
}

pub fn result_int64(context: *mut context, v: int64) {
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
    unsafe { invoke_sqlite!(result_null, context) }
}
pub fn result_pointer(
    context: *mut context,
    pointer: *mut c_void,
    name: *mut c_char,
    destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe { invoke_sqlite!(result_pointer, context, pointer, name, destructor) }
}

pub fn result_error(context: *mut context, text: &str) -> Result<(), NulError> {
    CString::new(text.as_bytes()).map(|s| {
        let n = text.len() as i32;
        let ptr = s.as_ptr();
        unsafe { invoke_sqlite!(result_error, context, ptr, n) }
    })
}

pub fn result_error_code(context: *mut context, code: i32) {
    unsafe { invoke_sqlite!(result_error_code, context, code) }
}

// d is our destructor function.
// -- https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb
pub fn result_text(context: *mut context, s: *const i8, n: i32, d: Destructor) {
    unsafe {
        invoke_sqlite!(
            result_text,
            context,
            s,
            n,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            }
        )
    }
}

pub fn result_subtype(context: *mut context, subtype: u32) {
    unsafe { invoke_sqlite!(result_subtype, context, subtype) }
}

pub fn set_auxdata(
    context: *mut context,
    n: c_int,
    p: *mut c_void,
    d: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe { invoke_sqlite!(set_auxdata, context, n, p, d) }
}

pub fn get_auxdata(context: *mut context, n: c_int) -> *mut c_void {
    unsafe { invoke_sqlite!(get_auxdata, context, n) }
}

pub type xFunc = unsafe extern "C" fn(*mut context, i32, *mut *mut value);
pub type xStep = unsafe extern "C" fn(*mut context, i32, *mut *mut value);
pub type xFinal = unsafe extern "C" fn(*mut context);
pub type xDestroy = unsafe extern "C" fn(*mut c_void);
pub fn create_function_v2(
    db: *mut sqlite3,
    s: *const c_char,
    argc: i32,
    flags: u32,
    p_app: *mut c_void,
    x_func: Option<xFunc>,
    x_step: Option<xStep>,
    x_final: Option<xFinal>,
    destroy: Option<xDestroy>,
) -> c_int {
    unsafe {
        invoke_sqlite!(
            create_function_v2,
            db,
            s,
            argc,
            i32::try_from(flags).expect("Invalid flags"),
            p_app,
            x_func,
            x_step,
            x_final,
            destroy
        )
    }
}

pub fn create_module_v2(
    db: *mut sqlite3,
    s: *const i8,
    module: *const module,
    p_app: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> i32 {
    unsafe { invoke_sqlite!(create_module_v2, db, s, module, p_app, destroy) }
}

pub fn vtab_distinct(index_info: *mut index_info) -> i32 {
    unsafe { invoke_sqlite!(vtab_distinct, index_info) }
}

pub fn declare_vtab(db: *mut sqlite3, s: *const i8) -> i32 {
    unsafe { invoke_sqlite!(declare_vtab, db, s) }
}

pub fn close(db: *mut sqlite3) -> i32 {
    unsafe { invoke_sqlite!(close, db) }
}
