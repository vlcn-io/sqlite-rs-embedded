import SQLiteAsyncESMFactory from "wa-sqlite/debug/wa-sqlite-async.mjs";
import * as SQLite from "wa-sqlite";

// @ts-ignore
import wasmUrl from "wa-sqlite/debug/wa-sqlite-async.wasm?url";

const wasmModule = await SQLiteAsyncESMFactory({
  locateFile(file: string) {
    return wasmUrl;
  },
});
const sqlite3 = SQLite.Factory(wasmModule);

sqlite3.open_v2(
  ":memory:",
  SQLite.SQLITE_OPEN_CREATE |
    SQLite.SQLITE_OPEN_READWRITE |
    SQLite.SQLITE_OPEN_URI,
  undefined
);
