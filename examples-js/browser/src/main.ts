import SQLiteAsyncESMFactory from "wa-sqlite/debug/wa-sqlite-async.mjs";
import * as SQLite from "wa-sqlite";
import { tag } from "./tag";

// @ts-ignore
import wasmUrl from "wa-sqlite/debug/wa-sqlite-async.wasm?url";

const wasmModule = await SQLiteAsyncESMFactory({
  locateFile(file: string) {
    return wasmUrl;
  },
});
const sqlite3 = SQLite.Factory(wasmModule);

const db = ((window as any).db = await sqlite3.open_v2(
  ":memory:",
  SQLite.SQLITE_OPEN_CREATE |
    SQLite.SQLITE_OPEN_READWRITE |
    SQLite.SQLITE_OPEN_URI,
  undefined
));

(window as any).sqlite3 = sqlite3;

const sql = tag(sqlite3, db);

console.log(await sql`CREATE TABLE foo (a)`);
console.log(await sql`INSERT INTO foo VALUES (1)`);

console.log(await sql`SELECT * FROM foo`);
