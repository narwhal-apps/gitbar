<script lang="ts">
  import type { Snippet } from 'svelte';
  import Login from './Login.svelte';
  import { auth } from '../lib/auth';

  let { children }: { children: Snippet } = $props();
</script>

{#if $auth.account}
  {@render children?.()}
{:else}
  <!-- user is a guest. Show a login widget instead of the current page. -->
  <Login />
{/if}
