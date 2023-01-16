extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

#[cfg(not(feature = "std"))]
use num_derive::FromPrimitive;
#[cfg(not(feature = "std"))]
use num_traits::FromPrimitive;

pub use sqlite3_allocator::*;
pub use sqlite3_capi::*;

// pub struct Stmt {
//     /// Internal pointer to the C stmt
//     stmt: *mut stmt,
// }

pub trait DB<'a> {
    fn prepare_v2(&self, sql: &str) -> Result<Box<dyn Stmt<'a>>, ResultCode>;
}

#[derive(FromPrimitive)]
pub enum ResultCode {
    OK = 0,
    ERROR = 1,
    INTERNAL = 2,
    PERM = 3,
    ABORT = 4,
    BUSY = 5,
    LOCKED = 6,
    NOMEM = 7,
    READONLY = 8,
    INTERRUPT = 9,
    IOERR = 10,
    CORRUPT = 11,
    NOTFOUND = 12,
    FULL = 13,
    CANTOPEN = 14,
    PROTOCOL = 15,
    EMPTY = 16,
    SCHEMA = 17,
    TOOBIG = 18,
    CONSTRAINT = 19,
    MISMATCH = 20,
    MISUSE = 21,
    NOLFS = 22,
    AUTH = 23,
    FORMAT = 24,
    RANGE = 25,
    NOTADB = 26,
    NOTICE = 27,
    WARNING = 28,
    ROW = 100,
    DONE = 101,
    ERROR_MISSING_COLLSEQ = bindings::SQLITE_ERROR_MISSING_COLLSEQ as isize,
    ERROR_RETRY = bindings::SQLITE_ERROR_RETRY as isize,
    ERROR_SNAPSHOT = bindings::SQLITE_ERROR_SNAPSHOT as isize,
    IOERR_READ = bindings::SQLITE_IOERR_READ as isize,
    IOERR_SHORT_READ = bindings::SQLITE_IOERR_SHORT_READ as isize,
    IOERR_WRITE = bindings::SQLITE_IOERR_WRITE as isize,
    IOERR_FSYNC = bindings::SQLITE_IOERR_FSYNC as isize,
    IOERR_DIR_FSYNC = bindings::SQLITE_IOERR_DIR_FSYNC as isize,
    IOERR_TRUNCATE = bindings::SQLITE_IOERR_TRUNCATE as isize,
    IOERR_FSTAT = bindings::SQLITE_IOERR_FSTAT as isize,
    IOERR_UNLOCK = bindings::SQLITE_IOERR_UNLOCK as isize,
    IOERR_RDLOCK = bindings::SQLITE_IOERR_RDLOCK as isize,
    IOERR_DELETE = bindings::SQLITE_IOERR_DELETE as isize,
    IOERR_BLOCKED = bindings::SQLITE_IOERR_BLOCKED as isize,
    IOERR_NOMEM = bindings::SQLITE_IOERR_NOMEM as isize,
    IOERR_ACCESS = bindings::SQLITE_IOERR_ACCESS as isize,
    IOERR_CHECKRESERVEDLOCK = bindings::SQLITE_IOERR_CHECKRESERVEDLOCK as isize,
    IOERR_LOCK = bindings::SQLITE_IOERR_LOCK as isize,
    IOERR_CLOSE = bindings::SQLITE_IOERR_CLOSE as isize,
    IOERR_DIR_CLOSE = bindings::SQLITE_IOERR_DIR_CLOSE as isize,
    IOERR_SHMOPEN = bindings::SQLITE_IOERR_SHMOPEN as isize,
    IOERR_SHMSIZE = bindings::SQLITE_IOERR_SHMSIZE as isize,
    IOERR_SHMLOCK = bindings::SQLITE_IOERR_SHMLOCK as isize,
    IOERR_SHMMAP = bindings::SQLITE_IOERR_SHMMAP as isize,
    IOERR_SEEK = bindings::SQLITE_IOERR_SEEK as isize,
    IOERR_DELETE_NOENT = bindings::SQLITE_IOERR_DELETE_NOENT as isize,
    IOERR_MMAP = bindings::SQLITE_IOERR_MMAP as isize,
    IOERR_GETTEMPPATH = bindings::SQLITE_IOERR_GETTEMPPATH as isize,
    IOERR_CONVPATH = bindings::SQLITE_IOERR_CONVPATH as isize,
    IOERR_VNODE = bindings::SQLITE_IOERR_VNODE as isize,
    IOERR_AUTH = bindings::SQLITE_IOERR_AUTH as isize,
    IOERR_BEGIN_ATOMIC = bindings::SQLITE_IOERR_BEGIN_ATOMIC as isize,
    IOERR_COMMIT_ATOMIC = bindings::SQLITE_IOERR_COMMIT_ATOMIC as isize,
    IOERR_ROLLBACK_ATOMIC = bindings::SQLITE_IOERR_ROLLBACK_ATOMIC as isize,
    IOERR_DATA = bindings::SQLITE_IOERR_DATA as isize,
    IOERR_CORRUPTFS = bindings::SQLITE_IOERR_CORRUPTFS as isize,
    LOCKED_SHAREDCACHE = bindings::SQLITE_LOCKED_SHAREDCACHE as isize,
    LOCKED_VTAB = bindings::SQLITE_LOCKED_VTAB as isize,
    BUSY_RECOVERY = bindings::SQLITE_BUSY_RECOVERY as isize,
    BUSY_SNAPSHOT = bindings::SQLITE_BUSY_SNAPSHOT as isize,
    BUSY_TIMEOUT = bindings::SQLITE_BUSY_TIMEOUT as isize,
    CANTOPEN_NOTEMPDIR = bindings::SQLITE_CANTOPEN_NOTEMPDIR as isize,
    CANTOPEN_ISDIR = bindings::SQLITE_CANTOPEN_ISDIR as isize,
    CANTOPEN_FULLPATH = bindings::SQLITE_CANTOPEN_FULLPATH as isize,
    CANTOPEN_CONVPATH = bindings::SQLITE_CANTOPEN_CONVPATH as isize,
    CANTOPEN_DIRTYWAL = bindings::SQLITE_CANTOPEN_DIRTYWAL as isize,
    CANTOPEN_SYMLINK = bindings::SQLITE_CANTOPEN_SYMLINK as isize,
    CORRUPT_VTAB = bindings::SQLITE_CORRUPT_VTAB as isize,
    CORRUPT_SEQUENCE = bindings::SQLITE_CORRUPT_SEQUENCE as isize,
    CORRUPT_INDEX = bindings::SQLITE_CORRUPT_INDEX as isize,
    READONLY_RECOVERY = bindings::SQLITE_READONLY_RECOVERY as isize,
    READONLY_CANTLOCK = bindings::SQLITE_READONLY_CANTLOCK as isize,
    READONLY_ROLLBACK = bindings::SQLITE_READONLY_ROLLBACK as isize,
    READONLY_DBMOVED = bindings::SQLITE_READONLY_DBMOVED as isize,
    READONLY_CANTINIT = bindings::SQLITE_READONLY_CANTINIT as isize,
    READONLY_DIRECTORY = bindings::SQLITE_READONLY_DIRECTORY as isize,
    ABORT_ROLLBACK = bindings::SQLITE_ABORT_ROLLBACK as isize,
    CONSTRAINT_CHECK = bindings::SQLITE_CONSTRAINT_CHECK as isize,
    CONSTRAINT_COMMITHOOK = bindings::SQLITE_CONSTRAINT_COMMITHOOK as isize,
    CONSTRAINT_FOREIGNKEY = bindings::SQLITE_CONSTRAINT_FOREIGNKEY as isize,
    CONSTRAINT_FUNCTION = bindings::SQLITE_CONSTRAINT_FUNCTION as isize,
    CONSTRAINT_NOTNULL = bindings::SQLITE_CONSTRAINT_NOTNULL as isize,
    CONSTRAINT_PRIMARYKEY = bindings::SQLITE_CONSTRAINT_PRIMARYKEY as isize,
    CONSTRAINT_TRIGGER = bindings::SQLITE_CONSTRAINT_TRIGGER as isize,
    CONSTRAINT_UNIQUE = bindings::SQLITE_CONSTRAINT_UNIQUE as isize,
    CONSTRAINT_VTAB = bindings::SQLITE_CONSTRAINT_VTAB as isize,
    CONSTRAINT_ROWID = bindings::SQLITE_CONSTRAINT_ROWID as isize,
    CONSTRAINT_PINNED = bindings::SQLITE_CONSTRAINT_PINNED as isize,
    CONSTRAINT_DATATYPE = bindings::SQLITE_CONSTRAINT_DATATYPE as isize,
    NOTICE_RECOVER_WAL = bindings::SQLITE_NOTICE_RECOVER_WAL as isize,
    NOTICE_RECOVER_ROLLBACK = bindings::SQLITE_NOTICE_RECOVER_ROLLBACK as isize,
    WARNING_AUTOINDEX = bindings::SQLITE_WARNING_AUTOINDEX as isize,
    AUTH_USER = bindings::SQLITE_AUTH_USER as isize,
    OK_LOAD_PERMANENTLY = bindings::SQLITE_OK_LOAD_PERMANENTLY as isize,
    OK_SYMLINK = bindings::SQLITE_OK_SYMLINK as isize,

