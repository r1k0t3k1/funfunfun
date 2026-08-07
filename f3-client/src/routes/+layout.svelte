<script lang="ts">
  // Carbon Design System のスタイル（g10 テーマ）を全体に適用する。
  import "carbon-components-svelte/css/g10.css";

  let { children } = $props();
</script>

{@render children()}
