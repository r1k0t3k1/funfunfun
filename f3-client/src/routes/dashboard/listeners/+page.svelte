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
    AgentListItem,
  } from "$lib/api/client";

  // ---- Listener 一覧（右カラム） ----
  let listeners = $state<ListenerListItem[]>([]);
  let listenersLoading = $state(false);
  let selectedId = $state<string | null>(null);
  let errorMessage = $state("");

  // ---- Agent 一覧（メイン） ----
  let agents = $state<AgentListItem[]>([]);
  let agentsLoading = $state(false);

  // 選択中の Listener オブジェクト。
  const selected = $derived(
    listeners.find((l) => l.id === selectedId) ?? null,
  );

  // 作成モーダルの状態
  let createOpen = $state(false);
  let submitting = $state(false);
  let formName = $state("");
  let formType = $state<ListenerType>("HTTP");
  let formLhost = $state("0.0.0.0");
  let formLport = $state(8080);

  // Agent 詳細モーダルの状態
  let detailOpen = $state(false);
  let detail = $state<AgentListItem | null>(null);

  const typeOptions = [
    { value: "TCP", label: "TCP" },
    { value: "HTTP", label: "HTTP" },
    { value: "HTTPS", label: "HTTPS" },
  ];

  // Listener 種別バッジの色分け。
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

  // Agent の status（サーバ enum の Debug 文字列）を見やすいラベル＋色に変換。
  function agentStatus(status: string): { label: string; badge: string } {
    switch (status) {
      case "CheckinProcessCompleted":
        return { label: "接続完了", badge: "badge-teal" };
      case "CheckinProcessStarted":
        return { label: "チェックイン中", badge: "badge-blue" };
      default:
        return { label: status, badge: "badge-gray" };
    }
  }

  // 32 バイトの公開鍵（数値配列）を短い 16 進フィンガープリントにする。
  function fingerprint(key: number[]): string {
    const hex = key
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("");
    return `${hex.slice(0, 8)}…${hex.slice(-8)}`;
  }

  // 公開鍵をコロン区切りの完全な 16 進表記にする（詳細ダイアログ用）。
  function fullHex(key: number[]): string {
    return key.map((b) => b.toString(16).padStart(2, "0")).join(":");
  }

  // Agent の短縮 ID（UUID の先頭ブロック）。
  function shortId(id: string): string {
    return id.split("-")[0] ?? id;
  }

  async function refreshListeners() {
    listenersLoading = true;
    errorMessage = "";
    try {
      listeners = await listListeners();
      // 選択が無効になっていたら先頭を選び直す。
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

  // 選択中の Listener が変わったら Agent 一覧を取り直す。
  $effect(() => {
    // selectedId を依存に取り込む。
    void selectedId;
    refreshAgents();
  });

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
      await refreshListeners();
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
      await refreshListeners();
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "操作に失敗しました";
    }
  }

  function showDetail(agent: AgentListItem) {
    detail = agent;
    detailOpen = true;
  }

  onMount(refreshListeners);
</script>

