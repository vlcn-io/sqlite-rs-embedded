# SQLite no_std

SQLite is lite. Its bindings should be lite too. They shold be able to be used _anywhere_ SQLite is used, _not_ incur any performance impact, _not_ include any extra dependencies, and be usable against _any_ SQLite version.

Thus this repository was born.

These bindings:

- Do not require the rust standard library
- Use the SQLite memory subsystem if no allocator exists
- Can be used to write SQLite extensions that compile to WASM and run in the browser
- Does 0 copying. E.g., through some tricks, Rust strings are passed directly to SQLite with no conversion to or copying to CString.
