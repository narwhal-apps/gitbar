<script lang="ts">
  import { Skeleton } from '$lib/components/ui/skeleton';
  import { appState } from '$lib/appState.svelte';
  import Empty from './review/Empty.svelte';
  import Card from './review/Card.svelte';
  import { onMount } from 'svelte';

  let loading = $state(false);

  onMount(() => {
    if (appState.reviews.length === 0) {
      loading = true;
      appState.fetchReviews().finally(() => {
        loading = false;
      });
    }
  });

  const skeletons = 5;
</script>

{#if loading}
  <ul class="divide-y">
    {#each Array.from({ length: skeletons }) as _}
      <li class="max-w m-2 mx-auto flex w-full items-center space-x-4 p-2">
        <Skeleton class="h-10 w-10 rounded-full" />
        <div class="space-y-2">
          <Skeleton class="h-3 w-[350px]" />
          <Skeleton class="h-3 w-[300px]" />
        </div>
      </li>
    {/each}
  </ul>
{:else if appState.reviewCount === 0}
  <Empty />
{:else}
  <ul id="reviews" role="menu">
    {#each appState.reviews as pr, index}
      <Card {pr} {index} />
    {/each}
  </ul>
{/if}
