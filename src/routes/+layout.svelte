<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { appState } from '$lib/appState.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { AppState, StateChangePayload } from '../types/index';

  let { children } = $props();

  async function subscribeToState() {
    try {
      // Listen for state changes
      await listen<StateChangePayload>('state-changed', event => {
        const { newState, changedFields } = event.payload;

        changedFields.forEach(field => (appState[field] = newState[field]));
      });
    } catch (error) {
      console.error('Error subscribing to state:', error);
    }
  }

  onMount(async () => {
    // Get initial state
    const initialState = await invoke<AppState>('get_state');

    // Update initial state
    appState.initialize(initialState);

    // Subscribe to state changes
    await subscribeToState();

    // use the existence of the dark class on the html element for the initial value
    let isDark = document.documentElement.classList.contains('dark');

    appState.theme = isDark ? 'dark' : 'light';
  });

  $effect(() => {
    invoke('update_state', { updatedState: appState.getState });
    if (appState.isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  });
</script>

<svelte:head>
  <!-- set dark mode class based on user preference / device settings (in head to avoid FOUC) -->
  <script>
    if (
      localStorage.getItem('gitbar-theme') === 'dark' ||
      (!localStorage.getItem('gitbar-theme') && window.matchMedia('(prefers-color-scheme: dark)').matches)
    ) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  </script>
</svelte:head>

{#if appState.initializing}
  <div class="flex h-screen items-center justify-center">
    <div class="h-32 w-32 animate-spin rounded-full border-b-2 border-t-2 border-gray-900"></div>
  </div>
{:else}
  {@render children?.()}
{/if}
