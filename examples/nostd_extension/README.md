# example_extension

runtime loadable extension that sets up the sqlite3 memory subsystem as the global allocator.

no_std extensions can only target certain environments. wasm is the main one we're targeting.
