<script lang="ts">
  let { status }: { status: 'success' | 'pending' | 'failure' } = $props();

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
  class="group flex h-5 w-5 items-center gap-1 rounded-full px-1 py-1 text-xs font-medium text-white {bg}"
  onmouseenter={() => (isHovered = true)}
  onmouseleave={() => (isHovered = false)}
>
  {#if status === 'success'}
    <svg class="h-3 w-3" viewBox="0 0 16 16">
      <path
        fill="currentColor"
        d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"
      />
    </svg>
  {:else if status === 'failure'}
    <svg class="h-3 w-3" viewBox="0 0 16 16">
      <path
        fill="currentColor"
        d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"
      />
    </svg>
  {/if}
  <!-- {text} -->
</div>
