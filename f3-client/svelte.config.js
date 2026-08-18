// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { preprocessMeltUI } from "@melt-ui/pp";
import sequence from "svelte-sequential-preprocessor";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Melt UI の `use:melt={$builder}` は専用プリプロセッサが必須。
  // 未設定だと実行時に "cannot be used without MeltUI's Preprocessor" となり
  // ビルダーの属性/イベントが一切適用されない（= 開閉しない）。
  // vitePreprocess の後に流す必要があるため sequence で連結する。
  preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),
  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),
  },
};

export default config;
