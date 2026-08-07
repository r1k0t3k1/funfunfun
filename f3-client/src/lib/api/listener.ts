import { api, type CreateListenerRequest, type ListenerListItem } from "./client";

/** Listener 一覧を取得する。 */
export async function listListeners(): Promise<ListenerListItem[]> {
  const { data, error, response } = await api.GET("/listener/list");
  if (error !== undefined || !response.ok) {
    throw new Error("Listener 一覧の取得に失敗しました");
  }
  // openapi.json にレスポンススキーマが無いため any 相当。ListenerListItem[] として扱う。
  return (data as ListenerListItem[] | undefined) ?? [];
}

/** Listener を作成する。 */
export async function createListener(
  body: CreateListenerRequest,
): Promise<void> {
  const { error, response } = await api.POST("/listener/create", { body });
  if (error !== undefined || !response.ok) {
    throw new Error("Listener の作成に失敗しました");
  }
}

/** Listener を起動する。 */
export async function startListener(listenerId: string): Promise<void> {
  const { error, response } = await api.POST("/listener/start", {
    body: { listener_id: listenerId },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("Listener の起動に失敗しました");
  }
}

/** Listener を停止する。 */
export async function stopListener(listenerId: string): Promise<void> {
  const { error, response } = await api.POST("/listener/stop", {
    body: { listener_id: listenerId },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("Listener の停止に失敗しました");
  }
}

/** Listener を削除する。 */
export async function removeListener(listenerId: string): Promise<void> {
  const { error, response } = await api.POST("/listener/remove", {
    body: { listener_id: listenerId },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("Listener の削除に失敗しました");
  }
}
