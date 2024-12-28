<script lang="ts">
  import { appState } from '$lib/appState.svelte';
  import Empty from './review/Empty.svelte';
  import Card from './review/Card.svelte';
</script>

{#if appState.issueCount === 0}
  <Empty />
{:else}
  <ul id="reviews" role="menu">
    {#each appState.reviews as pr, index}
      <Card {pr} {index} />
    {/each}
  </ul>
{/if}
