import { api, unwrap, type OperatorResponse } from "./client";

/** オペレータ一覧を取得する。 */
export async function listOperators(): Promise<OperatorResponse[]> {
  // openapi.json 追従: /operator/list は POST から GET に変更された。
  const { data, error, response } = await api.GET("/operator/list");
  if (error !== undefined || !response.ok) {
    throw new Error("オペレータ一覧の取得に失敗しました");
  }
  // レスポンスは封筒 `{ result, status_code, data: OperatorResponse[] }`。
  // openapi.json にスキーマが無いため封筒を剥がして OperatorResponse[] として扱う。
  return unwrap<OperatorResponse[]>(data) ?? [];
}

/** オペレータの詳細を取得する。 */
export async function getOperator(
  operatorId: string,
): Promise<OperatorResponse> {
  // openapi.json 追従: /operator/get は POST(body) から GET(query) に変更された。
  const { data, error, response } = await api.GET("/operator/get", {
    params: { query: { operator_id: operatorId } },
  });
  // レスポンスは封筒 `{ result, status_code, data: OperatorResponse }`。封筒を剥がす。
  const operator = unwrap<OperatorResponse>(data);
  if (error !== undefined || !response.ok || operator === undefined) {
    throw new Error("オペレータ詳細の取得に失敗しました");
  }
  return operator;
}

/**
 * オペレータの有効化状態を切り替える（有効⇔無効）。
 * openapi.json 追従: /operator/toggle_status（POST, Admin 権限）を新規追加。
 */
export async function toggleOperatorStatus(operatorId: string): Promise<void> {
  const { error, response } = await api.POST("/operator/toggle_status", {
    body: { operator_id: operatorId },
  });
  if (error !== undefined || !response.ok) {
    throw new Error("オペレータの状態変更に失敗しました");
  }
}
