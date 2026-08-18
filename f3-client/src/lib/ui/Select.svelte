<script lang="ts">
  import { createSelect, melt } from "@melt-ui/svelte";
  import { untrack } from "svelte";
  import { writable } from "svelte/store";
  import { fly } from "svelte/transition";
  import Icon from "./Icon.svelte";

  type Option = { value: string; label: string };

  // 旧 Carbon Select 相当。value を bind して選択値を制御する。
  let {
    value = $bindable(""),
    options,
    label = "",
  }: { value?: string; options: Option[]; label?: string } = $props();

  // options は初期化時点のスナップショットで十分（親側で不変に渡す想定）。
  const initial = untrack(
    () => options.find((o) => o.value === value) ?? options[0],
  );
  const selected = writable<Option>(initial);
  let syncing = false;

  // 親 value -> selected ストア
  $effect(() => {
    const match = options.find((o) => o.value === value);
    if (match && !syncing) {
      syncing = true;
      selected.set(match);
      syncing = false;
    }
  });

  // selected ストア -> 親 value
  selected.subscribe((s) => {
    if (s && !syncing && s.value !== value) {
      syncing = true;
      value = s.value;
      syncing = false;
    }
  });

  const {
    elements: { trigger, menu, option, label: labelEl },
    states: { selectedLabel, open },
    helpers: { isSelected },
  } = createSelect<string>({
    selected,
    positioning: { placement: "bottom", sameWidth: true },
    forceVisible: true,
  });
</script>

<div class="select">
  {#if label}
    <span use:melt={$labelEl} class="field-label">{label}</span>
  {/if}
  <button use:melt={$trigger} class="select-trigger" type="button">
    <span>{$selectedLabel || "選択してください"}</span>
    <Icon name="chevron-down" size={16} />
  </button>

  {#if $open}
    <div
      use:melt={$menu}
      class="select-menu"
      transition:fly={{ duration: 120, y: -4 }}
    >
      {#each options as opt}
        <div use:melt={$option({ value: opt.value, label: opt.label })} class="select-option">
          <span>{opt.label}</span>
          {#if $isSelected(opt.value)}
            <Icon name="check" size={16} />
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .select {
    display: block;
  }
  .field-label {
    display: block;
    margin-bottom: 0.35rem;
    font-size: 0.8rem;
    color: var(--text-dim);
  }
</style>
