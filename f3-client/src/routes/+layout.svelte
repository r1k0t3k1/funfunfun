<script lang="ts">
  // Carbon Design System を廃止し、Melt UI（ヘッドレス）へ移行した。
  // 配色・共通スタイルはすべて自前のグローバルテーマで定義する。
  import "$lib/ui/theme.css";

  let { children } = $props();
</script>

{@render children()}
