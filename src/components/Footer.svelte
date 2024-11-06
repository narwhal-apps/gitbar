<script lang="ts">
  import { open } from '@tauri-apps/api/shell';
  import { auth } from '../lib/auth';
  import { github } from '../lib/github';
  import * as Dialog from '$lib/components/ui/dialog';
  import * as Avatar from '$lib/components/ui/avatar';
  import Settings from './Settings.svelte';
  import Filters from './Filters.svelte';

  let fetching = $state(false);
  let settingsVisible = $state(false);
  let filterVisible = $state(false);

  const startFetch = () => {
    fetching = true;
    if ($auth.account) {
      $github.fetchReviews($auth.account, $auth.githubSettings).finally(() => {
        fetching = false;
      });
    }
  };

  const avatarFallback = (name: string | undefined) => {
    if (!name) return '';
    return name
      .split(' ')
      .map(n => n[0])
      .join('');
  };
</script>

<footer class="fixed bottom-0 left-0 w-full bg-muted px-2 py-2 shadow-md">
  <div class="flex justify-between">
    <div class="flex items-center p-2">
      <Avatar.Root class="h-6 w-6 flex-shrink-0">
        <Avatar.Image src={$auth.account?.user?.avatar_url} alt={$auth.account?.user?.name || ''} />
        <Avatar.Fallback class="border border-muted text-xs font-medium uppercase text-muted-foreground"
          >{avatarFallback($auth.account?.user?.name)}</Avatar.Fallback
        >
      </Avatar.Root>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <button
        class={`${
          $auth.account?.user?.html_url ? 'cursor-pointer hover:text-slate-600/70 dark:hover:text-white/70' : ''
        } ml-1 block truncate`}
        onclick={() => ($auth.account?.user?.html_url ? open($auth.account?.user?.html_url) : null)}
        >{$auth.account?.user?.name || $auth.account?.user?.email || ''}</button
      >
    </div>
    <div class="flex justify-between">
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button
        class="p-2 text-slate-600 hover:text-slate-600/70 dark:text-white dark:hover:text-white/70"
        onclick={startFetch}
        title="Fetch"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          width="24"
          height="24"
          stroke="none"
          fill="currentColor"
          class={`h-4 w-4 ${fetching ? 'animate-spin' : ''}`}
          ><path
            d="M3.38 8A9.502 9.502 0 0 1 12 2.5a9.502 9.502 0 0 1 9.215 7.182.75.75 0 1 0 1.456-.364C21.473 4.539 17.15 1 12 1a10.995 10.995 0 0 0-9.5 5.452V4.75a.75.75 0 0 0-1.5 0V8.5a1 1 0 0 0 1 1h3.75a.75.75 0 0 0 0-1.5H3.38Zm-.595 6.318a.75.75 0 0 0-1.455.364C2.527 19.461 6.85 23 12 23c4.052 0 7.592-2.191 9.5-5.451v1.701a.75.75 0 0 0 1.5 0V15.5a1 1 0 0 0-1-1h-3.75a.75.75 0 0 0 0 1.5h2.37A9.502 9.502 0 0 1 12 21.5c-4.446 0-8.181-3.055-9.215-7.182Z"
          ></path></svg
        >
      </button>
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button
        class="p-2 text-slate-600 hover:text-slate-600/70 dark:text-white dark:hover:text-white/70"
        onclick={() => (filterVisible = true)}
        title="Filter"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          width="24"
          height="24"
          class="h-4 w-4"
          stroke="none"
          fill="currentColor"
          ><path
            d="M11 18.25a.75.75 0 0 1 .75-.75h8.5a.75.75 0 0 1 0 1.5h-8.5a.75.75 0 0 1-.75-.75Zm-8-12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 6.25Zm13 6a.75.75 0 0 1 .75-.75h3.5a.75.75 0 0 1 0 1.5h-3.5a.75.75 0 0 1-.75-.75ZM8.75 16a.75.75 0 0 1 .75.75v3a.75.75 0 0 1-1.5 0v-3a.75.75 0 0 1 .75-.75Z"
          ></path><path
            d="M3 18.25a.75.75 0 0 1 .75-.75h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1-.75-.75Zm0-6a.75.75 0 0 1 .75-.75h8.5a.75.75 0 0 1 0 1.5h-8.5a.75.75 0 0 1-.75-.75ZM16.75 10a.75.75 0 0 1 .75.75v3a.75.75 0 0 1-1.5 0v-3a.75.75 0 0 1 .75-.75ZM14 6.25a.75.75 0 0 1 .75-.75h5.5a.75.75 0 0 1 0 1.5h-5.5a.75.75 0 0 1-.75-.75ZM11.25 4a.75.75 0 0 1 .75.75v3a.75.75 0 0 1-1.5 0v-3a.75.75 0 0 1 .75-.75Z"
          ></path></svg
        >
      </button>
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button
        class="p-2 text-slate-600 hover:text-slate-600/70 dark:text-white dark:hover:text-white/70"
        onclick={() => (settingsVisible = true)}
        title="Settings"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          width="24"
          height="24"
          class="h-4 w-4"
          stroke="none"
          fill="currentColor"
          ><path d="M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0Zm-1.5 0a2.5 2.5 0 1 0-5 0 2.5 2.5 0 0 0 5 0Z"></path><path
            d="M12 1c.266 0 .532.009.797.028.763.055 1.345.617 1.512 1.304l.352 1.45c.019.078.09.171.225.221.247.089.49.19.728.302.13.061.246.044.315.002l1.275-.776c.603-.368 1.411-.353 1.99.147.402.349.78.726 1.128 1.129.501.578.515 1.386.147 1.99l-.776 1.274c-.042.069-.058.185.002.315.112.238.213.481.303.728.048.135.142.205.22.225l1.45.352c.687.167 1.249.749 1.303 1.512.038.531.038 1.063 0 1.594-.054.763-.616 1.345-1.303 1.512l-1.45.352c-.078.019-.171.09-.221.225-.089.248-.19.491-.302.728-.061.13-.044.246-.002.315l.776 1.275c.368.603.353 1.411-.147 1.99-.349.402-.726.78-1.129 1.128-.578.501-1.386.515-1.99.147l-1.274-.776c-.069-.042-.185-.058-.314.002a8.606 8.606 0 0 1-.729.303c-.135.048-.205.142-.225.22l-.352 1.45c-.167.687-.749 1.249-1.512 1.303-.531.038-1.063.038-1.594 0-.763-.054-1.345-.616-1.512-1.303l-.352-1.45c-.019-.078-.09-.171-.225-.221a8.138 8.138 0 0 1-.728-.302c-.13-.061-.246-.044-.315-.002l-1.275.776c-.603.368-1.411.353-1.99-.147-.402-.349-.78-.726-1.128-1.129-.501-.578-.515-1.386-.147-1.99l.776-1.274c.042-.069.058-.185-.002-.314a8.606 8.606 0 0 1-.303-.729c-.048-.135-.142-.205-.22-.225l-1.45-.352c-.687-.167-1.249-.749-1.304-1.512a11.158 11.158 0 0 1 0-1.594c.055-.763.617-1.345 1.304-1.512l1.45-.352c.078-.019.171-.09.221-.225.089-.248.19-.491.302-.728.061-.13.044-.246.002-.315l-.776-1.275c-.368-.603-.353-1.411.147-1.99.349-.402.726-.78 1.129-1.128.578-.501 1.386-.515 1.99-.147l1.274.776c.069.042.185.058.315-.002.238-.112.481-.213.728-.303.135-.048.205-.142.225-.22l.352-1.45c.167-.687.749-1.249 1.512-1.304C11.466 1.01 11.732 1 12 1Zm-.69 1.525c-.055.004-.135.05-.161.161l-.353 1.45a1.832 1.832 0 0 1-1.172 1.277 7.147 7.147 0 0 0-.6.249 1.833 1.833 0 0 1-1.734-.074l-1.274-.776c-.098-.06-.186-.036-.228 0a9.774 9.774 0 0 0-.976.976c-.036.042-.06.131 0 .228l.776 1.274c.314.529.342 1.18.074 1.734a7.147 7.147 0 0 0-.249.6 1.831 1.831 0 0 1-1.278 1.173l-1.45.351c-.11.027-.156.107-.16.162a9.63 9.63 0 0 0 0 1.38c.004.055.05.135.161.161l1.45.353a1.832 1.832 0 0 1 1.277 1.172c.074.204.157.404.249.6.268.553.24 1.204-.074 1.733l-.776 1.275c-.06.098-.036.186 0 .228.301.348.628.675.976.976.042.036.131.06.228 0l1.274-.776a1.83 1.83 0 0 1 1.734-.075c.196.093.396.176.6.25a1.831 1.831 0 0 1 1.173 1.278l.351 1.45c.027.11.107.156.162.16a9.63 9.63 0 0 0 1.38 0c.055-.004.135-.05.161-.161l.353-1.45a1.834 1.834 0 0 1 1.172-1.278 6.82 6.82 0 0 0 .6-.248 1.831 1.831 0 0 1 1.733.074l1.275.776c.098.06.186.036.228 0 .348-.301.675-.628.976-.976.036-.042.06-.131 0-.228l-.776-1.275a1.834 1.834 0 0 1-.075-1.733c.093-.196.176-.396.25-.6a1.831 1.831 0 0 1 1.278-1.173l1.45-.351c.11-.027.156-.107.16-.162a9.63 9.63 0 0 0 0-1.38c-.004-.055-.05-.135-.161-.161l-1.45-.353c-.626-.152-1.08-.625-1.278-1.172a6.576 6.576 0 0 0-.248-.6 1.833 1.833 0 0 1 .074-1.734l.776-1.274c.06-.098.036-.186 0-.228a9.774 9.774 0 0 0-.976-.976c-.042-.036-.131-.06-.228 0l-1.275.776a1.831 1.831 0 0 1-1.733.074 6.88 6.88 0 0 0-.6-.249 1.835 1.835 0 0 1-1.173-1.278l-.351-1.45c-.027-.11-.107-.156-.162-.16a9.63 9.63 0 0 0-1.38 0Z"
          ></path></svg
        >
      </button>
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button
        class="p-2 text-slate-600 hover:text-slate-600/70 dark:text-white dark:hover:text-white/70"
        onclick={$auth.signOut}
        title="Sign Out"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          width="24"
          height="24"
          class="h-4 w-4"
          stroke="none"
          fill="currentColor"
          ><path
            d="M3 3.25c0-.966.784-1.75 1.75-1.75h5.5a.75.75 0 0 1 0 1.5h-5.5a.25.25 0 0 0-.25.25v17.5c0 .138.112.25.25.25h5.5a.75.75 0 0 1 0 1.5h-5.5A1.75 1.75 0 0 1 3 20.75Zm16.006 9.5H10.75a.75.75 0 0 1 0-1.5h8.256l-3.3-3.484a.75.75 0 0 1 1.088-1.032l4.5 4.75a.75.75 0 0 1 0 1.032l-4.5 4.75a.75.75 0 0 1-1.088-1.032Z"
          ></path></svg
        >
      </button>
    </div>
  </div>
</footer>

<Dialog.Root bind:open={settingsVisible}>
  <Dialog.Content class="w-11/12 rounded-md">
    <Dialog.Header class="text-left">
      <Dialog.Title>Settings</Dialog.Title>
    </Dialog.Header>
    <Settings bind:modalVisible={settingsVisible} />
  </Dialog.Content>
</Dialog.Root>
<Dialog.Root bind:open={filterVisible}>
  <Dialog.Content class="w-11/12 rounded-md">
    <Dialog.Header class="text-left">
      <Dialog.Title>Filter</Dialog.Title>
    </Dialog.Header>
    <Filters bind:modalVisible={filterVisible} onSaved={startFetch} />
  </Dialog.Content>
</Dialog.Root>

<style>
  @keyframes slide-up {
    0% {
      transform: translateY(110vh);
    }
    100% {
      transform: translateY(0vh);
    }
  }

  footer {
    transform: translateY(110vh);
    animation: slide-up 1s forwards;
  }
</style>
