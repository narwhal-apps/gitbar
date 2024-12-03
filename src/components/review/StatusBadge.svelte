<script lang="ts">
  import { getGithubContext } from '$lib/stores/contexts';
  import { cn } from '$lib/utils';

  let { status }: { status: 'success' | 'pending' | 'failure' } = $props();

  const ghCtx = getGithubContext();

  const statusConfig = {
    success: {
      bg: 'bg-github-status-success',
      text: 'Passed',
    },
    pending: {
      bg: 'bg-github-status-pending',
      text: 'Pending',
    },
    failure: {
      bg: 'bg-github-status-failure',
      text: 'Failed',
    },
  };

  const { bg, text } = statusConfig[status];
  let isHovered = $state(false);
</script>

<div
  class={cn(
    'group flex h-5 w-5 items-center justify-center rounded-full text-xs font-medium text-white',
    bg,
    ghCtx.settings.isCompactMode && 'bg-transparent'
  )}
  onmouseenter={() => (isHovered = true)}
  onmouseleave={() => (isHovered = false)}
>
  {#if status === 'success'}
    <svg
      class="h-4 w-4 fill-white group-data-[compact=true]:fill-github-status-success group-data-[compact=true]:stroke-github-status-success"
      viewBox="0 0 16 16"
    >
      <path
        d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"
      />
    </svg>
  {:else if status === 'failure'}
    <svg
      class="h-4 w-4 fill-white group-data-[compact=true]:fill-github-status-failure group-data-[compact=true]:stroke-github-status-failure"
      viewBox="0 0 16 16"
    >
      <path
        d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
      />
    </svg>
  {:else if status === 'pending'}
    <svg
      class="h-5 w-5 animate-spin stroke-white p-1 text-white group-data-[compact=true]:stroke-github-status-pending"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke-width="4"></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
  {/if}
  <!-- {text} -->
</div>
