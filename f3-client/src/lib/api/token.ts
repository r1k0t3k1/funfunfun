import { writable } from "svelte/store";

/**
 * f3-server の認証はセッション Cookie から
 * `Authorization: Bearer <access_token>` 方式へ変更された。
 * ここではそのアクセストークン（= セッションID, 16進64文字）を
 * 一元管理する。
 *
 * ブラウザでもそのまま動くよう localStorage に永続化し、
 * リロードしてもログイン状態を保持する。
 * （Tauri / ブラウザのどちらの WebView でも localStorage は利用可能）
 */
const STORAGE_KEY = "f3.access_token";

function load(): string | null {
  if (typeof localStorage === "undefined") return null;
  return localStorage.getItem(STORAGE_KEY);
}

/** アクセストークンの Svelte ストア（null = 未認証）。 */
export const accessToken = writable<string | null>(load());

// ストアの購読と同期して、非同期の fetch から同期的に参照できる値を保持する。
let current: string | null = load();
accessToken.subscribe((v) => {
  current = v;
});

/** 現在のアクセストークンを同期的に取得する（fetch ミドルウェア用）。 */
export function getAccessToken(): string | null {
  return current;
}

/** ログイン成功時にトークンを保存する。 */
export function setAccessToken(token: string): void {
  current = token;
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_KEY, token);
  }
  accessToken.set(token);
}

/** ログアウト時にトークンを破棄する。 */
export function clearAccessToken(): void {
  current = null;
  if (typeof localStorage !== "undefined") {
    localStorage.removeItem(STORAGE_KEY);
  }
  accessToken.set(null);
}
