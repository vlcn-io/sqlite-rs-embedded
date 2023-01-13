#![no_std]
#![feature(alloc_error_handler)]

use core::ffi::c_char;
use sqlite_nostd;
use sqlite_nostd::{
    sqlite3, sqlite3_api_routines, sqlite3_context, sqlite3_value, SQLite3Allocator, SQLITE_OK,
    SQLITE_UTF8,
};

#[global_allocator]
static ALLOCATOR: SQLite3Allocator = SQLite3Allocator {};

#[no_mangle]
pub extern "C" fn testext_fn(
    ctx: *mut sqlite3_context,
    _argc: i32,
    _argv: *mut *mut sqlite3_value,
) {
    sqlite_nostd::result_int(ctx, 100);
}

#[no_mangle]
pub extern "C" fn sqlite_nostdextension_init(
    db: *mut sqlite3,
    _err_msg: *mut *mut c_char,
    api: *mut sqlite3_api_routines,
) -> u32 {
    sqlite_nostd::EXTENSION_INIT2(api);

    // register a function extension
    // use some collections inside the function
    // return a string to test allocation
    sqlite_nostd::create_function_v2(
        db,
        "testit\0".as_ptr() as *const c_char,
        0,
        SQLITE_UTF8,
        core::ptr::null_mut(),
        Some(testext_fn),
        None,
        None,
        None,
    );

    SQLITE_OK
}

// TODO: these shouldn't be provided per extension.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::alloc::Layout;
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
