#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

use core::ffi::c_char;
use sqlite_nostd as sqlite;
use sqlite_nostd::SQLite3Allocator;

use core::alloc::GlobalAlloc;
#[global_allocator]
static ALLOCATOR: SQLite3Allocator = SQLite3Allocator {};

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

// ---
// Move these two functions to a separate crate
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
// ---

// We only need to define these symbols when targetin WASM
// So we should stick them in a crate that our extension can
// include when the target is WASM.
#[no_mangle]
pub extern "C" fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { ALLOCATOR.alloc(Layout::from_size_align_unchecked(size, align)) }
}

#[no_mangle]
pub extern "C" fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) {
    unsafe { ALLOCATOR.dealloc(ptr, Layout::from_size_align_unchecked(size, align)) }
}

#[no_mangle]
pub extern "C" fn __rust_realloc(
    ptr: *mut u8,
    old_size: usize,
    size: usize,
    align: usize,
) -> *mut u8 {
    unsafe {
        ALLOCATOR.realloc(
            ptr,
            Layout::from_size_align_unchecked(old_size, align),
            size,
        )
    }
}

#[no_mangle]
pub extern "C" fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    unsafe { ALLOCATOR.alloc_zeroed(Layout::from_size_align_unchecked(size, align)) }
}

#[no_mangle]
pub extern "C" fn __rust_alloc_error_handler(_: Layout) -> ! {
    core::intrinsics::abort()
}
