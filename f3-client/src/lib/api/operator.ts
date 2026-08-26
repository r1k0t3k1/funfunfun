import { api, unwrap, type OperatorResponse } from "./client";

/** オペレータ一覧を取得する。 */
export async function listOperators(): Promise<OperatorResponse[]> {
  const { data, error, response } = await api.POST("/operator/list");
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
  const { data, error, response } = await api.POST("/operator/get", {
    body: { operator_id: operatorId },
  });
  // レスポンスは封筒 `{ result, status_code, data: OperatorResponse }`。封筒を剥がす。
  const operator = unwrap<OperatorResponse>(data);
  if (error !== undefined || !response.ok || operator === undefined) {
    throw new Error("オペレータ詳細の取得に失敗しました");
  }
  return operator;
}
