<script lang="ts">
  import type { GitHubPR } from './types';
  import { getPRState, getStatusType, formatDate } from './types';
  import StatusBadge from './StatusBadge.svelte';
  import RepoIcon from './RepoIcon.svelte';
  import PRIcon from './PRIcon.svelte';

  let { pr }: { pr: GitHubPR } = $props();
  
  let isHovered = $state(false);

  const prState = getPRState(pr);
  const status = getStatusType(pr);
  const formattedDate = formatDate(new Date(pr.node.createdAt));
</script>

<div 
  class="bg-github-card border border-github-border px-3 py-2 transition-all duration-200 hover:bg-foreground/5"
  class:opacity-60={pr.node.closed && !pr.node.merged}
  onmouseenter={() => isHovered = true}
  onmouseleave={() => isHovered = false}
>
  <div class="flex justify-between items-center mb-2">
    <div class="flex items-center gap-2 text-sm text-github-text-secondary">
      <RepoIcon />
      <span>{pr.node.repository.nameWithOwner}</span>
      <span class="text-github-text-muted">#{pr.node.number}</span>
    </div>
    {#if pr.node.isReadByViewer}
      <div class="w-2 h-2 animate-pulse rounded-full bg-blue-500"></div>
    {/if}
    
    <!-- <div class="flex flex-row gap-1 items-center">
      <StatusBadge {status} />
      {#if pr.node.isReadByViewer}
        <div class="w-2 h-2 rounded-full bg-blue-500"></div>
      {/if}
    </div> -->
    
  </div>
  
  <div class="flex items-center gap-2">
    <PRIcon state={prState} />
    <h3 class="font-semibold truncate">
      <a href={pr.node.url} target="_blank" rel="noopener noreferrer" 
         class="hover:text-blue-400 transition-colors">
        {pr.node.title}
      </a>
    </h3>
  </div>
  
  <div class="flex flex-row justify-between items-center">

    <div class="flex flex-row items-center gap-4 text-sm mt-2">
      <span class="font-medium text-github-text-secondary flex items-center gap-1">
        {#if pr.node.author.__typename === 'Bot'}
        <img class="avatar" src="https://avatars.githubusercontent.com/in/29110?s=40&amp;v=4" width="20" height="20" alt="bot">
        {/if}
        {pr.node.author.login}
      </span>
      <span class="text-github-text-muted">{formattedDate}</span>
      {#if pr.node.totalCommentsCount > 0}
      <span class="text-github-text-muted flex items-center gap-1">
        <svg class="w-4 h-4" viewBox="0 0 16 16">
          <path fill="currentColor" d="M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0 1 13.25 12H9.06l-2.573 2.573A1.458 1.458 0 0 1 4 13.543V12H2.75A1.75 1.75 0 0 1 1 10.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h4.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"/>
        </svg>
        {pr.node.totalCommentsCount}
      </span>
      {/if}
      {#if pr.node.labels.edges.length > 0}
      <div class="flex flex-wrap gap-2">
        {#each pr.node.labels.edges as {node}}
        <span class="px-2 py-0.5 text-xs font-medium rounded-full"
        style="background-color: #{node.color}20; color: #{node.color};">
        {node.name}
      </span>
      {/each}
    </div>
    {/if}
  </div>
  <StatusBadge {status} />
</div>
</div>