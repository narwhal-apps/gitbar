<script>
  import App from '../components/App.svelte';
  import Login from '../components/login/Login.svelte';
  import { appState } from '$lib/stores/state.svelte';
</script>

{#if appState.isAuthenticated}
  <App />
{:else}
  <Login />
{/if}
