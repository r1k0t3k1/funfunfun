// @ts-nocheck -- Node 組み込みの test runner 用。@types/node 非導入のため型検査対象外。
import { test } from "node:test";
import assert from "node:assert/strict";
import { bearer, isValidToken, TOKEN_LENGTH } from "./authHeader.js";

const VALID = "a".repeat(TOKEN_LENGTH);

test("isValidToken: 64文字のトークンを受理する", () => {
  assert.equal(isValidToken(VALID), true);
});

test("isValidToken: 長さが違う / null / 非文字列は拒否する", () => {
  assert.equal(isValidToken("a".repeat(63)), false);
  assert.equal(isValidToken("a".repeat(65)), false);
  assert.equal(isValidToken(""), false);
  assert.equal(isValidToken(null), false);
  assert.equal(isValidToken(undefined), false);
  assert.equal(isValidToken(12345), false);
});

test("bearer: トークンがあれば 'Bearer <token>' を返す", () => {
  assert.equal(bearer(VALID), `Bearer ${VALID}`);
});

test("bearer: トークンが無ければ null を返す（ヘッダを付与しない）", () => {
  assert.equal(bearer(null), null);
  assert.equal(bearer(undefined), null);
  assert.equal(bearer(""), null);
});
