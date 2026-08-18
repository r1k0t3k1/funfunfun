<script lang="ts">
  import {
    Header,
    HeaderUtilities,
    HeaderAction,
    HeaderPanelLinks,
    HeaderPanelLink,
    SkipToContent,
    SideNav,
    SideNavItems,
    SideNavLink,
    Content,
  } from "carbon-components-svelte";
  import Network_3 from "carbon-icons-svelte/lib/Network_3.svelte";
  import UserMultiple from "carbon-icons-svelte/lib/UserMultiple.svelte";
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

  // メニューバーの表示切り替え（サイドナビの開閉）
  let isSideNavOpen = $state(true);
  let isUserPanelOpen = $state(false);

  async function handleLogout() {
    isUserPanelOpen = false;
    await logout();
  }
</script>

<Header companyName="f3" platformName="Console" bind:isSideNavOpen>
  <svelte:fragment slot="skipToContent">
    <SkipToContent />
  </svelte:fragment>
  <HeaderUtilities>
    <HeaderAction bind:isOpen={isUserPanelOpen}>
      <HeaderPanelLinks>
        <HeaderPanelLink on:click={handleLogout}>ログアウト</HeaderPanelLink>
      </HeaderPanelLinks>
    </HeaderAction>
  </HeaderUtilities>
</Header>

<SideNav bind:isOpen={isSideNavOpen}>
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
  </SideNavItems>
</SideNav>

<Content>
  {@render children()}
</Content>
