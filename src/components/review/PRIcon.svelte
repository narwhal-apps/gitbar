<script lang="ts">
  import { appState } from '$lib/appState.svelte';
  import { cn } from '$lib/utils';
  import type { PullRequestState } from './types';

  let { state }: { state: PullRequestState } = $props();

  const iconColors = {
    open: 'text-green-600 dark:text-green-500',
    draft: 'text-gray-400',
    merged: 'text-purple-500',
  };
</script>

{#if state === 'open'}
  <svg
    class={cn('h-5 w-5', iconColors[state], appState.settings.isCompactMode && 'h-3 w-3')}
    viewBox="0 0 16 16"
    fill="currentColor"
  >
    <path
      d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"
    />
  </svg>
{:else if state === 'draft'}
  <svg
    class={cn('h-5 w-5', iconColors[state], appState.settings.isCompactMode && 'h-3 w-3')}
    viewBox="0 0 16 16"
    fill="currentColor"
  >
    <path
      d="M3.25 1A2.25 2.25 0 0 1 4 5.372v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.251 2.251 0 0 1 3.25 1Zm9.5 14a2.25 2.25 0 1 1 0-4.5 2.25 2.25 0 0 1 0 4.5ZM2.5 3.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0ZM3.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm9.5 0a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5ZM14 7.5a1.25 1.25 0 1 1-2.5 0 1.25 1.25 0 0 1 2.5 0Zm0-4.25a1.25 1.25 0 1 1-2.5 0 1.25 1.25 0 0 1 2.5 0Z"
    />
  </svg>
{:else if state === 'merged'}
  <svg
    class={cn('h-5 w-5', iconColors[state], appState.settings.isCompactMode && 'h-3 w-3')}
    viewBox="0 0 16 16"
    fill="currentColor"
  >
    <path
      d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM5 3.25a.75.75 0 1 0 0 .005V3.25Z"
    />
  </svg>
{/if}
