# example_extension

runtime loadable extension that sets up the sqlite3 memory subsystem as the global allocator.

no_std extensions can only target certain environments. wasm is the main one we're targeting.

Build:

```
cargo build --target wasm32-unknown-unknown
```

Build something that can be linked together with other wasm artifacts (e.g., WASM SQLite):

```
RUSTFLAGS="--emit=llvm-bc" cargo build --target wasm32-unknown-unknown
```

```
RUSTFLAGS="--emit=llvm-bc" cargo build --target wasm32-unknown-emscripten
```


---

-Z build-std=panic_abort,std

-Z build-std=panic_abort