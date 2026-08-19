<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import Icon from "$lib/ui/Icon.svelte";
  import Logo from "$lib/ui/Logo.svelte";
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
  <!--
    左サイドバー：既定はアイコンのみの細い状態で、ホバー（またはキーボード
    フォーカス）で展開してラベルを表示する。展開はコンテンツの上に覆い被さる
    オーバーレイなので、テーブルの列幅が hover 毎にガタつくことがない。
  -->
  <aside class="sidebar">
    <div class="sidebar-brand">
      <span class="brand-mark"><Logo size={20} /></span>
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
          title={item.label}
        >
          <span class="nav-icon"><Icon name={item.icon} size={18} /></span>
          <span class="nav-label">{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <button
        class="nav-link logout"
        onclick={handleLogout}
        title="ログアウト"
        aria-label="ログアウト"
      >
        <span class="nav-icon"><Icon name="logout" size={18} /></span>
        <span class="nav-label">ログアウト</span>
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
    /* 折りたたみ時 / 展開時の幅。content 側のオフセットにも使う。 */
    --sidebar-w: 4rem;
    --sidebar-w-open: 15rem;

    display: flex;
    min-height: 100vh;
  }

  .sidebar {
    /* fixed + content 側の margin-left で「覆い被さる」展開にする。
       flex アイテムのままだと展開のたびにメイン領域が縮んで再レイアウトされる。 */
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    z-index: 40;

    display: flex;
    flex-direction: column;
    width: var(--sidebar-w);
    /* 折りたたみ時にラベルが外へはみ出すのを隠す */
    overflow: hidden;
    background: var(--bg-elev);
    border-right: 1px solid var(--border);
    padding: 1rem 0.75rem;
    transition:
      width 0.16s ease,
      box-shadow 0.16s ease;
  }

  /* :focus-within も条件に入れる。Tab でメニューを辿った時にもラベルが
     見えないと、キーボード操作ではどの項目か判別できなくなる。 */
  .sidebar:hover,
  .sidebar:focus-within {
    width: var(--sidebar-w-open);
    box-shadow: var(--shadow-lg);
  }

  /* 折りたたみ時はアイコンを中央に、展開時は左寄せ＋ラベル表示。 */
  .nav-icon {
    display: inline-flex;
    align-items: center;
    flex-shrink: 0;
  }

  .nav-label {
    display: none;
    white-space: nowrap;
  }
  .sidebar:hover .nav-label,
  .sidebar:focus-within .nav-label {
    display: inline;
  }

  @media (prefers-reduced-motion: reduce) {
    .sidebar {
      transition: none;
    }
  }

  .sidebar-brand {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.6rem;
    padding: 0.5rem 0.5rem 1.1rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
  }
  .sidebar:hover .sidebar-brand,
  .sidebar:focus-within .sidebar-brand {
    justify-content: flex-start;
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
    display: none;
    flex-direction: column;
    line-height: 1.2;
  }
  .sidebar:hover .brand-text,
  .sidebar:focus-within .brand-text {
    display: flex;
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
    /* 折りたたみ時はアイコンだけを中央に置く。ラベルは display:none で
       レイアウトから外れるので、これで正確に中央へ来る。 */
    justify-content: center;
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
  .sidebar:hover .nav-link,
  .sidebar:focus-within .nav-link {
    justify-content: flex-start;
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
    /* サイドバーは position:fixed なので、折りたたみ幅の分だけ自前でずらす。
       展開してもこの値は変わらない = メイン領域は動かない。 */
    margin-left: var(--sidebar-w);
    padding: 2rem 2.5rem;
    overflow-x: auto;
  }
</style>
