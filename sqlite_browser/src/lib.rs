#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

pub use sqlite_nostd::*;

use core::alloc::GlobalAlloc;
use core::ffi::c_char;
use sqlite_nostd::SQLite3Allocator;
#[global_allocator]
static ALLOCATOR: SQLite3Allocator = SQLite3Allocator {};

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

// TODO: allow registration of ext functions from
// crates using this one
pub fn register_extension() {}

#[no_mangle]
pub extern "C" fn sqlite3_sqlitebrowser_init(
    db: *mut sqlite3,
    _err_msg: *mut *mut c_char,
    api: *mut api_routines,
) -> u32 {
    EXTENSION_INIT2(api);

    // todo: call reigstered functions here

    OK
}
