import { derived } from "svelte/store";
import { goto } from "$app/navigation";
import {
  api,
  unwrap,
  type AuthenticateRequest,
  type AuthenticatedResponse,
} from "$lib/api/client";
import {
  accessToken,
  setAccessToken,
  clearAccessToken,
} from "$lib/api/token";
import { isValidToken } from "$lib/api/authHeader";

/**
 * 認証済みかどうか。
 * 認証状態はアクセストークンの有無で表す（トークンは localStorage に永続化）。
 */
export const isAuthenticated = derived(
  accessToken,
  ($token) => $token !== null,
);

/** ログイン。成功したらトークンを保存してダッシュボードへ遷移する。 */
export async function login(credential: AuthenticateRequest): Promise<void> {
  const { data, error, response } = await api.POST("/auth/login", {
    body: credential,
  });
  if (error !== undefined || !response.ok) {
    throw new Error("オペレータ名またはパスワードが正しくありません");
  }

  // レスポンスは封筒 `{ result, status_code, data: { access_token } }`。
  // openapi.json にスキーマが無いため型は付かない。封筒を剥がして手動型で受ける。
  const token = unwrap<AuthenticatedResponse>(data)?.access_token;
  if (!isValidToken(token)) {
    throw new Error("サーバからアクセストークンを取得できませんでした");
  }

  setAccessToken(token);
  await goto("/dashboard/listeners");
}

/** ログアウト。トークンを破棄してログイン画面へ遷移する。 */
export async function logout(): Promise<void> {
  try {
    // サーバ側の失効処理を試みる（失敗してもクライアントの破棄は行う）。
    await api.POST("/auth/logout");
  } catch {
    // ネットワークエラー等は無視してローカルの認証状態を確実に破棄する。
  } finally {
    clearAccessToken();
    await goto("/");
  }
}
