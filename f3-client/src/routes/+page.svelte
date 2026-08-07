<script lang="ts">
  import {
    Form,
    TextInput,
    PasswordInput,
    Button,
    InlineNotification,
    Tile,
  } from "carbon-components-svelte";
  import { login } from "$lib/stores/auth";

  let username = $state("");
  let password = $state("");
  let errorMessage = $state("");
  let submitting = $state(false);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    errorMessage = "";
    submitting = true;
    try {
      await login({ username, password });
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "ログインに失敗しました";
    } finally {
      submitting = false;
    }
  }
</script>

<div class="login-page">
  <Tile class="login-card">
    <h2 class="login-title">f3 コンソール</h2>
    <p class="login-subtitle">サインインして続行してください</p>

    {#if errorMessage}
      <InlineNotification
        kind="error"
        title="エラー"
        subtitle={errorMessage}
        lowContrast
        hideCloseButton
      />
    {/if}

    <Form on:submit={handleSubmit}>
      <div class="field">
        <TextInput
          labelText="ユーザー名"
          placeholder="username"
          bind:value={username}
          required
        />
      </div>
      <div class="field">
        <PasswordInput
          labelText="パスワード"
          placeholder="password"
          bind:value={password}
          required
        />
      </div>
      <Button type="submit" disabled={submitting}>
        {submitting ? "サインイン中..." : "サインイン"}
      </Button>
    </Form>
  </Tile>
</div>

<style>
  .login-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  :global(.login-card) {
    width: 100%;
    max-width: 24rem;
  }

  .login-title {
    margin-bottom: 0.25rem;
  }

  .login-subtitle {
    margin-bottom: 1.5rem;
    color: var(--cds-text-secondary, #525252);
  }

  .field {
    margin-bottom: 1.5rem;
  }
</style>
