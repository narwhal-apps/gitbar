<script lang="ts">
  import '../app.css';
  import { setAuthContext, setThemeContext } from '$lib/stores/contexts';
  import { createAuthStore } from '$lib/stores/createAuthStore.svelte';
  import { onMount } from 'svelte';
  import { createThemeStore } from '$lib/stores/createThemeStore.svelte';

  const authStore = createAuthStore();
  setAuthContext(authStore);

  const themeStore = createThemeStore();
  setThemeContext(themeStore);

  let { children } = $props();

  onMount(() => {
    // use the existence of the dark class on the html element for the initial value
    let isDark = document.documentElement.classList.contains('dark');

    themeStore.theme = isDark ? 'dark' : 'light';
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
