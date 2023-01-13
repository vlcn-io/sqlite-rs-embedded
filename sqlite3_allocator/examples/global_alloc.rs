#![no_std]
#![no_main]

extern crate alloc;

#[global_allocator]
static ALLOCATOR: SQLite3Allocator = SQLite3Allocator {};

#[entry]
fn main() -> ! {
    // Must invoke sqlite3_capi::SQLITE_EXTENSION_INIT2 from within sqlite3_init
    // such that the memory subsystem gets initialized properly.
}
