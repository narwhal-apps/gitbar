<script>
  import { getAuthContext } from '$lib/stores/contexts';
  import App from '../components/App.svelte';
  import Login from '../components/Login.svelte';

  let authCtx = getAuthContext();
</script>

{#if authCtx.isAuthenticated}
  <App />
{:else}
  <Login />
{/if}
