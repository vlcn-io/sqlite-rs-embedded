import SQLiteAsyncESMFactory from "wa-sqlite/dist/wa-sqlite-async.mjs";
import * as SQLite from "wa-sqlite";

const wasmModule = await SQLiteAsyncESMFactory({
  locateFile(file: string) {
    return new URL(file, import.meta.url).href;
  },
});
const sqlite3 = SQLite.Factory(wasmModule);
