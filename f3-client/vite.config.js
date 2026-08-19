import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
// @ts-expect-error type error without @types/node package
import process from "node:process";
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [sveltekit()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || "127.0.0.1",
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    // 4. ルートの .svelte を起動時にプリコンパイルしておく。
    //
    //    vite-plugin-svelte は `Foo.svelte` をコンパイルした際に、その
    //    `<style>` を `Foo.svelte?svelte&type=style&lang.css` という仮想
    //    モジュールとしてキャッシュに載せる。ブラウザ側のモジュールグラフが
    //    古い（依存を差し替えた直後にリロードしていない等）と、JS 本体より
    //    先に CSS だけを要求することがあり、その時キャッシュが空だと
    //      [vite-plugin-svelte:load] failed to load virtual css module ...
    //    を出したうえで Vite が .svelte ファイルを「生のCSS」として読み込む。
    //    結果、ソース全文がスタイルシートとして注入され、CSSパーサが壊れた
    //    先頭部分を読み飛ばす過程で先頭のルール（ログイン画面なら
    //    `.login-page` の中央寄せ）が丸ごと消える。
    //    warmup で必ずキャッシュを温めておけば、この経路には入らない。
    warmup: {
      clientFiles: ["./src/routes/**/*.svelte", "./src/lib/**/*.svelte"],
    },
  },
}));
