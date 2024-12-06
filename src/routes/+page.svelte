<script>
  import App from '../components/App.svelte';
  import Login from '../components/login/Login.svelte';
  import { appState } from '$lib/appState.svelte';
  import { onMount } from 'svelte';

  onMount(() => {
    appState.fetchReviews();
  });
</script>

{#if appState.isAuthenticated}
  <App />
{:else}
  <Login />
{/if}
