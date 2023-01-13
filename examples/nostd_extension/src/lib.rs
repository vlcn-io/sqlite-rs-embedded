#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

use core::ffi::c_char;
use sqlite_browser as sqlite;

#[no_mangle]
pub extern "C" fn testext_fn(
    ctx: *mut sqlite::context,
    _argc: i32,
    _argv: *mut *mut sqlite::value,
) {
    // let args = sqlite::args!(argc, argv);
    // Static strings:
    // sqlite::result_text(
    //     ctx,
    //     sqlite::strlit!("Hello, world!"),
    //     -1,
    //     sqlite::Destructor::STATIC,
    // );

    // Dynamic strings:
    // sqlite::strdyn("Hello, world!")
    //     .map(|s| sqlite::result_text(ctx, s.as_ptr(), -1, sqlite::Destructor::TRANSIENT))
    //     .unwrap_or_else(|_| sqlite::result_error(ctx, "oom").unwrap());

    // Dynamic strings with custom deallocator:
    sqlite::strdyn("Hello, world!")
        .map(|s| {
            sqlite::result_text(
                ctx,
                s.into_raw(),
                -1,
                sqlite::Destructor::CUSTOM(sqlite::dropstr),
            )
        })
        .unwrap_or_else(|_| sqlite::result_error(ctx, "oom").unwrap());
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
