<script>
  import App from '../components/App.svelte';
  import Login from '../components/login/Login.svelte';
  import { appState } from '$lib/appState.svelte';
</script>

{#if appState.isLoggedIn}
  <App />
{:else}
  <Login />
{/if}
