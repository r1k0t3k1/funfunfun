// 認証ヘッダ生成に関する純粋関数群。
// DOM / Tauri / Svelte に依存しないため、Node の `node --test` で単体テストできる。

/**
 * f3-server が発行するアクセストークンの形式（UUID）。
 * サーバは `/auth/login` のレスポンスでセッション ID（UUID v7）を
 * `access_token` として返し、認証ミドルウェアは Bearer トークンを
 * `Uuid::try_parse` で検証する。したがってクライアントも UUID 形式
 * （8-4-4-4-12 の 16 進表記）で妥当性を判定する。
 */
const TOKEN_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/**
 * アクセストークンが f3-server の認証ミドルウェアの
 * 形式要件（UUID）を満たすかを判定する。
 * @param {unknown} token
 * @returns {token is string}
 */
export function isValidToken(token) {
  return typeof token === "string" && TOKEN_PATTERN.test(token);
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
