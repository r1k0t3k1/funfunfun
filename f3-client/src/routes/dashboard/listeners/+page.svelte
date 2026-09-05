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
    CreateListenerRequest,
  } from "$lib/api/client";

  type Row = ListenerListItem;

  let rows = $state<Row[]>([]);
  let loading = $state(false);
  let errorMessage = $state("");

  // Listener ID → 接続中の Agent 数。
  // 一覧に Agent 数の集計エンドポイントが無いため、Listener ごとに
  // /agent/list を引いて件数を数える。取得に失敗した Listener は
  // エントリを持たず、表示上は「—」（不明）になる。
  let agentCounts = $state<Record<string, number>>({});

  // 作成モーダルの状態
  let createOpen = $state(false);
  let submitting = $state(false);
  let formName = $state("");
  let formType = $state<ListenerType>("HTTP");
  let formLhost = $state("0.0.0.0");
  let formLport = $state(8080);

  // HTTP/HTTPS 選択時のみ使う config（protocol=Http）の詳細設定。
  // TCP/DNS では送信されない（protocol ごとに config の構造が変わるため）。
  let formHttpPath = $state("/");
  let formHttpMethod = $state("GET");
  let formHttpHostHeader = $state("");
  let formHttpUserAgent = $state(
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
  );

  const typeOptions = [
    { value: "TCP", label: "TCP" },
    { value: "HTTP", label: "HTTP" },
    { value: "HTTPS", label: "HTTPS" },
    { value: "DNS", label: "DNS" },
  ];

  // Http 系（path 等の詳細設定を持つ protocol）かどうか。
  const isHttp = $derived(formType === "HTTP" || formType === "HTTPS");

  // ListenerResponse.config（protocol で判別されるユニオン）から
  // 表示用の種別ラベルを求める。Http は is_ssl で HTTP/HTTPS を出し分ける。
  function displayProtocol(row: Row): string {
    const c = row.config;
    if (c.protocol === "Http") return c.is_ssl ? "HTTPS" : "HTTP";
    return c.protocol.toUpperCase();
  }

  // lhost:lport をアドレス表記にする（旧 `addr` フィールドの代替）。
  function displayAddr(row: Row): string {
    return `${row.lhost}:${row.lport}`;
  }

  // 起動状態バッジ。起動中は緑、停止中はグレー。
  function statusBadge(running: boolean): string {
    return running ? "badge-green" : "badge-gray";
  }

  // Listener 種別バッジの色分け。
  function protocolBadge(protocol: string): string {
    switch (protocol) {
      case "HTTPS":
        return "badge-blue";
      case "HTTP":
        return "badge-teal";
      case "DNS":
        return "badge-purple";
      default:
        return "badge-gray";
    }
  }

  // UI の種別（TCP/HTTP/HTTPS/DNS）をサーバの config へ変換する。
  // protocol ごとに config の構造が変わる:
  //   - Http/Https: path/user_agent/host_header/http_method/is_ssl を送る
  //   - Tcp/Dns: protocol のみ
  function toConfig(type: ListenerType): CreateListenerRequest["config"] {
    if (type === "TCP") return { protocol: "Tcp" };
    if (type === "DNS") return { protocol: "Dns" };
    return {
      protocol: "Http",
      path: formHttpPath,
      user_agent: formHttpUserAgent,
      host_header: formHttpHostHeader,
      http_method: formHttpMethod,
      is_ssl: type === "HTTPS",
    };
  }

  async function refresh() {
    loading = true;
    errorMessage = "";
    try {
      rows = await listListeners();
      await refreshAgentCounts();
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "取得に失敗しました";
    } finally {
      loading = false;
    }
  }

  // 各 Listener の接続中 Agent 数を並列で数え直す。
  // 一部の Listener で取得に失敗しても他の集計は反映する（失敗分は「—」表示）。
  async function refreshAgentCounts() {
    const entries = await Promise.all(
      rows.map(async (row) => {
        try {
          const agents = await listAgents(row.id);
          return [row.id, agents.length] as const;
        } catch {
          return null;
        }
      }),
    );
    agentCounts = Object.fromEntries(
      entries.filter((e): e is readonly [string, number] => e !== null),
    );
  }

  function resetForm() {
    formName = "";
    formType = "HTTP";
    formLhost = "0.0.0.0";
    formLport = 8080;
    formHttpPath = "/";
    formHttpMethod = "GET";
    formHttpHostHeader = "";
    formHttpUserAgent =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
  }

  async function handleCreate(event: SubmitEvent) {
    event.preventDefault();
    submitting = true;
    errorMessage = "";
    try {
      await createListener({
        name: formName,
        lhost: formLhost,
        lport: formLport,
        config: toConfig(formType),
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

<div class="page-header">
  <h1>Listener</h1>
  <p>リスナーの作成・起動・停止・削除を行います。</p>
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
        <th>状態</th>
        <th>Agent</th>
        <th class="col-action"></th>
      </tr>
    </thead>
    <tbody>
      {#each rows as row (row.id)}
        {@const proto = displayProtocol(row)}
        <tr>
          <td><span class="mono">{row.name}</span></td>
          <td>
            <span class="badge {protocolBadge(proto)}">{proto}</span>
          </td>
          <td><span class="mono muted">{displayAddr(row)}</span></td>
          <td>
            <span class="badge {statusBadge(row.is_running)}">
              {row.is_running ? "起動中" : "停止中"}
            </span>
          </td>
          <td>
            <span class="mono">{agentCounts[row.id] ?? "—"}</span>
          </td>
          <td class="col-action">
            <RowMenu
              actions={[
                { label: "起動", onSelect: () => runAction(startListener, row.id) },
                { label: "停止", onSelect: () => runAction(stopListener, row.id) },
                {
                  label: "削除",
                  danger: true,
                  onSelect: () => runAction(removeListener, row.id),
                },
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

    {#if isHttp}
      <!-- protocol=Http のときだけ現れる詳細設定。TCP/DNS では送信されない。 -->
      <label class="field">
        <span class="field-label">パス</span>
        <input class="input" placeholder="/" bind:value={formHttpPath} required />
      </label>
      <label class="field">
        <span class="field-label">HTTP メソッド</span>
        <input class="input" placeholder="GET" bind:value={formHttpMethod} required />
      </label>
      <label class="field">
        <span class="field-label">Host ヘッダ</span>
        <input
          class="input"
          placeholder="example.com（任意）"
          bind:value={formHttpHostHeader}
        />
      </label>
      <label class="field">
        <span class="field-label">User-Agent</span>
        <input class="input" bind:value={formHttpUserAgent} required />
      </label>
    {/if}
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

  .mono {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
      "Liberation Mono", monospace;
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