    NULL = 5000,
}

#[derive(FromPrimitive)]
pub enum ColumnType {
    Integer = 1,
    Float = 2,
    Text = 3,
    Blob = 4,
    Null = 5,
}

pub trait Stmt<'a> {
    // TODO: step should be a result as only two result codes are not
    // errors for step
    fn step(&self) -> ResultCode;
    fn column_count(&self) -> i32;
    fn column_name(&self, i: i32) -> Result<&'a str, ResultCode>;
    fn column_type(&self, i: i32) -> Result<ColumnType, ResultCode>;
    fn column_text(&self, i: i32) -> Result<&'a str, ResultCode>;
    fn column_blob(&self, i: i32) -> Result<&'a [u8], ResultCode>;
    fn column_double(&self, i: i32) -> Result<f64, ResultCode>;
    fn column_int(&self, i: i32) -> Result<i32, ResultCode>;
    fn column_int64(&self, i: i32) -> Result<i64, ResultCode>;
}

impl<'a> Stmt<'a> for *mut stmt {
    fn step(&self) -> ResultCode {
        ResultCode::from_i32(step(*self)).unwrap()
    }

    fn column_count(&self) -> i32 {
        column_count(*self)
    }

    fn column_name(&self, i: i32) -> Result<&'a str, ResultCode> {
        let ptr = column_name(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(
                unsafe {
                    core::str::from_utf8_unchecked(core::ffi::CStr::from_ptr(ptr).to_bytes())
                },
            )
        }
    }

    fn column_type(&self, i: i32) -> Result<ColumnType, ResultCode> {
        ColumnType::from_i32(column_type(*self, i)).ok_or(ResultCode::NULL)
    }

    fn column_text(&self, i: i32) -> Result<&'a str, ResultCode> {
        let ptr = column_text(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(unsafe {
                core::str::from_utf8_unchecked(
                    core::ffi::CStr::from_ptr(ptr as *const i8).to_bytes(),
                )
            })
        }
    }

    fn column_blob(&self, i: i32) -> Result<&'a [u8], ResultCode> {
        let len = column_bytes(*self, i);
        let ptr = column_blob(*self, i);
        if ptr.is_null() {
            Err(ResultCode::NULL)
        } else {
            Ok(unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) })
        }
    }

    fn column_double(&self, i: i32) -> Result<f64, ResultCode> {
        Ok(column_double(*self, i))
    }

    fn column_int(&self, i: i32) -> Result<i32, ResultCode> {
        Ok(column_int(*self, i))
    }

    fn column_int64(&self, i: i32) -> Result<i64, ResultCode> {
        Ok(column_int64(*self, i))
    }
}

