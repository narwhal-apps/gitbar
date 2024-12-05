<script>
  import Footer from './Footer.svelte';
  import Reviews from './Reviews.svelte';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { setGithubContext } from '$lib/stores/contexts';
  import { createGithubStore } from '$lib/stores/createGithubStore.svelte';
  import { onMount } from 'svelte';

  const githubStore = createGithubStore();

  onMount(() => {
    githubStore.fetchInitialReviews();
  });
  setGithubContext(githubStore);
</script>

<ScrollArea class="h-[344px]">
  <Reviews />
</ScrollArea>
<Footer />
