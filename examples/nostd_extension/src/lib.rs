#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

use core::ffi::c_char;
use sqlite_nostd as sqlite;
use sqlite_nostd::SQLite3Allocator;

#[global_allocator]
static ALLOCATOR: SQLite3Allocator = SQLite3Allocator {};

#[no_mangle]
pub extern "C" fn testext_fn(
    ctx: *mut sqlite::context,
    _argc: i32,
    _argv: *mut *mut sqlite::value,
) {
    sqlite::result_text(
        ctx,
        "Hello, world!\0".as_ptr() as *const c_char,
        -1,
        sqlite::Destructor::TRANSIENT,
    );
}

#[no_mangle]
pub extern "C" fn sqlite3_nostdextension_init(
    db: *mut sqlite::sqlite3,
    _err_msg: *mut *mut c_char,
    api: *mut sqlite::api_routines,
) -> u32 {
    sqlite::EXTENSION_INIT2(api);

    // register a function extension
    // use some collections inside the function
    // return a string to test allocation
    sqlite::create_function_v2(
        db,
        "testit\0".as_ptr() as *const c_char,
        0,
        sqlite::UTF8,
        core::ptr::null_mut(),
        Some(testext_fn),
        None,
        None,
        None,
    );

    sqlite::OK
}

// TODO: these shouldn't be provided per extension.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::intrinsics::abort()
}

use core::alloc::Layout;
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    core::intrinsics::abort()
}
