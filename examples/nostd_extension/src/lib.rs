#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

extern crate alloc;
// todo: make this example just an example in the browser crate

use alloc::string::String;
// use alloc::vec;
use core::ffi::c_char;
use sqlite::Connection;
use sqlite::Context;

#[cfg(target_family = "wasm")]
use sqlite_web as sqlite;

#[cfg(not(target_family = "wasm"))]
use sqlite_nostd as sqlite;

#[no_mangle]
pub extern "C" fn testext_fn(
    ctx: *mut sqlite::context,
    _argc: i32,
    _argv: *mut *mut sqlite::value,
) {
    // let args = sqlite::args!(argc, argv);
    // Static strings:
    // sqlite::result_text_static(ctx, "Hello, world!");

    // Transient string slices:
    // sqlite::result_text_shared(ctx, "Hello, world!");

    // Dynamic strings with custom deallocator:
    ctx.result_text_owned(String::from("ello mate!"));
}

#[no_mangle]
pub extern "C" fn sqlite3_webext_init(
    db: *mut sqlite::sqlite3,
    _err_msg: *mut *mut c_char,
    api: *mut sqlite::api_routines,
) -> u32 {
    sqlite::EXTENSION_INIT2(api);

    db.create_function_v2(
        sqlite::strlit!("testit"),
        0,
        sqlite::UTF8,
        None,
        Some(testext_fn),
        None,
        None,
        None,
    )
    .unwrap_or(sqlite::ResultCode::ERROR) as u32
}
