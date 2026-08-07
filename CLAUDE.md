# プロジェクト概要
Tauri v2 + SvelteKit(TypeScript) 製デスクトップアプリ。
バックエンドAPI (https://localhost:8443) のクライアント。

# あなた(CC)の役割
- `f3-server/app/openapi.json` の変更に追従してクライアント実装を更新する
- 直前の変更内容は `git log` と `git diff HEAD~1 -- f3-server/app/openapi.json` で把握すること

# ワークフロー(毎回必ずこの順で)
1. openapi.json の diff を確認し、影響範囲を特定
2. `npm run generate:api` で型・APIクライアントを再生成
3. 生成物に合わせて UI / hooks / store を修正
4. `npm run lint && npm run test` を通す
5. `npm run dev:web` でフロントを起動し、Playwright MCP で実際に UI を操作して
   新機能・変更機能が動くことを確認(スクリーンショットも撮る)
6. コミットして PR を作成 (`gh pr create`)。本文にテスト結果とスクショの要約を書く

# 制約
- main へ直接 push しない。必ず feature ブランチ + PR
- f3-server以下のファイルは変更を禁止する
- API サーバ (http://localhost:8443) は起動済みの前提。落ちていたら
  実装を進めず、その旨を報告して終了すること
- 破壊的変更(既存エンドポイント削除等)を検知したら、影響を報告に明記

# コマンド
- 型生成: npm run generate:api   (openapi-typescript / orval)
- dev(ブラウザ): npm run dev:web  (Vite, http://localhost:1420)
- dev(Tauri): npm run tauri dev
- テスト: npm run test
- E2E(実バイナリ): npm run e2e:tauri  (重いので指示があった時のみ)
