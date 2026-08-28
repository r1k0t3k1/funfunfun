<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Dialog from "$lib/ui/Dialog.svelte";
  import Select from "$lib/ui/Select.svelte";
  import RowMenu from "$lib/ui/RowMenu.svelte";
  import {
    listListeners,
    createListener,
    startListener,
    stopListener,
    removeListener,
  } from "$lib/api/listener";
  import { listAgents } from "$lib/api/agent";
  import type {
    ListenerListItem,
    ListenerType,
    AgentResponse,
  } from "$lib/api/client";

  type Row = ListenerListItem;

  let rows = $state<Row[]>([]);
  let loading = $state(false);
  let errorMessage = $state("");

  // 接続エージェント一覧モーダルの状態。
  // Agent は Listener に紐づくため、選択した Listener の接続エージェントを表示する。
  let agentsOpen = $state(false);
  let agentsLoading = $state(false);
  let agentsError = $state("");
  let agentRows = $state<AgentResponse[]>([]);
  let agentsListener = $state<Row | null>(null);

  // 作成モーダルの状態
  let createOpen = $state(false);
  let submitting = $state(false);
  let formName = $state("");
  let formType = $state<ListenerType>("HTTP");
  let formLhost = $state("0.0.0.0");
  let formLport = $state(8080);

  const typeOptions = [
    { value: "TCP", label: "TCP" },
    { value: "HTTP", label: "HTTP" },
    { value: "HTTPS", label: "HTTPS" },
  ];

  async function refresh() {
    loading = true;
    errorMessage = "";
    try {
      rows = await listListeners();
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

  async function handleCreate(event: SubmitEvent) {
    event.preventDefault();
    submitting = true;
    errorMessage = "";
    try {
      await createListener({
        name: formName,
        protocol: formType,
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

  // 選択した Listener に接続中のエージェント一覧を取得して表示する。
  async function showAgents(listener: Row) {
    agentsListener = listener;
    agentsOpen = true;
    await loadAgents();
  }

  async function loadAgents() {
    if (agentsListener === null) return;
    agentsLoading = true;
    agentsError = "";
    try {
      agentRows = await listAgents(agentsListener.id);
    } catch (e) {
      agentRows = [];
      agentsError = e instanceof Error ? e.message : "取得に失敗しました";
    } finally {
      agentsLoading = false;
    }
  }

  // session_pubkey（バイト配列）を先頭数バイトの 16 進フィンガープリントにする。
  // shared_secret は機密のため一切表示しない。
  function pubkeyFingerprint(bytes: number[]): string {
    if (!bytes || bytes.length === 0) return "—";
    const head = bytes
      .slice(0, 8)
      .map((b) => (b & 0xff).toString(16).padStart(2, "0"))
      .join(":");
    return bytes.length > 8 ? `${head}…` : head;
  }

  // 状態文字列に応じてバッジ色を割り当てる（接続中は緑系、それ以外はグレー）。
  function statusBadge(status: string): string {
    const s = status.toLowerCase();
    if (s.includes("active") || s.includes("connect") || s.includes("online")) {
      return "badge-teal";
    }
    return "badge-gray";
  }

  onMount(refresh);
</script>

<div class="page-header">
  <h1>Listener</h1>
  <p>
    リスナーの作成・起動・停止・削除を行います。行メニューの「接続エージェント」から、そのリスナーに接続中のエージェント一覧を確認できます。
  </p>
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
      <button class="btn btn-primary" onclick={() => (createOpen = true)}>
        <Icon name="plus" size={18} />
        Listener を作成
      </button>
    </div>
  </div>

  <table class="data-table">
    <thead>
      <tr>
        <th>名前</th>
        <th>種別</th>
        <th>アドレス</th>
        <th class="col-action"></th>
      </tr>
    </thead>
    <tbody>
      {#each rows as row (row.id)}
        <tr>
          <td><span class="badge badge-gray">{row.name}</span></td>
          <td><span class="badge badge-teal">{row.protocol}</span></td>
          <td>{row.addr}</td>
          <td class="col-action">
            <RowMenu
              actions={[
                { label: "接続エージェント", onSelect: () => showAgents(row) },
                { label: "起動", onSelect: () => runAction(startListener, row.id) },
                { label: "停止", onSelect: () => runAction(stopListener, row.id) },
                { label: "削除", danger: true, onSelect: () => runAction(removeListener, row.id) },
              ]}
            />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if loading}
    <div class="empty">読み込み中...</div>
  {:else if rows.length === 0}
    <div class="empty">
      Listener がありません。「Listener を作成」から追加してください。
    </div>
  {/if}
</div>

<Dialog bind:open={createOpen} title="Listener を作成">
  <form id="create-listener-form" onsubmit={handleCreate}>
    <label class="field">
      <span class="field-label">名前</span>
      <input class="input" placeholder="my-listener" bind:value={formName} required />
    </label>
    <div class="field">
      <Select label="種別" options={typeOptions} bind:value={formType} />
    </div>
    <label class="field">
      <span class="field-label">LHOST</span>
      <input class="input" placeholder="0.0.0.0" bind:value={formLhost} required />
    </label>
    <label class="field">
      <span class="field-label">LPORT</span>
      <input
        class="input"
        type="number"
        min="0"
        max="65535"
        bind:value={formLport}
      />
    </label>
  </form>

  {#snippet footer()}
    <button class="btn" onclick={() => (createOpen = false)}>キャンセル</button>
    <button
      class="btn btn-primary"
      type="submit"
      form="create-listener-form"
      disabled={submitting || formName.trim() === ""}
    >
      {submitting ? "作成中..." : "作成"}
    </button>
  {/snippet}
</Dialog>

<Dialog
  bind:open={agentsOpen}
  title={agentsListener
    ? `接続エージェント — ${agentsListener.name}`
    : "接続エージェント"}
>
  {#if agentsError}
    <div class="notification error" role="alert">
      <strong>エラー</strong>
      <span>{agentsError}</span>
    </div>
  {/if}

  <div class="table-toolbar">
    <span class="muted">{agentRows.length} 台接続中</span>
    <button
      class="btn btn-ghost btn-icon"
      onclick={loadAgents}
      aria-label="更新"
    >
      <Icon name="refresh" size={18} />
    </button>
  </div>

  {#if agentsLoading}
    <div class="empty">読み込み中...</div>
  {:else if agentRows.length === 0}
    <div class="empty">接続中のエージェントはありません。</div>
  {:else}
    <table class="data-table">
      <thead>
        <tr>
          <th>エージェント ID</th>
          <th>状態</th>
          <th>公開鍵</th>
        </tr>
      </thead>
      <tbody>
        {#each agentRows as agent (agent.id)}
          <tr>
            <td><code class="mono">{agent.id}</code></td>
            <td>
              <span class="badge {statusBadge(agent.status)}">
                {agent.status}
              </span>
            </td>
            <td><code class="mono">{pubkeyFingerprint(agent.session_pubkey)}</code></td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</Dialog>

<style>
  .field {
    margin-bottom: 1.1rem;
  }
  .field-label {
    display: block;
    margin-bottom: 0.35rem;
    font-size: 0.8rem;
    color: var(--text-dim);
  }
  /* エージェント ID / 公開鍵フィンガープリントは等幅で表示して視認性を上げる。 */
  .mono {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 0.82rem;
    color: var(--text-dim);
  }
</style>
