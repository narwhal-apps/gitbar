<script lang="ts">
  import { loginSchema, type LoginSchema } from './schema';
  import { safeParse, flatten } from 'valibot';
  import { open } from '@tauri-apps/plugin-shell';
  import { onDestroy, onMount } from 'svelte';
  import { getServerPort } from '$lib/app';
  import { invoke } from '@tauri-apps/api/core';
  import { createAuthURL, getAccessToken } from '$lib/api';
  import { listen } from '@tauri-apps/api/event';
  import { Button } from '$lib/components/ui/button';
  import { cn } from '$lib/utils';
  import { Input } from '$lib/components/ui/input';
  import Label from '$lib/components/ui/label/label.svelte';
  import { appState } from '$lib/appState.svelte';

  const defaultHost = 'github.com';

  let token = $state('');
  let hostname = $state(defaultHost);
  const formData = $derived({ token, hostname });
  let fieldErrors = $state<{ [x: string]: [string, ...string[]] } | undefined>(undefined);
  let loading = $state(false);
  let processing = $state(false);

  let port: number;
  let unlistenFn: () => void;

  function handleToken() {
    open(createAuthURL(port));
  }

  onMount(async () => {
    if (!appState.isAuthenticated) {
      await invoke('start_server');
      port = await getServerPort();
      unlistenFn = await listen('code', async (event: { payload: string }) => {
        processing = true;
        try {
          const token = await getAccessToken({
            clientId: import.meta.env.VITE_CLIENT_ID,
            clientSecret: import.meta.env.VITE_CLIENT_SECRET,
            code: event.payload,
            hostname: defaultHost,
          });

          appState.signIn({ token });

          await invoke('stop_server');
        } catch (error) {
          console.error('Error:', error);
        } finally {
          processing = false;
        }
      });
    }
  });

  onDestroy(() => {
    unlistenFn();
  });

  const handleSubmit = async () => {
    const { issues, success } = safeParse<LoginSchema>(loginSchema, formData);

    fieldErrors = issues ? flatten<LoginSchema>(issues).nested : undefined;

    if (success) {
      loading = true;
      try {
        await appState.signIn(formData);
      } finally {
        loading = false;
      }
    }
  };
</script>

<div class="flex flex-col gap-2 p-8">
  <div class="flex h-4 flex-row items-center justify-between">
    <Label for="token">Token</Label>
    {#if fieldErrors?.token}
      <p class="text-sm text-red-400 dark:text-red-300">{fieldErrors.token.at(0)}</p>
    {/if}
  </div>
  <Input
    bind:value={token}
    class={cn(fieldErrors?.token && 'border-red-300')}
    placeholder="The 40 characters token generated on GitHub"
  />
  <span class="text-sm">
    To generate a token, go to GitHub,
    <button
      class="cursor-pointer underline hover:text-gray-500 dark:hover:text-gray-300"
      onclick={() =>
        open('https://github.com/settings/tokens/new?description=gitbar&default_expires_at=none&scopes=repo,read:org')}
    >
      personal access tokens
    </button>
  </span>
  <div class="flex h-4 flex-row items-center justify-between">
    <Label for="hostname">Hostname</Label>
    {#if fieldErrors?.hostname}
      <p class="text-sm text-red-400 dark:text-red-300">{fieldErrors.hostname.at(-1)}</p>
    {/if}
  </div>
  <Input bind:value={hostname} class={cn(fieldErrors?.hostname && 'border-red-300')} placeholder="github.company.com" />
  <span class="text-sm">
    Defaults to {defaultHost}. Change only if you are using GitHub for Enterprise.
  </span>
  <div class="flex flex-col items-center gap-1">
    <Button
      variant="default"
      size="sm"
      class={cn('w-full', loading && 'opacity-50')}
      disabled={loading}
      title="Submit"
      onclick={handleSubmit}
      >Submit {#if loading}
        <svg
          class="ml-3 h-4 w-4 animate-spin text-white"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
      {/if}</Button
    >
    OR
    <Button
      variant="secondary"
      size="sm"
      class={cn('w-full', processing && 'opacity-50')}
      disabled={processing}
      onclick={handleToken}
      title="Login via GitHub"
      >Login via GitHub
      {#if processing}
        <svg
          class="ml-3 h-4 w-4 animate-spin text-white"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
      {/if}
    </Button>
  </div>
</div>
