import { api, type OperatorResponse } from "./client";

/** オペレータ一覧を取得する。 */
export async function listOperators(): Promise<OperatorResponse[]> {
  const { data, error, response } = await api.POST("/operator/list");
  if (error !== undefined || !response.ok) {
    throw new Error("オペレータ一覧の取得に失敗しました");
  }
  // openapi.json にレスポンススキーマが無いため any 相当。OperatorResponse[] として扱う。
  return (data as OperatorResponse[] | undefined) ?? [];
}

/** オペレータの詳細を取得する。 */
export async function getOperator(
  operatorId: string,
): Promise<OperatorResponse> {
  const { data, error, response } = await api.POST("/operator/get", {
    body: { operator_id: operatorId },
  });
  if (error !== undefined || !response.ok || data === undefined) {
    throw new Error("オペレータ詳細の取得に失敗しました");
  }
  // openapi.json にレスポンススキーマが無いため any 相当。OperatorResponse として扱う。
  return data as OperatorResponse;
}
