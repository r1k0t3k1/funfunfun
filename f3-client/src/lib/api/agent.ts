import { api, unwrap, type AgentListItem } from "./client";

/**
 * 指定した Listener が管理する Agent の一覧を取得する。
 * openapi.json 追従: `/agent/list`（GET, クエリ `listener_id`）を新規追加。
 */
export async function listAgents(
  listenerId: string,
): Promise<AgentListItem[]> {
  const { data, error, response } = await api.GET("/agent/list", {
    params: { query: { listener_id: listenerId } },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("Agent 一覧の取得に失敗しました");
  }
  // レスポンスは封筒 `{ result, status_code, data: AgentResponse[] }`。封筒を剥がす。
  return unwrap<AgentListItem[]>(data) ?? [];
}

/**
 * Agent の詳細を取得する。
 * openapi.json 追従: `/agent/get`（GET, クエリ `agent_id`）を新規追加。
 */
export async function getAgent(agentId: string): Promise<AgentListItem> {
  const { data, error, response } = await api.GET("/agent/get", {
    params: { query: { agent_id: agentId } },
  });
  // レスポンスは封筒 `{ result, status_code, data: AgentResponse }`。封筒を剥がす。
  const agent = unwrap<AgentListItem>(data);
  if (error !== undefined || !response.ok || agent === undefined) {
    throw new Error("Agent 詳細の取得に失敗しました");
  }
  return agent;
}
