import createClient from "openapi-fetch";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import type { paths, components } from "./schema";

/**
 * f3-server のベース URL。
 * ホストは localhost:8443（HTTPS）。環境変数 VITE_API_BASE_URL で上書き可能。
 */
export const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? "https://localhost:8443";

/**
 * localhost / 127.0.0.1 の開発サーバは自己署名証明書のため、
 * その場合のみ TLS 検証を無効化する（本番の外部ホストでは有効のまま）。
 */
const isLocalHost = /^https?:\/\/(localhost|127\.0\.0\.1)(:|\/|$)/.test(
  API_BASE_URL,
);

/**
 * openapi-fetch から利用するカスタム fetch。
 *
 * webview 標準の fetch はクロスオリジンのセッション Cookie を
 * （SameSite / WebKit のサードパーティ Cookie ブロック等で）送出できない。
 * Tauri HTTP プラグイン（Rust の reqwest 経由）を使うことで CORS を回避し、
 * reqwest の cookie_store がセッション Cookie を自動で保持・送出する。
 *
 * openapi-fetch は fetch(request, requestInitExt) の形で呼ぶため、
 * ClientOptions（danger 等）はここで注入する。
 */
async function apiFetch(input: Request): Promise<Response> {
  try {
    return await tauriFetch(input, {
      danger: isLocalHost
        ? { acceptInvalidCerts: true, acceptInvalidHostnames: true }
        : undefined,
    });
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
 * Cookie は Tauri HTTP プラグイン側の cookie_store が保持するため
 * credentials 指定は不要。
 */
export const api = createClient<paths>({
  baseUrl: API_BASE_URL,
  fetch: apiFetch,
});

// ---- openapi.json 由来のリクエスト型エイリアス ----
export type OperatorCredential = components["schemas"]["OperatorCredential"];
export type CreateListenerRequest =
  components["schemas"]["CreateListenerRequest"];
export type RemoveListenerRequest =
  components["schemas"]["RemoveListenerRequest"];
export type StartListenerRequest =
  components["schemas"]["StartListenerRequest"];
export type StopListenerRequest = components["schemas"]["StopListenerRequest"];
export type ListenerType = components["schemas"]["ListenerType"];

/**
 * /listener/list のレスポンス要素。
 * openapi.json にレスポンススキーマが定義されていないため、
 * サーバの ListListenerResponse に合わせて手動定義する。
 */
export type ListenerListItem = {
  name: string;
  addr: string;
};
