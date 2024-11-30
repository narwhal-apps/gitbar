<script lang="ts">
  import type { Snippet } from 'svelte';
  import Login from './Login.svelte';
  import { auth } from '../lib/auth';
  import { loadState } from '$lib/storage';
  import type { AuthState } from '../types';
  import { authStore } from '../lib/auth.svelte';
  import AuthProvider from '$lib/auth/AuthProvider.svelte';
  authStore.signIn(authStore.account);

  const account = loadState().account;


  let { children }: { children: Snippet } = $props();
</script>

{#if account}
  <AuthProvider account={account} >
    {@render children?.()}
  </AuthProvider>
{:else}
  <!-- user is a guest. Show a login widget instead of the current page. -->
  <Login />
{/if}
