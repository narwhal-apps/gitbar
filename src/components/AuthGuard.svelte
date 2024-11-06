<script>
  import Login from './Login.svelte';
  import { auth } from '../lib/auth';
  

  let { children } = $props();
</script>

{#if $auth.account}
  {@render children?.()}
{:else}
  <!-- user is a guest. Show a login widget instead of the current page. -->
  <Login />
{/if}
