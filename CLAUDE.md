# プロジェクト概要
Tauri v2 + SvelteKit(TypeScript) 製デスクトップアプリ。
バックエンドAPI (https://localhost:8443) のクライアント。

- クライアント: `f3-client/` (npm プロジェクトのルートはここ。リポジトリ直下に package.json は無い)
- サーバ: `f3-server/` (Rust / cargo-make ワークスペース)

# あなた(CC)の役割
- `f3-server/app/openapi.json` の変更に追従してクライアント実装を更新する
- 直前の変更内容は `git log` と `git diff HEAD~1 -- f3-server/app/openapi.json` で把握すること

# openapi.json の扱い
- 正となる定義: `f3-server/app/openapi.json`
  (サーバ側で `cargo test export_openapi_json` により生成。CI でも差分チェックされる)
- 型生成の入力: `f3-client/openapi.json`
  (上記のコピー。`npm run gen:api` はこちらを読む)
- したがって追従時は **server → client にコピーしてから型生成する**:
  `cp f3-server/app/openapi.json f3-client/openapi.json && (cd f3-client && npm run gen:api)`
- 生成物: `f3-client/src/lib/api/schema.d.ts` (直接編集しない)

# ワークフロー(毎回必ずこの順で)
1. `f3-server/app/openapi.json` の diff を確認し、影響範囲を特定
2. `f3-client/openapi.json` へコピーし、`npm run gen:api` で型を再生成
3. 生成物に合わせて実装を修正
   - UI: `f3-client/src/routes/`
   - ストア: `f3-client/src/lib/stores/`
   - API ラッパ: `f3-client/src/lib/api/` (`client.ts` / `listener.ts`)
4. `npm run check` (svelte-kit sync + svelte-check) を通す
   ※ lint / test スクリプトは現状存在しない。型チェックがゲート
5. UI を実際に操作して新機能・変更機能が動くことを確認(スクリーンショットも撮る)
   - API 通信は `@tauri-apps/plugin-http` (Tauri IPC 経由) で行うため、
     **ブラウザだけの `npm run dev` では API 呼び出しは動かない**。
     API を伴う確認は `npm run tauri dev` を使うこと
   - 画面レイアウトなど API 非依存の確認は `npm run dev`
     (http://localhost:1420) + Playwright MCP で可
6. コミットして PR を作成 (`gh pr create`)。本文にテスト結果とスクショの要約を書く

# 制約
- main へ直接 push しない。必ず feature ブランチ + PR
- f3-server以下のファイルは変更を禁止する
  (`f3-client/openapi.json` へのコピーは可)
- API サーバ (https://localhost:8443) は起動済みの前提。落ちていたら
  実装を進めず、その旨を報告して終了すること
  疎通確認: `curl -sk https://localhost:8443/health/db`
  (自己署名証明書のため `-k` が必要)
- 破壊的変更(既存エンドポイント削除等)を検知したら、影響を報告に明記

# コマンド
すべて `f3-client/` ディレクトリで実行する。

- 依存インストール: `npm ci --legacy-peer-deps`
  (openapi-typescript が typescript ^5.x を要求する一方、本体は ~6.0.3 を使うため
   素の `npm ci` は ERESOLVE で失敗する)
- 型生成: `npm run gen:api`   (openapi-typescript: `./openapi.json` → `src/lib/api/schema.d.ts`)
- 型チェック: `npm run check`  (`--watch` 版は `npm run check:watch`)
- dev(ブラウザ): `npm run dev`  (Vite, http://localhost:1420)
- dev(Tauri): `npm run tauri dev`
- ビルド: `npm run build`  (adapter-static / SPA。出力は `f3-client/build`)

# API のベース URL
- 既定値: `https://localhost:8443` (`src/lib/api/client.ts` の `API_BASE_URL`)
- 環境変数 `VITE_API_BASE_URL` で上書き可能
- localhost/127.0.0.1 は自己署名証明書のため TLS 検証を無効化している
- 接続先は Tauri の capability (`src-tauri/capabilities/default.json`) でも
  許可が必要。既定は `https://localhost:8443/*` と `https://127.0.0.1:8443/*`
