<script lang="ts">
  // Carbon Design System のスタイル（g100 = 最も暗いダークテーマ）を全体に適用する。
  import "carbon-components-svelte/css/g100.css";

  let { children } = $props();
</script>

{@render children()}

<style>
  /* ダークテーマに合わせてアプリ全体の背景・文字色を Carbon トークンで揃える。 */
  :global(html),
  :global(body) {
    background-color: var(--cds-background, #161616);
    color: var(--cds-text-primary, #f4f4f4);
  }
</style>
