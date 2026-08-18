// 認証ヘッダ生成に関する純粋関数群。
// DOM / Tauri / Svelte に依存しないため、Node の `node --test` で単体テストできる。

/** f3-server が発行するセッショントークンの長さ（16進64文字）。 */
export const TOKEN_LENGTH = 64;

/**
 * セッショントークンが f3-server の認証ミドルウェアの
 * 形式要件（64文字）を満たすかを判定する。
 * @param {unknown} token
 * @returns {token is string}
 */
export function isValidToken(token) {
  return typeof token === "string" && token.length === TOKEN_LENGTH;
}

/**
 * `Authorization: Bearer <token>` に使う値を組み立てる。
 * トークンが無い / 空なら null（ヘッダを付与しない）を返す。
 * @param {string | null | undefined} token
 * @returns {string | null}
 */
export function bearer(token) {
  if (!token) return null;
  return `Bearer ${token}`;
}
