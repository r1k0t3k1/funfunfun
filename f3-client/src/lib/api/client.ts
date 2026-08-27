import createClient from "openapi-fetch";
import { isTauri } from "@tauri-apps/api/core";
import type { paths, components } from "./schema";
import { getAccessToken } from "./token";
import { bearer } from "./authHeader";

/**
 * f3-server のベース URL。
 * ホストは localhost:8443（HTTPS）。環境変数 VITE_API_BASE_URL で上書き可能。
 */
export const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? "https://localhost:8443";

/**
 * localhost / 127.0.0.1 の開発サーバは自己署名証明書のため、
 * Tauri（reqwest）経由の場合のみ TLS 検証を無効化する。
 */
const isLocalHost = /^https?:\/\/(localhost|127\.0\.0\.1)(:|\/|$)/.test(
  API_BASE_URL,
);

/**
 * openapi-fetch から利用するカスタム fetch。
 *
 * 認証が Cookie から `Authorization: Bearer` トークンへ変わったため、
 * クロスオリジン Cookie を保持する目的で Tauri HTTP プラグインを使う必要は
 * なくなった。よって：
 *   - ブラウザ（vite dev / dev:web）では標準の fetch をそのまま使い、
 *     HTML+JS だけで画面遷移・API 操作ができる。
 *   - Tauri WebView 上でのみ、自己署名証明書を許可するために
 *     HTTP プラグイン（reqwest）へフォールバックする。
 *
 * プラグインはブラウザには存在しないため、Tauri 実行時だけ動的 import する。
 */
async function apiFetch(input: Request): Promise<Response> {
  try {
    if (isTauri()) {
      const { fetch: tauriFetch } = await import("@tauri-apps/plugin-http");
      return await tauriFetch(input, {
        danger: isLocalHost
          ? { acceptInvalidCerts: true, acceptInvalidHostnames: true }
          : undefined,
      });
    }
    // ブラウザ実行時は標準 fetch。Cookie に依存しないため credentials は不要。
    return await globalThis.fetch(input);
  } catch (e) {
    // Tauri プラグインは文字列を throw することがあるため、
    // 元のエラー内容を握り潰さず Error にして上位へ伝える。
    const detail = e instanceof Error ? e.message : String(e);
    console.error("[api] request failed:", input.method, input.url, e);
    throw new Error(`通信エラー: ${detail}`);
  }
}

/**
 * openapi.json から生成した型（./schema.d.ts）を用いた型安全な HTTP クライアント。
 */
export const api = createClient<paths>({
  baseUrl: API_BASE_URL,
  fetch: apiFetch,
});

/**
 * 認証ミドルウェア。
 * 保持しているアクセストークンを `Authorization: Bearer <token>` として
 * 全リクエストに付与する（未ログイン時は付与しない）。
 * ログイン系エンドポイントは AuthN 対象外なので余分に付いても無害。
 */
api.use({
  onRequest({ request }) {
    const header = bearer(getAccessToken());
    if (header) {
      request.headers.set("Authorization", header);
    }
    return request;
  },
});

/**
 * f3-server の共通レスポンス封筒（envelope）。
 * サーバは全エンドポイントのボディを
 *   `{ "result": "OK" | "ERROR", "status_code": number, "data": <payload> }`
 * の形で返すようになった。実データは常に `data` フィールドの下にネストされる。
 *
 * openapi.json にはこの封筒もレスポンススキーマも定義されていないため、
 * サーバ実装（ApiResponse）に合わせて手動定義する。
 */
export type ApiEnvelope<T> = {
  result: "OK" | "ERROR";
  status_code: number;
  data: T;
};

/**
 * openapi-fetch が返すボディ（= 封筒）から実データ（`data`）を取り出す。
 * 実装は純粋関数として ./envelope.js に切り出し、単体テスト可能にしている。
 */
export { unwrap } from "./envelope.js";

// ---- openapi.json 由来のリクエスト型エイリアス ----
export type AuthenticateRequest = components["schemas"]["AuthenticateRequest"];
export type CreateListenerRequest =
  components["schemas"]["CreateListenerRequest"];
export type RemoveListenerRequest =
  components["schemas"]["RemoveListenerRequest"];
export type StartListenerRequest =
  components["schemas"]["StartListenerRequest"];
export type StopListenerRequest = components["schemas"]["StopListenerRequest"];
export type ListenerType = components["schemas"]["ListenerType"];
// /operator/toggle_status のリクエスト型（operator_id のみ）。
export type ToggleOperatorStatusRequest =
  components["schemas"]["ToggleOperatorStatusRequest"];

/**
 * /auth/login のレスポンス。
 * openapi.json にレスポンススキーマが定義されていないため、
 * サーバの AuthenticatedResponse に合わせて手動定義する。
 */
export type AuthenticatedResponse = {
  access_token: string;
};

/**
 * /listener/list のレスポンス要素。
 * openapi.json に `ListenerResponse` スキーマが定義されたため、
 * 生成型（./schema.d.ts）をそのまま利用する。
 * （レスポンスは配列 `ListenerResponse[]` を封筒に包んで返る）
 */
export type ListenerListItem = components["schemas"]["ListenerResponse"];

/**
 * オペレータの権限ロール。
 * サーバの Role（Admin / Write / Read）に対応する文字列。
 */
export type OperatorRole = "Admin" | "Write" | "Read";

/**
 * /operator/list・/operator/get のレスポンス要素。
 * openapi.json にレスポンススキーマが定義されていないため、
 * サーバの OperatorResponse に合わせて手動定義する。
 */
export type OperatorResponse = {
  id: string;
  name: string;
  description: string;
  role: OperatorRole;
  /** アカウントが有効化されているか（/operator/toggle_status で切り替え）。 */
  is_enabled: boolean;
};
