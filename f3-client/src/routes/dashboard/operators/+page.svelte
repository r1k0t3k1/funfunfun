<script lang="ts">
  import { onMount } from "svelte";
  import {
    DataTable,
    Toolbar,
    ToolbarContent,
    Button,
    Modal,
    OverflowMenu,
    OverflowMenuItem,
    InlineNotification,
    InlineLoading,
    Tag,
    StructuredList,
    StructuredListBody,
    StructuredListRow,
    StructuredListCell,
  } from "carbon-components-svelte";
  import Renew from "carbon-icons-svelte/lib/Renew.svelte";
  import { listOperators, getOperator } from "$lib/api/operator";
  import type { OperatorResponse, OperatorRole } from "$lib/api/client";

  // OperatorResponse は id を持つため、そのまま DataTable の行として扱える。
  type Row = OperatorResponse;

  let rows = $state<Row[]>([]);
  let loading = $state(false);
  let errorMessage = $state("");

  // 詳細モーダルの状態
  let detailOpen = $state(false);
  let detailLoading = $state(false);
  let detail = $state<OperatorResponse | null>(null);

  const headers = [
    { key: "name", value: "名前" },
    { key: "role", value: "ロール" },
    { key: "description", value: "説明" },
    { key: "overflow", empty: true },
  ] as const;

  // ロールごとに色を割り当てる（Admin=赤系, Write=青系, Read=グレー）。
  function roleTagType(role: OperatorRole): "red" | "blue" | "cool-gray" {
    switch (role) {
      case "Admin":
        return "red";
      case "Write":
        return "blue";
      default:
        return "cool-gray";
    }
  }

  async function refresh() {
    loading = true;
    errorMessage = "";
    try {
      rows = await listOperators();
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "取得に失敗しました";
    } finally {
      loading = false;
    }
  }

  // /operator/get を叩いて最新の詳細を取得して表示する。
  async function showDetail(id: string) {
    detailOpen = true;
    detailLoading = true;
    detail = null;
    errorMessage = "";
    try {
      detail = await getOperator(id);
    } catch (e) {
      detailOpen = false;
      errorMessage = e instanceof Error ? e.message : "詳細の取得に失敗しました";
    } finally {
      detailLoading = false;
    }
  }

  onMount(refresh);
</script>

<div class="operators">
  <div class="page-header">
    <h1>Operator</h1>
    <p>サーバに登録されているオペレータの一覧と詳細を確認します。</p>
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
      </ToolbarContent>
    </Toolbar>

    <svelte:fragment slot="cell" let:row let:cell>
      {#if cell.key === "overflow"}
        <OverflowMenu flipped>
          <OverflowMenuItem text="詳細" on:click={() => showDetail(row.id)} />
        </OverflowMenu>
      {:else if cell.key === "name"}
        <Tag type="cool-gray">{cell.value}</Tag>
      {:else if cell.key === "role"}
        <Tag type={roleTagType(cell.value as OperatorRole)}>{cell.value}</Tag>
      {:else if cell.key === "description"}
        {cell.value || "—"}
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>
  </DataTable>

  {#if loading}
    <InlineLoading description="読み込み中..." />
  {:else if rows.length === 0}
    <p class="empty">オペレータが登録されていません。</p>
  {/if}
</div>

<Modal
  bind:open={detailOpen}
  passiveModal
  modalHeading="オペレータ詳細"
  on:close={() => (detail = null)}
>
  {#if detailLoading}
    <InlineLoading description="読み込み中..." />
  {:else if detail}
    <StructuredList>
      <StructuredListBody>
        <StructuredListRow>
          <StructuredListCell head>ID</StructuredListCell>
          <StructuredListCell>{detail.id}</StructuredListCell>
        </StructuredListRow>
        <StructuredListRow>
          <StructuredListCell head>名前</StructuredListCell>
          <StructuredListCell>{detail.name}</StructuredListCell>
        </StructuredListRow>
        <StructuredListRow>
          <StructuredListCell head>ロール</StructuredListCell>
          <StructuredListCell>
            <Tag type={roleTagType(detail.role)}>{detail.role}</Tag>
          </StructuredListCell>
        </StructuredListRow>
        <StructuredListRow>
          <StructuredListCell head>説明</StructuredListCell>
          <StructuredListCell>{detail.description || "—"}</StructuredListCell>
        </StructuredListRow>
      </StructuredListBody>
    </StructuredList>
  {/if}
</Modal>

<style>
  .page-header {
    margin-bottom: 1.5rem;
  }

  .page-header h1 {
    margin-bottom: 0.25rem;
  }

  .page-header p {
    color: var(--cds-text-secondary, #c6c6c6);
  }

  .empty {
    margin-top: 1rem;
    color: var(--cds-text-secondary, #c6c6c6);
  }
</style>
