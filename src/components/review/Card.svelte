<script lang="ts">
  import { TooltipProvider, Tooltip, TooltipTrigger, TooltipContent } from '$lib/components/ui/tooltip';
  import type { GitHubPR } from './types';
  import StatusBadge from './StatusBadge.svelte';
  import RepoIcon from './RepoIcon.svelte';
  import PRIcon from './PRIcon.svelte';
  import { getContrastYIQ, hexToRGBA, getPRState, getStatusType, formatDate } from './utils';
  import { getGithubContext, getThemeContext } from '$lib/stores/contexts';
  import { cn } from '$lib/utils';

  let { pr, index }: { pr: GitHubPR; index: number } = $props();

  const themeCtx = getThemeContext();

  const ghCtx = getGithubContext();

  const prState = getPRState(pr);
  const status = getStatusType(pr);
  const formattedDate = formatDate(new Date(pr.node.createdAt));
</script>

<div
  role="menuitem"
  tabindex={index + 1}
  data-compact={ghCtx.settings.isCompactMode}
  class="group flex flex-col gap-2 border bg-background px-3 py-2 outline-0 -outline-offset-2 transition-all duration-200 hover:bg-foreground/5 data-[compact=true]:gap-0"
  class:opacity-60={pr.node.closed && !pr.node.merged}
>
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2 text-sm text-secondary-foreground group-data-[compact=true]:text-xs">
      <RepoIcon />
      <span>{pr.node.repository.nameWithOwner}</span>
      <span class="text-github-text-muted">#{pr.node.number}</span>
    </div>
    {#if pr.node.isReadByViewer}
      <TooltipProvider delayDuration={200}>
        <Tooltip>
          <TooltipTrigger>
            <div class="mr-[6px] h-2 w-2 animate-pulse cursor-default rounded-full bg-blue-500"></div>
          </TooltipTrigger>
          <TooltipContent
            class={cn('border border-blue-500/25', ghCtx.settings.isCompactMode && 'px-1 py-0 text-[12px]')}
            >Unread</TooltipContent
          >
        </Tooltip>
      </TooltipProvider>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    <PRIcon state={prState} />
    <h3 class="truncate font-semibold">
      <a
        href={pr.node.url}
        target="_blank"
        rel="noopener noreferrer"
        class="transition-colors hover:text-blue-400 group-data-[compact=true]:text-sm"
      >
        {pr.node.title}
      </a>
    </h3>
  </div>

  <div class="flex flex-row items-end justify-between">
    <div
      class="flex flex-row items-center gap-4 text-sm group-data-[compact=true]:gap-2 group-data-[compact=true]:text-xs"
    >
      <span class="flex items-center gap-1 font-medium text-github-text-secondary">
        {#if pr.node.author.login === 'dependabot'}
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
          <svg class="h-4 w-4 group-data-[compact=true]:h-3 group-data-[compact=true]:w-3" viewBox="0 0 16 16">
            <path
              fill="currentColor"
              d="M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0 1 13.25 12H9.06l-2.573 2.573A1.458 1.458 0 0 1 4 13.543V12H2.75A1.75 1.75 0 0 1 1 10.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h4.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"
            />
          </svg>
          {pr.node.totalCommentsCount}
        </span>
      {/if}
    </div>
    <div class="flex flex-row items-center gap-2 group-data-[compact=true]:gap-1">
      {#if pr.node.labels.edges.length > 0}
        {#each pr.node.labels.edges as label}
          {#if themeCtx.isDark}
            <span
              class="items-center rounded-full border px-2 text-xs text-black group-data-[compact=true]:p-0 group-data-[compact=true]:text-[10px]"
              style="color: #{label.node.color}; filter: brightness(160%); border-color: {hexToRGBA(
                label.node.color,
                ghCtx.settings.isCompactMode ? 0 : 0.3
              )}; background-color: {hexToRGBA(label.node.color, ghCtx.settings.isCompactMode ? 0 : 0.18)};"
              >{label.node.name}</span
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
      <StatusBadge {status} />
    </div>
  </div>
</div>