{#if errorMessage}
  <div class="notification error" role="alert">
    <strong>エラー</strong>
    <span>{errorMessage}</span>
  </div>
{/if}

<!--
  Adaptix C2 風の 2 ペイン構成。
    - メイン（左・広い）：選択中 Listener の Agent 一覧
    - 右カラム（狭い）：Listener 一覧（クリックで選択）
-->
<div class="console">
  <!-- メイン：Agent 一覧 -->
  <section class="agents-pane card">
    <header class="pane-header">
      <div class="pane-title">
        <h1>Agents</h1>
        {#if selected}
          <span class="pane-sub">
            <span class="badge {protocolBadge(selected.protocol)}"
              >{selected.protocol}</span
            >
            <span class="mono">{selected.name}</span>
            <span class="muted">· {selected.addr}</span>
          </span>
        {:else}
          <span class="pane-sub muted">Listener を選択してください</span>
        {/if}
      </div>
      <div class="pane-actions">
        <span class="muted">{agents.length} 件</span>
        <button
          class="btn btn-ghost btn-icon"
          onclick={refreshAgents}
          aria-label="Agent を更新"
          disabled={selectedId === null}
        >
          <Icon name="refresh" size={18} />
        </button>
      </div>
    </header>

    <div class="pane-body">
      <table class="data-table">
        <thead>
          <tr>
            <th>Agent ID</th>
            <th>状態</th>
            <th>Session 公開鍵</th>
            <th class="col-action"></th>
          </tr>
        </thead>
        <tbody>
          {#each agents as agent (agent.id)}
            {@const s = agentStatus(agent.status)}
            <tr>
              <td
                ><span class="mono" title={agent.id}>{shortId(agent.id)}</span
                ></td
              >
              <td>
                <span class="badge {s.badge}">{s.label}</span>
              </td>
              <td
                ><span class="mono muted">{fingerprint(agent.session_pubkey)}</span
                ></td
              >
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
          右のカラムから Listener を選択すると Agent 一覧が表示されます。
        </div>
      {:else if agentsLoading}
        <div class="empty">読み込み中...</div>
      {:else if agents.length === 0}
        <div class="empty">この Listener に接続中の Agent はありません。</div>
      {/if}
    </div>
  </section>

  <!-- 右カラム：Listener 一覧 -->
  <aside class="listeners-pane card">
    <header class="pane-header">
      <div class="pane-title">
        <h2>Listeners</h2>
        <span class="pane-sub muted">{listeners.length} 件</span>
      </div>
      <div class="pane-actions">
        <button
          class="btn btn-ghost btn-icon"
          onclick={refreshListeners}
          aria-label="Listener を更新"
        >
          <Icon name="refresh" size={18} />
        </button>
      </div>
    </header>

    <div class="pane-body">
      <ul class="listener-list">
        {#each listeners as listener (listener.id)}
          <li>
            <div
              class="listener-item"
              class:active={listener.id === selectedId}
            >
              <button
                class="listener-select"
                onclick={() => selectListener(listener.id)}
              >
                <span class="listener-name mono">{listener.name}</span>
                <span class="listener-meta">
                  <span class="badge {protocolBadge(listener.protocol)}"
                    >{listener.protocol}</span
                  >
                  <span class="muted">{listener.addr}</span>
                </span>
              </button>
              <RowMenu
                actions={[
                  {
                    label: "起動",
                    onSelect: () => runAction(startListener, listener.id),
                  },
                  {
                    label: "停止",
                    onSelect: () => runAction(stopListener, listener.id),
                  },
                  {
                    label: "削除",
                    danger: true,
                    onSelect: () => runAction(removeListener, listener.id),
                  },
                ]}
              />
            </div>
          </li>
        {/each}
      </ul>

      {#if listenersLoading}
        <div class="empty">読み込み中...</div>
      {:else if listeners.length === 0}
        <div class="empty">Listener がありません。</div>
      {/if}
    </div>

    <footer class="pane-footer">
      <button
        class="btn btn-primary btn-block"
        onclick={() => (createOpen = true)}
      >
        <Icon name="plus" size={18} />
        Listener を作成
      </button>
    </footer>
  </aside>
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

<Dialog bind:open={detailOpen} title="Agent 詳細">
  {#if detail}
    {@const s = agentStatus(detail.status)}
    <dl class="detail-list">
      <dt>Agent ID</dt>
      <dd class="mono">{detail.id}</dd>
      <dt>Listener ID</dt>
      <dd class="mono">{detail.listener_id}</dd>
      <dt>状態</dt>
      <dd><span class="badge {s.badge}">{s.label}</span></dd>
      <dt>Session 公開鍵</dt>
      <dd class="mono wrap">{fullHex(detail.session_pubkey)}</dd>
    </dl>
  {/if}
</Dialog>

<style>
  /* 2 ペインを content 領域の高さいっぱいに広げる（Adaptix C2 風の据え置きレイアウト）。
     dashboard レイアウトの .content は上下 2rem のパディングを持つため差し引く。 */
  .console {
    display: flex;
    gap: 1rem;
    height: calc(100vh - 4rem);
    min-height: 0;
  }

  /* メイン（Agent）は残り幅すべて、右カラム（Listener）は固定幅。 */
  .agents-pane {
    flex: 1;
    min-width: 0;
  }
  .listeners-pane {
    width: 22rem;
    flex-shrink: 0;
  }

  .agents-pane,
  .listeners-pane {
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .pane-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem 0.9rem;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .pane-title {
    display: flex;
    align-items: baseline;
    gap: 0.6rem;
    min-width: 0;
  }
  .pane-title h1 {
    font-size: 1.05rem;
    font-weight: 650;
  }
  .pane-title h2 {
    font-size: 0.95rem;
    font-weight: 650;
  }
  .pane-sub {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.8rem;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .pane-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  /* 中身はペイン内でスクロールさせ、ヘッダ/フッタは固定。 */
  .pane-body {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }

  /* スクロールしてもテーブル見出しを見えるように固定する。 */
  .pane-body :global(.data-table thead th) {
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .pane-footer {
    padding: 0.75rem;
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  /* Listener 一覧（右カラム） */
  .listener-list {
    list-style: none;
    margin: 0;
    padding: 0.4rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .listener-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    border-radius: var(--radius);
    border: 1px solid transparent;
    padding-right: 0.25rem;
  }
  .listener-item:hover {
    background: var(--bg-hover);
  }
  .listener-item.active {
    background: var(--bg-elev-2);
    border-color: var(--border-strong);
    box-shadow: inset 2px 0 0 var(--accent);
  }
  .listener-select {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    padding: 0.55rem 0.6rem;
    background: transparent;
    border: none;
    color: var(--text);
    text-align: left;
    cursor: pointer;
    font-family: inherit;
  }
  .listener-name {
    font-weight: 600;
    font-size: 0.875rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .listener-meta {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.75rem;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  /* 等幅表示（ID・鍵・アドレスなど） */
  .mono {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
      "Liberation Mono", monospace;
  }
  .wrap {
    word-break: break-all;
  }

  .field {
    margin-bottom: 1.1rem;
  }
  .field-label {
    display: block;
    margin-bottom: 0.35rem;
    font-size: 0.8rem;
    color: var(--text-dim);
  }
</style>
