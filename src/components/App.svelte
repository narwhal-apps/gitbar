<script>
  import Footer from './Footer.svelte';
  import Reviews from './Reviews.svelte';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { setGithubContext, getAuthContext } from '$lib/stores/contexts';
  import { createGithubStore } from '$lib/stores/createGithubStore.svelte';
  import { onMount } from 'svelte';

  const githubStore = createGithubStore();

  onMount(() => {
    githubStore.fetchInitialReviews();
  });

  setGithubContext(githubStore);
</script>

<ScrollArea class="h-[400px]">
  <Reviews />
</ScrollArea>
<Footer />
