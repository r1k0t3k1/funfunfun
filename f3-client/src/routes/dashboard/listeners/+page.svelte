<script lang="ts">
  import { onMount } from "svelte";
  import {
    DataTable,
    Toolbar,
    ToolbarContent,
    Button,
    Modal,
    TextInput,
    NumberInput,
    Select,
    SelectItem,
    OverflowMenu,
    OverflowMenuItem,
    InlineNotification,
    InlineLoading,
    Tag,
  } from "carbon-components-svelte";
  import Add from "carbon-icons-svelte/lib/Add.svelte";
  import Renew from "carbon-icons-svelte/lib/Renew.svelte";
  import {
    listListeners,
    createListener,
    startListener,
    stopListener,
    removeListener,
  } from "$lib/api/listener";
  import type { ListenerListItem, ListenerType } from "$lib/api/client";

  type Row = ListenerListItem & { id: string };

  let rows = $state<Row[]>([]);
  let loading = $state(false);
  let errorMessage = $state("");

  // 作成モーダルの状態
  let createOpen = $state(false);
  let submitting = $state(false);
  let formName = $state("");
  let formType = $state<ListenerType>("HTTP");
  let formLhost = $state("0.0.0.0");
  let formLport = $state(8080);

  const headers = [
    { key: "name", value: "名前" },
    { key: "addr", value: "アドレス" },
    { key: "overflow", empty: true },
  ] as const;

  async function refresh() {
    loading = true;
    errorMessage = "";
    try {
      const items = await listListeners();
      // 一覧にはIDが無いため name を識別子として扱う。
      rows = items.map((i) => ({ ...i, id: i.name }));
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "取得に失敗しました";
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    formName = "";
    formType = "HTTP";
    formLhost = "0.0.0.0";
    formLport = 8080;
  }

  async function handleCreate() {
    submitting = true;
    errorMessage = "";
    try {
      await createListener({
        name: formName,
        listener_type: formType,
        lhost: formLhost,
        lport: formLport,
      });
      createOpen = false;
      resetForm();
      await refresh();
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "作成に失敗しました";
    } finally {
      submitting = false;
    }
  }

  async function runAction(action: (id: string) => Promise<void>, id: string) {
    errorMessage = "";
    try {
      await action(id);
      await refresh();
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "操作に失敗しました";
    }
  }

  onMount(refresh);
</script>

<div class="listeners">
  <div class="page-header">
    <h1>Listener</h1>
    <p>リスナーの作成・起動・停止・削除を行います。</p>
  </div>

  {#if errorMessage}
    <InlineNotification
      kind="error"
      title="エラー"
      subtitle={errorMessage}
      lowContrast
      on:close={() => (errorMessage = "")}
    />
  {/if}

  <DataTable {headers} {rows}>
    <Toolbar>
      <ToolbarContent>
        <Button
          kind="ghost"
          icon={Renew}
          iconDescription="更新"
          on:click={refresh}
        />
        <Button icon={Add} on:click={() => (createOpen = true)}>
          Listener を作成
        </Button>
      </ToolbarContent>
    </Toolbar>

    <svelte:fragment slot="cell" let:row let:cell>
      {#if cell.key === "overflow"}
        <OverflowMenu flipped>
          <OverflowMenuItem
            text="起動"
            on:click={() => runAction(startListener, row.id)}
          />
          <OverflowMenuItem
            text="停止"
            on:click={() => runAction(stopListener, row.id)}
          />
          <OverflowMenuItem
            danger
            text="削除"
            on:click={() => runAction(removeListener, row.id)}
          />
        </OverflowMenu>
      {:else if cell.key === "name"}
        <Tag type="cool-gray">{cell.value}</Tag>
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>
  </DataTable>

  {#if loading}
    <InlineLoading description="読み込み中..." />
  {:else if rows.length === 0}
    <p class="empty">Listener がありません。「Listener を作成」から追加してください。</p>
  {/if}
</div>

<Modal
  bind:open={createOpen}
  modalHeading="Listener を作成"
  primaryButtonText={submitting ? "作成中..." : "作成"}
  secondaryButtonText="キャンセル"
  primaryButtonDisabled={submitting || formName.trim() === ""}
  on:click:button--secondary={() => (createOpen = false)}
  on:submit={handleCreate}
  on:close={resetForm}
>
  <div class="form-field">
    <TextInput labelText="名前" placeholder="my-listener" bind:value={formName} required />
  </div>
  <div class="form-field">
    <Select labelText="種別" bind:selected={formType}>
      <SelectItem value="TCP" text="TCP" />
      <SelectItem value="HTTP" text="HTTP" />
      <SelectItem value="HTTPS" text="HTTPS" />
    </Select>
  </div>
  <div class="form-field">
    <TextInput labelText="LHOST" placeholder="0.0.0.0" bind:value={formLhost} required />
  </div>
  <div class="form-field">
    <NumberInput labelText="LPORT" min={0} max={65535} bind:value={formLport} />
  </div>
</Modal>

<style>
  .page-header {
    margin-bottom: 1.5rem;
  }

  .page-header h1 {
    margin-bottom: 0.25rem;
  }

  .page-header p {
    color: var(--cds-text-secondary, #525252);
  }

  .form-field {
    margin-bottom: 1.5rem;
  }

  .empty {
    margin-top: 1rem;
    color: var(--cds-text-secondary, #525252);
  }
</style>
