<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { appState } from '$lib/appState.svelte';

  let { children } = $props();

  onMount(() => {
    // use the existence of the dark class on the html element for the initial value
    let isDark = document.documentElement.classList.contains('dark');

    appState.theme = isDark ? 'dark' : 'light';
  });

  $effect(() => {
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

{@render children?.()}
