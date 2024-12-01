<script lang="ts">
  import type { GitHubPR } from './types';
  import StatusBadge from './StatusBadge.svelte';
  import RepoIcon from './RepoIcon.svelte';
  import PRIcon from './PRIcon.svelte';
  import { getContrastYIQ, hexToRGBA, getPRState, getStatusType, formatDate } from './utils';
  import { getThemeContext } from '$lib/stores/contexts';

  let { pr }: { pr: GitHubPR } = $props();

  const themeCtx = getThemeContext();

  let isHovered = $state(false);

  const prState = getPRState(pr);
  const status = getStatusType(pr);
  const formattedDate = formatDate(new Date(pr.node.createdAt));
</script>

<div
  class="border bg-background px-3 py-2 transition-all duration-200 hover:bg-foreground/5"
  class:opacity-60={pr.node.closed && !pr.node.merged}
  onmouseenter={() => (isHovered = true)}
  onmouseleave={() => (isHovered = false)}
>
  <div class="mb-2 flex items-center justify-between">
    <div class="flex items-center gap-2 text-sm text-secondary-foreground">
      <RepoIcon />
      <span>{pr.node.repository.nameWithOwner}</span>
      <span class="text-github-text-muted">#{pr.node.number}</span>
    </div>
    {#if pr.node.isReadByViewer}
      <div class="h-2 w-2 animate-pulse rounded-full bg-blue-500"></div>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    <PRIcon state={prState} />
    <h3 class="truncate font-semibold">
      <a href={pr.node.url} target="_blank" rel="noopener noreferrer" class="transition-colors hover:text-blue-400">
        {pr.node.title}
      </a>
    </h3>
  </div>

  <div class="flex flex-row items-end justify-between">
    <div class="mt-2 flex flex-row items-center gap-4 text-sm">
      <span class="flex items-center gap-1 font-medium text-github-text-secondary">
        {#if pr.node.author.__typename === 'Bot'}
          <img
            class="avatar"
            src="https://avatars.githubusercontent.com/in/29110?s=40&amp;v=4"
            width="20"
            height="20"
            alt="bot"
          />
        {/if}
        {pr.node.author.login}
      </span>
      <span class="text-github-text-muted">{formattedDate}</span>
      {#if pr.node.totalCommentsCount > 0}
        <span class="flex items-center gap-1 text-github-text-muted">
          <svg class="h-4 w-4" viewBox="0 0 16 16">
            <path
              fill="currentColor"
              d="M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0 1 13.25 12H9.06l-2.573 2.573A1.458 1.458 0 0 1 4 13.543V12H2.75A1.75 1.75 0 0 1 1 10.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h4.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"
            />
          </svg>
          {pr.node.totalCommentsCount}
        </span>
      {/if}
      {#if pr.node.labels.edges.length > 0}
        {#each pr.node.labels.edges as label}
          {#if themeCtx.isDark}
            <span
              class="rounded-full border px-2 text-xs text-black"
              style="color: #{label.node.color}; filter: brightness(160%); border-color: {hexToRGBA(
                label.node.color,
                0.3
              )}; background-color: {hexToRGBA(label.node.color, 0.18)};">{label.node.name}</span
            >
          {:else}
            <span
              class="rounded-full px-2 text-xs font-semibold text-black"
              style="background-color: #{label.node.color}; color: {getContrastYIQ(label.node.color)};"
              >{label.node.name}</span
            >
          {/if}
        {/each}
      {/if}
    </div>
    <StatusBadge {status} />
  </div>
</div>
