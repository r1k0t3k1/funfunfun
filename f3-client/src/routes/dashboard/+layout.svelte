<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import Icon from "$lib/ui/Icon.svelte";
  import { logout } from "$lib/stores/auth";
  import { getAccessToken } from "$lib/api/token";

  let { children } = $props();

  // 未認証（トークン無し）でダッシュボードへ来た場合はログイン画面へ戻す。
  onMount(() => {
    if (getAccessToken() === null) {
      goto("/");
    }
  });

  // 左サイドバーのナビゲーション項目。
  const navItems = [
    { href: "/dashboard/listeners", label: "Listener", icon: "network" },
    { href: "/dashboard/operators", label: "Operator", icon: "users" },
  ] as const;

  async function handleLogout() {
    await logout();
  }
</script>

<div class="app-shell">
  <!-- 左サイドバー：メニューのみ -->
  <aside class="sidebar">
    <div class="sidebar-brand">
      <span class="brand-mark"><Icon name="terminal" size={18} /></span>
      <div class="brand-text">
        <span class="brand-name">f3</span>
        <span class="brand-sub">Console</span>
      </div>
    </div>

    <nav class="sidebar-nav">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-link"
          class:active={$page.url.pathname.startsWith(item.href)}
        >
          <Icon name={item.icon} size={18} />
          <span>{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <button class="nav-link logout" onclick={handleLogout}>
        <Icon name="logout" size={18} />
        <span>ログアウト</span>
      </button>
    </div>
  </aside>

  <!-- 残りの余白はすべてメインコンテンツ -->
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    min-height: 100vh;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    width: 240px;
    flex-shrink: 0;
    background: var(--bg-elev);
    border-right: 1px solid var(--border);
    padding: 1rem 0.75rem;
  }

  .sidebar-brand {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.5rem 0.5rem 1.1rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
  }
  .brand-mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: var(--radius);
    background: var(--accent);
    color: var(--accent-contrast);
    flex-shrink: 0;
  }
  .brand-text {
    display: flex;
    flex-direction: column;
    line-height: 1.2;
  }
  .brand-name {
    font-weight: 700;
    font-size: 1rem;
  }
  .brand-sub {
    font-size: 0.75rem;
    color: var(--text-dim);
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    flex: 1;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.55rem 0.6rem;
    border-radius: var(--radius);
    color: var(--text-dim);
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    border: none;
    background: transparent;
    width: 100%;
    text-align: left;
    font-family: inherit;
  }
  .nav-link:hover {
    background: var(--bg-hover);
    color: var(--text);
  }
  .nav-link.active {
    background: var(--bg-elev-2);
    color: var(--text);
    box-shadow: inset 2px 0 0 var(--accent);
  }

  .sidebar-footer {
    padding-top: 0.5rem;
    border-top: 1px solid var(--border);
  }
  .logout:hover {
    color: var(--danger);
  }

  .content {
    flex: 1;
    min-width: 0;
    padding: 2rem 2.5rem;
    overflow-x: auto;
  }
</style>
