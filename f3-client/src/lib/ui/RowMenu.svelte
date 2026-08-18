<script lang="ts">
  import { createDropdownMenu, melt } from "@melt-ui/svelte";
  import { fly } from "svelte/transition";
  import Icon from "./Icon.svelte";

  // 行アクション用のドロップダウンメニュー（旧 Carbon OverflowMenu 相当）。
  export type MenuAction = {
    label: string;
    danger?: boolean;
    onSelect: () => void;
  };

  let { actions }: { actions: MenuAction[] } = $props();

  const {
    elements: { trigger, menu, item },
    states: { open },
  } = createDropdownMenu({
    positioning: { placement: "bottom-end" },
    loop: true,
  });
</script>

<button
  use:melt={$trigger}
  class="btn btn-ghost btn-icon"
  aria-label="操作メニュー"
>
  <Icon name="dots" size={18} />
</button>

{#if $open}
  <div
    use:melt={$menu}
    class="menu"
    transition:fly={{ duration: 120, y: -4 }}
  >
    {#each actions as action}
      <button
        use:melt={$item}
        class="menu-item"
        class:danger={action.danger}
        onclick={action.onSelect}
      >
        {action.label}
      </button>
    {/each}
  </div>
{/if}
