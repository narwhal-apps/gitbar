<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-shell';
  import { timeAgo } from '../lib/formatters';
  import { github } from '../lib/github';
  import { auth, defaultSettings } from '../lib/auth';
  import { appearance } from '../lib/theme';
  import { Skeleton } from '$lib/components/ui/skeleton';
  import { Badge } from '$lib/components/ui/badge';
  import { Tooltip, TooltipContent, TooltipTrigger, TooltipProvider } from '$lib/components/ui/tooltip';

  let loading = $state(true);
  let dateUpdater = $state(0);
  const skeletons = 5;

  function onClick(url: string) {
    open(url);
  }
  const interval = setInterval(() => {
    if ($auth.account) {
      $github.fetchReviews($auth.account, $auth.githubSettings);
      dateUpdater++;
    }
  }, $auth.settings.fetchInterval || defaultSettings.fetchInterval);

  const hexToRGBA = (hexCode: string, opacity = 1) => {
    let hex = hexCode.replace('#', '');

    if (hex.length === 3) {
      hex = `${hex[0]}${hex[0]}${hex[1]}${hex[1]}${hex[2]}${hex[2]}`;
    }

    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);

    /* Backward compatibility for whole number based opacity values. */
    if (opacity > 1 && opacity <= 100) {
      opacity = opacity / 100;
    }

    return `rgba(${r},${g},${b},${opacity})`;
  };

  const getContrastYIQ = (hexcolor: string) => {
    const r = parseInt(hexcolor.substring(0, 2), 16);
    const g = parseInt(hexcolor.substring(2, 4), 16);
    const b = parseInt(hexcolor.substring(4, 6), 16);
    const yiq = (r * 299 + g * 587 + b * 114) / 1000;
    return yiq >= 128 ? 'black' : 'white';
  };

  onMount(() => {
    if ($auth.account) {
      $github.fetchReviews($auth.account, $auth.githubSettings).finally(() => (loading = false));
    }
  });

  onDestroy(() => clearInterval(interval));

  $inspect('$github.reviews.count', $github.reviews);
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
{:else if $github.reviews.count === 0}
  <div class="mx-4 my-8">
    <div class="w-full rounded-lg border bg-background p-4 shadow-md">
      <span class="flex items-center">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="h-8 w-8 pr-2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M6.633 10.5c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 012.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 00.322-1.672V3a.75.75 0 01.75-.75A2.25 2.25 0 0116.5 4.5c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 01-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 00-1.423-.23H5.904M14.25 9h2.25M5.904 18.75c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 01-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 10.203 4.167 9.75 5 9.75h1.053c.472 0 .745.556.5.96a8.958 8.958 0 00-1.302 4.665c0 1.194.232 2.333.654 3.375z"
          />
        </svg>
        You have no reviews at the moment!
      </span>
    </div>
  </div>
{:else}
  <ul class="mb-14 divide-y" id="reviews">
    {#each $github.reviews.data as review}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li class="group p-2 hover:cursor-pointer hover:bg-muted" onclick={() => onClick(review.node.url)}>
        <div class="flex items-start">
          <div class="pr-1">
            <span aria-label="Open pull request">
              {#if review.node.merged}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  width="24"
                  height="24"
                  class="inline-block h-4 w-4 overflow-visible fill-purple-500"
                  ><path
                    d="M15 13.25a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm-12.5 6a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm0-14.5a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0ZM5.75 6.5a1.75 1.75 0 1 0-.001-3.501A1.75 1.75 0 0 0 5.75 6.5Zm0 14.5a1.75 1.75 0 1 0-.001-3.501A1.75 1.75 0 0 0 5.75 21Zm12.5-6a1.75 1.75 0 1 0-.001-3.501A1.75 1.75 0 0 0 18.25 15Z"
                  ></path><path d="M6.5 7.25c0 2.9 2.35 5.25 5.25 5.25h4.5V14h-4.5A6.75 6.75 0 0 1 5 7.25Z"></path><path
                    d="M5.75 16.75A.75.75 0 0 1 5 16V8a.75.75 0 0 1 1.5 0v8a.75.75 0 0 1-.75.75Z"
                  ></path></svg
                >
              {:else if review.node.closed}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  width="24"
                  height="24"
                  class="inline-block h-4 w-4 overflow-visible fill-red-500"
                  ><path
                    d="M22.266 2.711a.75.75 0 1 0-1.061-1.06l-1.983 1.983-1.984-1.983a.75.75 0 1 0-1.06 1.06l1.983 1.983-1.983 1.984a.75.75 0 0 0 1.06 1.06l1.984-1.983 1.983 1.983a.75.75 0 0 0 1.06-1.06l-1.983-1.984 1.984-1.983ZM4.75 1.5a3.25 3.25 0 0 1 .745 6.414A.827.827 0 0 1 5.5 8v8a.827.827 0 0 1-.005.086A3.25 3.25 0 0 1 4.75 22.5a3.25 3.25 0 0 1-.745-6.414A.827.827 0 0 1 4 16V8c0-.029.002-.057.005-.086A3.25 3.25 0 0 1 4.75 1.5ZM16 19.25a3.252 3.252 0 0 1 2.5-3.163V9.625a.75.75 0 0 1 1.5 0v6.462a3.252 3.252 0 0 1-.75 6.413A3.25 3.25 0 0 1 16 19.25ZM3 4.75a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 4.75Zm0 14.5a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 19.25Zm16.25-1.75a1.75 1.75 0 1 0 .001 3.501 1.75 1.75 0 0 0-.001-3.501Z"
                  ></path></svg
                >
              {:else if review.node.isDraft}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  width="24"
                  height="24"
                  class="inline-block h-4 w-4 overflow-visible fill-gray-500"
                  ><path
                    d="M4.75 1.5a3.25 3.25 0 0 1 .745 6.414A.827.827 0 0 1 5.5 8v8a.827.827 0 0 1-.005.086A3.25 3.25 0 0 1 4.75 22.5a3.25 3.25 0 0 1-.745-6.414A.827.827 0 0 1 4 16V8c0-.029.002-.057.005-.086A3.25 3.25 0 0 1 4.75 1.5ZM16 19.25a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0ZM3 4.75a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 4.75Zm0 14.5a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 19.25Zm16.25-1.75a1.75 1.75 0 1 0 .001 3.501 1.75 1.75 0 0 0-.001-3.501Zm0-11.5a1.75 1.75 0 1 0 0-3.5 1.75 1.75 0 0 0 0 3.5ZM21 11.25a1.75 1.75 0 1 1-3.5 0 1.75 1.75 0 0 1 3.5 0Z"
                  ></path></svg
                >
              {:else}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="inline-block h-4 w-4 overflow-visible fill-green-500"
                  viewBox="0 0 24 24"
                  width="24"
                  height="24"
                  ><path
                    d="M16 19.25a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm-14.5 0a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0Zm0-14.5a3.25 3.25 0 1 1 6.5 0 3.25 3.25 0 0 1-6.5 0ZM4.75 3a1.75 1.75 0 1 0 .001 3.501A1.75 1.75 0 0 0 4.75 3Zm0 14.5a1.75 1.75 0 1 0 .001 3.501A1.75 1.75 0 0 0 4.75 17.5Zm14.5 0a1.75 1.75 0 1 0 .001 3.501 1.75 1.75 0 0 0-.001-3.501Z"
                  ></path><path
                    d="M13.405 1.72a.75.75 0 0 1 0 1.06L12.185 4h4.065A3.75 3.75 0 0 1 20 7.75v8.75a.75.75 0 0 1-1.5 0V7.75a2.25 2.25 0 0 0-2.25-2.25h-4.064l1.22 1.22a.75.75 0 0 1-1.061 1.06l-2.5-2.5a.75.75 0 0 1 0-1.06l2.5-2.5a.75.75 0 0 1 1.06 0ZM4.75 7.25A.75.75 0 0 1 5.5 8v8A.75.75 0 0 1 4 16V8a.75.75 0 0 1 .75-.75Z"
                  ></path></svg
                >
              {/if}
            </span>
          </div>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger class="w-full overflow-hidden text-ellipsis whitespace-nowrap text-left">
                <div class="w-full overflow-hidden text-ellipsis whitespace-nowrap">
                  <span
                    class="inline-flex w-full items-center justify-between text-sm text-slate-500 dark:text-gray-300"
                    >{review.node.repository.nameWithOwner} #{review.node.number}
                    {#if !review.node.isReadByViewer}
                      <div class="mr-2 inline-flex h-2 w-2 rounded-full bg-blue-500 dark:bg-blue-400"></div>
                    {/if}</span
                  >
                  <br />
                  <span
                    class="text-sm font-medium text-gray-900 transition-colors group-hover:text-blue-700 dark:text-white dark:group-hover:text-blue-500"
                  >
                    {review.node.title}
                  </span>
                  <br />
                  {#if !$auth.settings?.isCompactMode}
                    <div class="flex flex-row items-center justify-between gap-1">
                      {#key dateUpdater}
                        <span class="truncate text-sm text-gray-500 dark:text-gray-400">
                          {timeAgo(review.node.createdAt)} by {review.node.author?.login || 'ghost'}
                          {#if review.node.author?.__typename === 'Bot'}
                            <Badge variant="outline" class="inline-flex py-0.5 text-[11px] leading-none text-opacity-60"
                              >Bot</Badge
                            >
                          {/if}
                        </span>
                      {/key}
                      {#if review.node.statusCheckRollup && review.node.statusCheckRollup?.state !== 'ERROR'}
                        <Badge
                          variant="outline"
                          class="inline-flex items-center gap-0.5 px-1 py-0 text-[11px] leading-[11px] text-gray-500"
                        >
                          {#if review.node.statusCheckRollup.state === 'SUCCESS'}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              viewBox="0 0 16 16"
                              width="16"
                              height="16"
                              class="flex h-3 w-3 overflow-visible fill-green-600"
                              ><path
                                d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
                              ></path></svg
                            >
                          {:else if review.node.statusCheckRollup.state === 'FAILURE'}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              viewBox="0 0 12 12"
                              width="12"
                              height="12"
                              class="flex h-3 w-3 overflow-visible fill-red-600"
                              ><path
                                d="M2.22 2.22a.749.749 0 0 1 1.06 0L6 4.939 8.72 2.22a.749.749 0 1 1 1.06 1.06L7.061 6 9.78 8.72a.749.749 0 1 1-1.06 1.06L6 7.061 3.28 9.78a.749.749 0 1 1-1.06-1.06L4.939 6 2.22 3.28a.749.749 0 0 1 0-1.06Z"
                              ></path></svg
                            >
                          {:else}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              class="flex h-3 w-3 overflow-visible fill-yellow-500"
                              viewBox="0 0 16 16"
                              width="16"
                              height="16"><path d="M8 4a4 4 0 1 1 0 8 4 4 0 0 1 0-8Z"></path></svg
                            >
                          {/if} Checks
                        </Badge>
                      {/if}
                    </div>
                    <div class="mt-1 flex flex-row flex-wrap gap-1">
                      {#each review.node.labels.edges as label}
                        {#if $appearance.theme === 'dark'}
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
                    </div>
                    <Badge
                      variant="outline"
                      class="inline-flex items-center gap-0.5 px-1 py-0 text-[11px] leading-[11px] text-gray-500"
                    >
                      {#if review.node.reviewDecision === 'APPROVED'}
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          viewBox="0 0 16 16"
                          width="16"
                          height="16"
                          class="flex h-3 w-3 overflow-visible fill-green-600"
                          ><path
                            d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"
                          ></path></svg
                        >
                      {:else}
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          class="flex h-3 w-3 overflow-visible fill-gray-400 dark:fill-gray-500"
                          viewBox="0 0 16 16"
                          width="16"
                          height="16"><path d="M8 4a4 4 0 1 1 0 8 4 4 0 0 1 0-8Z"></path></svg
                        >
                      {/if} Reviews
                    </Badge>
                    {#if review.node.totalCommentsCount > 0}
                      <Badge
                        variant="outline"
                        class="inline-flex items-center gap-1 px-1 py-0 text-[11px] leading-[11px] text-gray-500"
                      >
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          viewBox="0 0 16 16"
                          width="16"
                          height="16"
                          class="flex h-3 w-3 overflow-visible fill-gray-500"
                          ><path
                            d="M1.75 1h8.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 10.25 10H7.061l-2.574 2.573A1.458 1.458 0 0 1 2 11.543V10h-.25A1.75 1.75 0 0 1 0 8.25v-5.5C0 1.784.784 1 1.75 1ZM1.5 2.75v5.5c0 .138.112.25.25.25h1a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h3.5a.25.25 0 0 0 .25-.25v-5.5a.25.25 0 0 0-.25-.25h-8.5a.25.25 0 0 0-.25.25Zm13 2a.25.25 0 0 0-.25-.25h-.5a.75.75 0 0 1 0-1.5h.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 14.25 12H14v1.543a1.458 1.458 0 0 1-2.487 1.03L9.22 12.28a.749.749 0 0 1 .326-1.275.749.749 0 0 1 .734.215l2.22 2.22v-2.19a.75.75 0 0 1 .75-.75h1a.25.25 0 0 0 .25-.25Z"
                          ></path></svg
                        >
                        {review.node.totalCommentsCount}
                      </Badge>
                    {/if}
                  {/if}
                </div>
              </TooltipTrigger>
              <TooltipContent>
                {review.node.title}
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </li>
    {/each}
  </ul>
{/if}
