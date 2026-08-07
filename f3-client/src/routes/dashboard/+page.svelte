<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  // ダッシュボードのトップは Listener 画面へリダイレクトする。
  onMount(() => {
    goto("/dashboard/listeners", { replaceState: true });
  });
</script>