// impl Drop for *mut stmt {
//     fn drop(&mut self) {
//         finalize(*self);
//     }
// }

pub trait Context {
    /// Pass and give ownership of the string to SQLite.
    /// SQLite will not copy the string.
    /// This method will correctly drop the string when SQLite is finished
    /// using it.
    fn result_text_owned(&self, text: String);
    fn result_text_shared(&self, text: &str);
    fn result_text_static(&self, text: &'static str);
    fn result_blob_owned(&self, blob: Vec<u8>);
    fn result_blob_shared(&self, blob: &[u8]);
    fn result_blob_static(&self, blob: &'static [u8]);
}

impl Context for *mut context {
    fn result_text_owned(&self, text: String) {
        let (ptr, len, _) = text.into_raw_parts();
        result_text(
            *self,
            ptr as *const c_char,
            len as i32,
            Destructor::CUSTOM(droprust),
        );
    }

    /// Takes a reference to a string, has SQLite copy the contents
    /// and take ownership of the copy.
    fn result_text_shared(&self, text: &str) {
        result_text(
            *self,
            text.as_ptr() as *mut c_char,
            text.len() as i32,
            Destructor::TRANSIENT,
        );
    }

    /// Takes a reference to a string that is statically allocated.
    /// SQLite will not copy this string.
    fn result_text_static(&self, text: &'static str) {
        result_text(
            *self,
            text.as_ptr() as *mut c_char,
            text.len() as i32,
            Destructor::STATIC,
        );
    }

    fn result_blob_owned(&self, blob: Vec<u8>) {
        let (ptr, len, _) = blob.into_raw_parts();
        result_blob(*self, ptr, len as i32, Destructor::CUSTOM(droprust));
    }

    fn result_blob_shared(&self, blob: &[u8]) {
        result_blob(
            *self,
            blob.as_ptr(),
            blob.len() as i32,
            Destructor::TRANSIENT,
        );
    }

    fn result_blob_static(&self, blob: &'static [u8]) {
        result_blob(*self, blob.as_ptr(), blob.len() as i32, Destructor::STATIC);
    }
}
