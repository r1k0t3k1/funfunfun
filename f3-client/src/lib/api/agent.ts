import { api, unwrap, type AgentResponse } from "./client";

/**
 * 指定した Listener に接続している Agent の一覧を取得する。
 *
 * openapi.json の drift 対応:
 *   utoipa 側は `params(ListAgentRequest)` を宣言しているが、生成された
 *   openapi.json には query パラメータが欠落していた。実サーバは
 *   `GET /agent/list?listener_id=<uuid>` を要求し、未指定だと
 *   「missing field `listener_id`」で失敗する。そのため openapi.json 側に
 *   `listener_id`(query, required) を補って追従している。
 */
export async function listAgents(listenerId: string): Promise<AgentResponse[]> {
  const { data, error, response } = await api.GET("/agent/list", {
    params: { query: { listener_id: listenerId } },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("エージェント一覧の取得に失敗しました");
  }
  // レスポンスは封筒 `{ result, status_code, data: AgentResponse[] }`。封筒を剥がす。
  return unwrap<AgentResponse[]>(data) ?? [];
}

/**
 * 指定した Agent の詳細を取得する。
 *
 * openapi.json の drift 対応: 実サーバは `GET /agent/get?agent_id=<uuid>` を
 * 要求する（未指定だと「missing field `agent_id`」）。openapi.json 側に
 * `agent_id`(query, required) を補って追従している。
 */
export async function getAgent(agentId: string): Promise<AgentResponse> {
  const { data, error, response } = await api.GET("/agent/get", {
    params: { query: { agent_id: agentId } },
  });
  // レスポンスは封筒 `{ result, status_code, data: AgentResponse }`。封筒を剥がす。
  const agent = unwrap<AgentResponse>(data);
  if (error !== undefined || !response.ok || agent === undefined) {
    throw new Error("エージェント詳細の取得に失敗しました");
  }
  return agent;
}
