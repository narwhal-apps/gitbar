<script lang="ts">
  import Footer from './Footer.svelte';
  import Reviews from './Reviews.svelte';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { setGithubContext } from '$lib/stores/contexts';
  import { createGithubStore } from '$lib/stores/createGithubStore.svelte';
  import { onMount } from 'svelte';
  import { appState } from '$lib/stores/state.svelte';

  console.log('appState :', appState);

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
