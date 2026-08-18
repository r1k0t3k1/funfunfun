<script lang="ts">
  import {
    SideNav,
    SideNavItems,
    SideNavLink,
    SideNavDivider,
    Content,
  } from "carbon-components-svelte";
  import Network_3 from "carbon-icons-svelte/lib/Network_3.svelte";
  import UserMultiple from "carbon-icons-svelte/lib/UserMultiple.svelte";
  import Logout from "carbon-icons-svelte/lib/Logout.svelte";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { logout } from "$lib/stores/auth";
  import { getAccessToken } from "$lib/api/token";

  let { children } = $props();

  // 未認証（トークン無し）でダッシュボードへ来た場合はログイン画面へ戻す。
  onMount(() => {
    if (getAccessToken() === null) {
      goto("/");
    }
  });

  async function handleLogout(event: Event) {
    // href="/" は「JS が動かなくてもログイン画面へ戻れる」ためのフォールバック。
    // 通常はここでトークン破棄まで行うので既定の遷移は止める。
    event.preventDefault();
    await logout();
  }
</script>

<!--
  レール表示のサイドナビのみでシェルを構成する（上部ヘッダは持たない）。
  - rail        : 通常時は 3rem 幅でアイコンのみ、マウスオーバーで 16rem に開いて
                  ラベルを表示する（Carbon の CSS が担当）。
  - isOpen      : SideNav は isOpen を aria-hidden に直結させているため、
                  レールでも支援技術から見えるよう常に true にしておく。
  - expansionBreakpoint=0
                : 「この幅以上なら自動で展開しハンバーガーを隠す」閾値。
                  ヘッダ（=ハンバーガーの置き場）が無く常にレールで振る舞わせたいので
                  0 にして、狭い幅でもモバイル用ドロワー扱いにならないようにする。
-->
<SideNav rail isOpen expansionBreakpoint={0} ariaLabel="メインナビゲーション">
  <SideNavItems>
    <SideNavLink
      icon={Network_3}
      text="Listener"
      isSelected={$page.url.pathname.startsWith("/dashboard/listeners")}
      on:click={() => goto("/dashboard/listeners")}
    />
    <SideNavLink
      icon={UserMultiple}
      text="Operator"
      isSelected={$page.url.pathname.startsWith("/dashboard/operators")}
      on:click={() => goto("/dashboard/operators")}
    />
    <SideNavDivider />
    <!--
      ヘッダ削除に伴い、ログアウトはサイドナビ最下部へ移動した。
      href を与えるのはキーボード操作（Tab で到達し Enter で実行）のため。
    -->
    <SideNavLink
      icon={Logout}
      text="ログアウト"
      href="/"
      on:click={handleLogout}
    />
  </SideNavItems>
</SideNav>

<Content>
  {@render children()}
</Content>

<style>
  /*
    Carbon の SideNav（--ux バリアント）はヘッダの高さ分 top を下げる。
    ヘッダを廃止したので画面最上部から表示する。
    テーマ側が .bx--side-nav--ux.bx--side-nav--ux で top を指定しているため、
    クラス 3 つ分の詳細度で上書きする。
  */
  :global(.bx--side-nav.bx--side-nav.bx--side-nav--ux) {
    top: 0;
  }

  /*
    Carbon は幅 1056px 未満で「開いたドロワーを閉じるための暗幕」を敷く
    （.bx--side-nav__overlay-active、isOpen=true で有効）。
    レール表示に開閉状態は無く、ヘッダを廃止してハンバーガーも無いため
    暗幕を消す手段が無い。常に無効化する。
  */
  :global(.bx--side-nav__overlay) {
    display: none;
  }

  /* ログアウトはナビゲーションではなく操作なので、リストの末尾に寄せる。 */
  :global(.bx--side-nav__items) {
    display: flex;
    flex-direction: column;
  }

  :global(.bx--side-nav__items > .bx--side-nav__divider) {
    margin-top: auto;
  }
</style>
