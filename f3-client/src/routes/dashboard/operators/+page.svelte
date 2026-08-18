<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Dialog from "$lib/ui/Dialog.svelte";
  import RowMenu from "$lib/ui/RowMenu.svelte";
  import { listOperators, getOperator } from "$lib/api/operator";
  import type { OperatorResponse, OperatorRole } from "$lib/api/client";

  type Row = OperatorResponse;

  let rows = $state<Row[]>([]);
  let loading = $state(false);
  let errorMessage = $state("");

  // 詳細モーダルの状態
  let detailOpen = $state(false);
  let detailLoading = $state(false);
  let detail = $state<OperatorResponse | null>(null);

  // ロールごとにバッジ色を割り当てる（Admin=赤系, Write=青系, Read=グレー）。
  function roleBadge(role: OperatorRole): string {
    switch (role) {
      case "Admin":
        return "badge-red";
      case "Write":
        return "badge-blue";
      default:
        return "badge-gray";
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

<div class="page-header">
  <h1>Operator</h1>
  <p>サーバに登録されているオペレータの一覧と詳細を確認します。</p>
</div>

{#if errorMessage}
  <div class="notification error" role="alert">
    <strong>エラー</strong>
    <span>{errorMessage}</span>
  </div>
{/if}

<div class="card">
  <div class="table-toolbar">
    <span class="muted">{rows.length} 件</span>
    <div class="toolbar-actions">
      <button class="btn btn-ghost btn-icon" onclick={refresh} aria-label="更新">
        <Icon name="refresh" size={18} />
      </button>
    </div>
  </div>

  <table class="data-table">
    <thead>
      <tr>
        <th>名前</th>
        <th>ロール</th>
        <th>説明</th>
        <th class="col-action"></th>
      </tr>
    </thead>
    <tbody>
      {#each rows as row (row.id)}
        <tr>
          <td><span class="badge badge-gray">{row.name}</span></td>
          <td><span class="badge {roleBadge(row.role)}">{row.role}</span></td>
          <td>{row.description || "—"}</td>
          <td class="col-action">
            <RowMenu actions={[{ label: "詳細", onSelect: () => showDetail(row.id) }]} />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if loading}
    <div class="empty">読み込み中...</div>
  {:else if rows.length === 0}
    <div class="empty">オペレータが登録されていません。</div>
  {/if}
</div>

<Dialog bind:open={detailOpen} title="オペレータ詳細">
  {#if detailLoading}
    <div class="empty">読み込み中...</div>
  {:else if detail}
    <dl class="detail-list">
      <dt>ID</dt>
      <dd>{detail.id}</dd>
      <dt>名前</dt>
      <dd>{detail.name}</dd>
      <dt>ロール</dt>
      <dd><span class="badge {roleBadge(detail.role)}">{detail.role}</span></dd>
      <dt>説明</dt>
      <dd>{detail.description || "—"}</dd>
    </dl>
  {/if}
</Dialog>
