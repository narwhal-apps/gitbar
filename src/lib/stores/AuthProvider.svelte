<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { loadState } from '../storage';
  import { AUTH_CONTEXT_KEY } from './constants';
  import { initContext } from './authContext.svelte';

  const context = initContext();

  onMount(() => {
    const prevState = loadState();
    if (prevState.account) {
      context.account = prevState.account;
    }
  });

  setContext(AUTH_CONTEXT_KEY, context);

  let { children } = $props();
</script>

{@render children?.()}
