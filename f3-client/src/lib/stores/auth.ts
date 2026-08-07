import { writable } from "svelte/store";
import { goto } from "$app/navigation";
import { api, type OperatorCredential } from "$lib/api/client";

/** 認証済みかどうか。SPA のメモリ上の状態として保持する。 */
export const isAuthenticated = writable<boolean>(false);

/** ログイン。成功したらダッシュボードへ遷移する。 */
export async function login(credential: OperatorCredential): Promise<void> {
  const { error, response } = await api.POST("/auth/login", {
    body: credential,
  });
  if (error !== undefined || !response.ok) {
    throw new Error("ユーザー名またはパスワードが正しくありません");
  }
  isAuthenticated.set(true);
  await goto("/dashboard/listeners");
}

/** ログアウト。ログイン画面へ遷移する。 */
export async function logout(): Promise<void> {
  await api.POST("/auth/logout");
  isAuthenticated.set(false);
  await goto("/");
}
