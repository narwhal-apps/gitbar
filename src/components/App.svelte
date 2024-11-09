<script>
  import { onMount } from 'svelte';
  import AuthGuard from './AuthGuard.svelte';
  import Footer from './Footer.svelte';
  import Reviews from './Reviews.svelte';
  import { appearance } from '../lib/theme';
  import { ScrollArea } from '$lib/components/ui/scroll-area';

  onMount(() => {
    // use the existence of the dark class on the html element for the initial value
    let dark = document.documentElement.classList.contains('dark');
    $appearance.setTheme(dark);

    // listen for changes so we auto-adjust based on system settings
    const matcher = window.matchMedia('(prefers-color-scheme: dark)');
    matcher.addEventListener('change', $appearance.setMatchTheme);
    return () => matcher.removeEventListener('change', $appearance.setMatchTheme);
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

<AuthGuard>
  <ScrollArea class="h-[400px]">
    <Reviews />
  </ScrollArea>
  <Footer />
</AuthGuard>
