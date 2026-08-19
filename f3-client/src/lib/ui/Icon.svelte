<script lang="ts">
  // Carbon のアイコンパッケージを廃止したため、必要なアイコンだけを
  // インライン SVG で持つ軽量コンポーネント。stroke=currentColor で
  // 親の文字色に追従する。
  type IconName =
    | "network"
    | "users"
    | "plus"
    | "refresh"
    | "dots"
    | "logout"
    | "chevron-down"
    | "close"
    | "check";

  let { name, size = 18 }: { name: IconName; size?: number } = $props();

  // 各アイコンの <path> などの中身。viewBox は 24x24 に統一。
  const paths: Record<IconName, string> = {
    network:
      '<rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/><path d="M6.5 10v4M17.5 10v4M6.5 14h11"/>',
    users:
      '<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75"/>',
    plus: '<path d="M12 5v14M5 12h14"/>',
    refresh:
      '<path d="M21 12a9 9 0 1 1-2.64-6.36M21 3v6h-6"/>',
    dots:
      '<circle cx="12" cy="5" r="1.6"/><circle cx="12" cy="12" r="1.6"/><circle cx="12" cy="19" r="1.6"/>',
    logout:
      '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4M16 17l5-5-5-5M21 12H9"/>',
    "chevron-down": '<path d="M6 9l6 6 6-6"/>',
    close: '<path d="M18 6 6 18M6 6l12 12"/>',
    check: '<path d="M20 6 9 17l-5-5"/>',
  };
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
  focusable="false"
>
  {@html paths[name]}
</svg>
