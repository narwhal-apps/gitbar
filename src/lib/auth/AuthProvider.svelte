<script lang="ts">
    import type { Snippet } from 'svelte';
    import { AuthStore } from './authStore.svelte';
    import { setAuthContext } from './context';
    import { onMount } from 'svelte';
    import type { AuthState } from '../../types';
    
    let { account, children }: { account: AuthState, children: Snippet } = $props();

    const store = new AuthStore();
    setAuthContext(store);
    
    onMount(() => {
      if(!account.user) {
        store.signIn(account);
      }
      // Initialize any necessary auth state here
      return () => {
        // Cleanup if needed
      };
    });

  </script>
  
  {@render children?.()}