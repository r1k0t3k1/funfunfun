// @ts-nocheck -- Node 組み込みの test runner 用。@types/node 非導入のため型検査対象外。
import { test } from "node:test";
import assert from "node:assert/strict";
import { unwrap } from "./envelope.js";

test("unwrap: 封筒から access_token 入りの data を取り出す", () => {
  const body = {
    result: "OK",
    status_code: 200,
    data: { access_token: "a".repeat(64) },
  };
  assert.deepEqual(unwrap(body), { access_token: "a".repeat(64) });
});

test("unwrap: data が配列でもそのまま取り出す", () => {
  const body = { result: "OK", status_code: 200, data: [{ id: "1" }] };
  assert.deepEqual(unwrap(body), [{ id: "1" }]);
});

test("unwrap: data が null / 空配列でもその値を返す", () => {
  assert.equal(unwrap({ result: "OK", status_code: 200, data: null }), null);
  assert.deepEqual(unwrap({ result: "OK", status_code: 200, data: [] }), []);
});

test("unwrap: 封筒でない旧形式のボディはそのまま返す", () => {
  assert.deepEqual(unwrap({ access_token: "x" }), { access_token: "x" });
});

test("unwrap: undefined / null は undefined を返す", () => {
  assert.equal(unwrap(undefined), undefined);
  assert.equal(unwrap(null), undefined);
});
