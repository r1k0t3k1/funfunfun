// f3-server 共通レスポンス封筒（envelope）を扱う純粋関数。
// DOM / Tauri / Svelte / openapi-fetch に依存しないため、
// Node の `node --test` で単体テストできる。

/**
 * f3-server は全エンドポイントのボディを
 *   `{ "result": "OK" | "ERROR", "status_code": number, "data": <payload> }`
 * の形（封筒）で返すようになった。実データは常に `data` の下にネストされる。
 *
 * openapi.json にはこの封筒もレスポンススキーマも定義されていないため、
 * openapi-fetch は封筒全体を `data` として返す。この関数で封筒を剥がし、
 * 実データ（`data` フィールド）を取り出す。
 *
 * 封筒でない旧形式や null / undefined も安全に扱えるよう、
 * `data` フィールドを持たなければボディ自体を返す。
 *
 * @template T
 * @param {unknown} body openapi-fetch が返したレスポンスボディ
 * @returns {T | undefined}
 */
export function unwrap(body) {
  if (body !== null && typeof body === "object" && "data" in body) {
    return /** @type {{ data: T }} */ (body).data;
  }
  return /** @type {T | undefined} */ (body ?? undefined);
}
