<script lang="ts">
  import Logo from "$lib/ui/Logo.svelte";
  import { login } from "$lib/stores/auth";

  let operatorId = $state("");
  let password = $state("");
  let errorMessage = $state("");
  let submitting = $state(false);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    errorMessage = "";
    submitting = true;
    try {
      await login({ operator_id: operatorId, password });
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "ログインに失敗しました";
    } finally {
      submitting = false;
    }
  }
</script>

<div class="login-page">
  <div class="login-card">
    <div class="brand">
      <span class="brand-mark"><Logo size={26} /></span>
      <div>
        <h1 class="login-title">Fun Fun Fun</h1>
        <p class="login-subtitle">サインインして続行してください</p>
      </div>
    </div>

    {#if errorMessage}
      <div class="notification error" role="alert">
        <strong>エラー</strong>
        <span>{errorMessage}</span>
      </div>
    {/if}

    <form onsubmit={handleSubmit}>
      <label class="field">
        <span class="field-label">ユーザー名</span>
        <input
          class="input"
          type="text"
          placeholder="operator_id"
          autocomplete="username"
          bind:value={operatorId}
          required
        />
      </label>
      <label class="field">
        <span class="field-label">パスワード</span>
        <input
          class="input"
          type="password"
          placeholder="password"
          autocomplete="current-password"
          bind:value={password}
          required
        />
      </label>
      <button class="btn btn-primary btn-block" type="submit" disabled={submitting}>
        {submitting ? "サインイン中..." : "サインイン"}
      </button>
    </form>
  </div>
</div>

<style>
  /*
   * 画面いっぱいに広げてカードを上下左右中央へ。
   * 中央寄せは align/justify ではなく カード側の `margin: auto` で行う。
   * flex の align-items:center はコンテナよりコンテンツが高いと上側に
   * はみ出して切れてしまう（スクロールしても届かない）が、auto margin なら
   * 余白が無い時に 0 に潰れるだけなので切れない。
   */
  .login-page {
    min-height: 100vh;
    /* モバイル/Tauri のツールバー分を差し引いた実高さ。未対応環境では上の
       100vh がそのまま使われる。 */
    min-height: 100dvh;
    display: flex;
    padding: 2rem;
    background:
      radial-gradient(
        1200px 600px at 50% -10%,
        rgba(68, 147, 248, 0.1),
        transparent
      ),
      var(--bg);
  }

  .login-card {
    width: 100%;
    max-width: 22rem;
    margin: auto;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 1.75rem;
    box-shadow: var(--shadow-lg);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }

  /*
    ロゴは塗りタイルを敷かず、背景を透過させてマーク単体で見せる。
    Logo.svelte は currentColor で描くので、タイルの色だった --accent を
    そのままマークの色に移して視認性を保つ。
  */
  .brand-mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    background: transparent;
    color: var(--accent);
    flex-shrink: 0;
  }

  .login-title {
    font-size: 1.15rem;
    font-weight: 650;
  }

  .login-subtitle {
    color: var(--text-dim);
    font-size: 0.85rem;
  }

  .field {
    display: block;
    margin-bottom: 1rem;
  }

  .field-label {
    display: block;
    margin-bottom: 0.35rem;
    font-size: 0.8rem;
    color: var(--text-dim);
  }
</style>
