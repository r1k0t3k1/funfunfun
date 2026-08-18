<script lang="ts">
  import { createDialog, melt } from "@melt-ui/svelte";
  import { writable } from "svelte/store";
  import { fade, scale } from "svelte/transition";
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";

  // 親が open を bind して開閉を制御する。footer スロットにボタン群を渡す。
  let {
    open = $bindable(false),
    title = "",
    children,
    footer,
  }: {
    open?: boolean;
    title?: string;
    children?: Snippet;
    footer?: Snippet;
  } = $props();

  // Melt の controlled ストア。親の boolean prop と双方向で同期する。
  const openStore = writable(open);
  let syncing = false;

  // 親 prop -> ストア
  $effect(() => {
    if (!syncing) {
      syncing = true;
      openStore.set(open);
      syncing = false;
    }
  });

  // ストア -> 親 prop（オーバーレイクリック / Esc / 閉じるボタン経由）
  openStore.subscribe((v) => {
    if (!syncing && v !== open) {
      syncing = true;
      open = v;
      syncing = false;
    }
  });

  const {
    elements: { overlay, content, title: titleEl, close, portalled },
  } = createDialog({ forceVisible: true, open: openStore });
</script>

{#if open}
  <div use:melt={$portalled}>
    <div
      use:melt={$overlay}
      class="dialog-overlay"
      transition:fade={{ duration: 120 }}
    ></div>
    <div class="dialog-wrapper">
      <div
        use:melt={$content}
        class="dialog-content"
        transition:scale={{ duration: 140, start: 0.96 }}
      >
        <header class="dialog-header">
          <h2 use:melt={$titleEl} class="dialog-title">{title}</h2>
          <button use:melt={$close} class="dialog-close" aria-label="閉じる">
            <Icon name="close" size={18} />
          </button>
        </header>

        <div class="dialog-body">
          {@render children?.()}
        </div>

        {#if footer}
          <footer class="dialog-footer">
            {@render footer()}
          </footer>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .dialog-overlay {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: rgba(1, 4, 9, 0.7);
  }

  .dialog-wrapper {
    position: fixed;
    inset: 0;
    z-index: 41;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
    pointer-events: none;
  }

  .dialog-content {
    pointer-events: auto;
    width: 100%;
    max-width: 30rem;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    display: flex;
    flex-direction: column;
    max-height: calc(100vh - 3rem);
  }

  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1.1rem 1.25rem;
    border-bottom: 1px solid var(--border);
  }

  .dialog-title {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 600;
    color: var(--text);
  }

  .dialog-close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: none;
    border-radius: var(--radius);
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
  }
  .dialog-close:hover {
    background: var(--bg-hover);
    color: var(--text);
  }

  .dialog-body {
    padding: 1.25rem;
    overflow-y: auto;
  }

  .dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.6rem;
    padding: 1rem 1.25rem;
    border-top: 1px solid var(--border);
  }
</style>
