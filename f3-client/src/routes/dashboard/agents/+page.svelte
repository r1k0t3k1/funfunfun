<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "$lib/ui/Icon.svelte";
  import Dialog from "$lib/ui/Dialog.svelte";
  import RowMenu from "$lib/ui/RowMenu.svelte";
  import { listListeners } from "$lib/api/listener";
  import { listAgents } from "$lib/api/agent";
  import type { ListenerListItem, AgentListItem } from "$lib/api/client";

  // ---- Listener タブ ----
  let listeners = $state<ListenerListItem[]>([]);
  let listenersLoading = $state(false);
  let selectedId = $state<string | null>(null);
  let errorMessage = $state("");

  // ---- 選択中 Listener の Agent 一覧 ----
  let agents = $state<AgentListItem[]>([]);
  let agentsLoading = $state(false);

  // Agent 詳細モーダルの状態
  let detailOpen = $state(false);
  let detail = $state<AgentListItem | null>(null);

  // 選択中の Listener オブジェクト。
  const selected = $derived(
    listeners.find((l) => l.id === selectedId) ?? null,
  );

  // Listener 種別バッジ（config から HTTP/HTTPS/TCP/DNS を導く）。
  function listenerProtocol(l: ListenerListItem): string {
    const c = l.config;
    if (c.protocol === "Http") return c.is_ssl ? "HTTPS" : "HTTP";
    return c.protocol.toUpperCase();
  }
  function protocolBadge(protocol: string): string {
    switch (protocol) {
      case "HTTPS":
        return "badge-blue";
      case "HTTP":
        return "badge-teal";
      default:
        return "badge-gray";
    }
  }

  // Agent の短縮 ID（UUID の先頭ブロック）。
  function shortId(id: string): string {
    return id.split("-")[0] ?? id;
  }

  // ドメイン\ユーザー名 の表記（ドメインが空ならユーザー名のみ）。
  function userLabel(a: AgentListItem): string {
    return a.domain_name ? `${a.domain_name}\\${a.user_name}` : a.user_name;
  }

  async function refreshListeners() {
    listenersLoading = true;
    errorMessage = "";
    try {
      listeners = await listListeners();
      // 選択が無効になっていたら先頭のタブを選び直す。
      if (!listeners.some((l) => l.id === selectedId)) {
        selectedId = listeners[0]?.id ?? null;
      }
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "取得に失敗しました";
    } finally {
      listenersLoading = false;
    }
  }

  async function refreshAgents() {
    if (selectedId === null) {
      agents = [];
      return;
    }
    agentsLoading = true;
    try {
      agents = await listAgents(selectedId);
    } catch (e) {
      agents = [];
      errorMessage = e instanceof Error ? e.message : "Agent の取得に失敗しました";
    } finally {
      agentsLoading = false;
    }
  }

  function selectListener(id: string) {
    if (selectedId === id) return;
    selectedId = id;
  }

  // 選択中の Listener（タブ）が変わったら Agent 一覧を取り直す。
  $effect(() => {
    void selectedId;
    refreshAgents();
  });

  function showDetail(agent: AgentListItem) {
    detail = agent;
    detailOpen = true;
  }

  onMount(refreshListeners);
</script>

<div class="page-header">
  <h1>Agent</h1>
  <p>Listener タブを選択して、その Listener に接続中の Agent を表示します。</p>
</div>

