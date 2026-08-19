<script lang="ts">
  // f3 のロゴマーク。リング（円）を三角のポインタで切り抜いた形。
  // 色は `--logo` で上書きでき、既定は currentColor。アプリ内はダーク固定
  // テーマなので、置き場所の文字色に追従させるのが一番破綻しない。
  // （OS のライト/ダークに追従する版は static/favicon.svg 側で持つ）
  let { size = 18 }: { size?: number } = $props();

  // 同一ページに複数描画しても mask の id が衝突しないようにする。
  const uid = $props.id();
  const maskId = `logo-cut-${uid}`;
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 512 512"
  aria-hidden="true"
  focusable="false"
>
  <defs>
    <mask id={maskId} maskUnits="userSpaceOnUse" x="0" y="0" width="512" height="512">
      <rect x="-64" y="-64" width="640" height="640" fill="white" />
      <path
        d="M262 150 L301 391.7 L200 474 Z"
        fill="none"
        stroke="black"
        stroke-width="24"
        stroke-linejoin="round"
      />
    </mask>
  </defs>
  <g transform="translate(0,-25.5)">
    <circle
      cx="256"
      cy="228"
      r="124"
      fill="none"
      stroke="var(--logo, currentColor)"
      stroke-width="52"
      mask="url(#{maskId})"
    />
    <path d="M262 150 L301 391.7 L200 474 Z" fill="var(--logo, currentColor)" />
  </g>
</svg>