{#if errorMessage}
  <div class="notification error" role="alert">
    <strong>エラー</strong>
    <span>{errorMessage}</span>
  </div>
{/if}

<!-- Listener ごとのタブ。クリックでその Listener の Agent 一覧に切り替える。 -->
<div class="tabs" role="tablist">
  {#each listeners as listener (listener.id)}
    {@const proto = listenerProtocol(listener)}
    <button
      class="tab"
      class:active={listener.id === selectedId}
      role="tab"
      aria-selected={listener.id === selectedId}
      onclick={() => selectListener(listener.id)}
    >
      <span class="badge {protocolBadge(proto)}">{proto}</span>
      <span class="tab-name mono">{listener.name}</span>
    </button>
  {/each}
  {#if !listenersLoading && listeners.length === 0}
    <span class="tabs-empty muted">Listener がありません。</span>
  {/if}
</div>

<div class="card">
  <div class="table-toolbar">
    <div class="toolbar-info">
      {#if selected}
        <span class="mono">{selected.name}</span>
        <span class="muted">· {selected.lhost}:{selected.lport}</span>
      {/if}
      <span class="muted">{agents.length} 件</span>
    </div>
    <div class="toolbar-actions">
      <button
        class="btn btn-ghost btn-icon"
        onclick={refreshAgents}
        aria-label="Agent を更新"
        disabled={selectedId === null}
      >
        <Icon name="refresh" size={18} />
      </button>
    </div>
  </div>

  <table class="data-table">
    <thead>
      <tr>
        <th>Agent ID</th>
        <th>ユーザー</th>
        <th>コンピュータ</th>
        <th>OS</th>
        <th>Arch</th>
        <th>権限</th>
        <th>プロセス</th>
        <th class="col-action"></th>
      </tr>
    </thead>
    <tbody>
      {#each agents as agent (agent.id)}
        <tr>
          <td><span class="mono" title={agent.id}>{shortId(agent.id)}</span></td>
          <td>{userLabel(agent)}</td>
          <td>{agent.computer_name || "—"}</td>
          <td>{agent.os || "—"}</td>
          <td><span class="mono">{agent.arch || "—"}</span></td>
          <td>
            <span class="badge {agent.is_admin ? 'badge-red' : 'badge-gray'}">
              {agent.is_admin ? "管理者" : "ユーザー"}
            </span>
          </td>
          <td>
            <span class="mono muted">
              {agent.process_name || "—"}
              {#if agent.process_id}({agent.process_id}){/if}
            </span>
          </td>
          <td class="col-action">
            <RowMenu
              actions={[{ label: "詳細", onSelect: () => showDetail(agent) }]}
            />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if selectedId === null}
    <div class="empty">
      Listener を作成・選択すると、その Listener の Agent 一覧が表示されます。
    </div>
  {:else if agentsLoading}
    <div class="empty">読み込み中...</div>
  {:else if agents.length === 0}
    <div class="empty">この Listener に接続中の Agent はありません。</div>
  {/if}
</div>

<Dialog bind:open={detailOpen} title="Agent 詳細">
  {#if detail}
    <dl class="detail-list">
      <dt>Agent ID</dt>
      <dd class="mono">{detail.id}</dd>
      <dt>Listener ID</dt>
      <dd class="mono">{detail.listener_id}</dd>
      <dt>ユーザー</dt>
      <dd>{userLabel(detail)}</dd>
      <dt>コンピュータ</dt>
      <dd>{detail.computer_name || "—"}</dd>
      <dt>OS</dt>
      <dd>{detail.os || "—"}</dd>
      <dt>アーキテクチャ</dt>
      <dd class="mono">{detail.arch || "—"}</dd>
      <dt>権限</dt>
      <dd>
        <span class="badge {detail.is_admin ? 'badge-red' : 'badge-gray'}">
          {detail.is_admin ? "管理者" : "ユーザー"}
        </span>
      </dd>
      <dt>プロセス</dt>
      <dd class="mono">{detail.process_name || "—"} (PID {detail.process_id})</dd>
      <dt>スレッド ID</dt>
      <dd class="mono">{detail.thread_id}</dd>
    </dl>
  {/if}
</Dialog>

<style>
  .page-header {
    margin-bottom: 1.5rem;
  }
  .page-header h1 {
    margin-bottom: 0.25rem;
  }
  .page-header p {
    color: var(--text-dim);
  }

  /* Listener タブバー */
  .tabs {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.5rem;
  }
  .tab {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.45rem 0.75rem;
    border-radius: var(--radius);
    border: 1px solid transparent;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    font-family: inherit;
    font-size: 0.85rem;
  }
  .tab:hover {
    background: var(--bg-hover);
    color: var(--text);
  }
  .tab.active {
    background: var(--bg-elev-2);
    border-color: var(--border-strong);
    color: var(--text);
    box-shadow: inset 0 -2px 0 var(--accent);
  }
  .tab-name {
    font-weight: 600;
  }
  .tabs-empty {
    padding: 0.45rem 0.25rem;
    font-size: 0.85rem;
  }

  .toolbar-info {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .mono {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
      "Liberation Mono", monospace;
  }
</style>
