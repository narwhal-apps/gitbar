<script lang="ts">
  import * as Form from '$lib/components/ui/form/index.js';
  import { zodClient } from 'sveltekit-superforms/adapters';
  import { type Infer, superForm, type ValidationErrors } from 'sveltekit-superforms';
  import { loginSchema, type LoginSchema } from './schema';
  import { open } from '@tauri-apps/plugin-shell';
  import { onDestroy, onMount } from 'svelte';
  import { getServerPort } from '$lib/app';
  import { invoke } from '@tauri-apps/api/core';
  import { createAuthURL, getAccessToken } from '$lib/api';
  import { listen } from '@tauri-apps/api/event';
  import { Button } from '$lib/components/ui/button';
  import { cn } from '$lib/utils';
  import { Input } from '$lib/components/ui/input';
  import { getAuthContext } from '$lib/stores/contexts';

  let ghCtx = getAuthContext();

  const defaultHost = 'github.com';
  let loading = $state(false);
  let processing = $state(false);
  let port: number;
  let unlistenFn: () => void;

  const form = superForm(
    {
      token: '',
      hostname: defaultHost,
    },
    {
      validators: zodClient(loginSchema),
    }
  );

  let fieldErrors: ValidationErrors<Infer<LoginSchema>> = $state({});

  function handleToken() {
    open(createAuthURL(port));
  }

  onMount(async () => {
    if (!ghCtx.isAuthenticated) {
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

          ghCtx.signIn({ token });

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

  const { form: formData } = form;

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    const { errors, valid } = await form.validateForm();

    if (valid) {
      loading = true;
      try {
        await ghCtx.signIn(form.capture().data);
      } finally {
        loading = false;
      }
    } else {
      fieldErrors = errors;
    }
  };
</script>

<div class="m-8">
  <form onsubmit={handleSubmit}>
    <Form.Field {form} name="token">
      <Form.Control>
        {#snippet children({ props })}
          <div class="m-0 flex flex-row justify-between">
            <Form.Label class={cn(fieldErrors.token?.length && 'text-destructive')}>Token</Form.Label>
            {#if fieldErrors.token}
              <Form.Label class="text-destructive">{fieldErrors.token.at(-1)}</Form.Label>
            {/if}
          </div>
          <Input
            {...props}
            class={cn(fieldErrors.token?.length && 'border-destructive')}
            placeholder="The 40 characters token generated on GitHub"
            bind:value={$formData.token}
          />
        {/snippet}
      </Form.Control>
      <Form.Description class="pb-1">
        To generate a token, go to GitHub,
        <button
          class="cursor-pointer underline hover:text-gray-500 dark:hover:text-gray-300"
          onclick={() =>
            open(
              'https://github.com/settings/tokens/new?description=gitbar&default_expires_at=none&scopes=repo,read:org'
            )}
        >
          personal access tokens
        </button></Form.Description
      >
    </Form.Field>

    <Form.Field {form} name="hostname">
      <Form.Control>
        {#snippet children({ props })}
          <div class="m-0 flex flex-row justify-between">
            <Form.Label class={cn(fieldErrors.hostname?.length && 'text-destructive')}>Hostname</Form.Label>
            {#if fieldErrors.hostname}
              <Form.Label class="text-destructive">{fieldErrors.hostname.at(-1)}</Form.Label>
            {/if}
          </div>
          <Input
            {...props}
            placeholder="github.company.com"
            bind:value={$formData.hostname}
            class={cn(fieldErrors.hostname?.length && 'border-destructive')}
          />
        {/snippet}
      </Form.Control>
      <Form.Description class="pb-1">
        Defaults to {defaultHost}. Change only if you are using GitHub for Enterprise.
      </Form.Description>
    </Form.Field>

    <div class="flex flex-col items-center gap-1">
      <Button
        variant="default"
        size="sm"
        class={cn('w-full', loading && 'opacity-50')}
        disabled={loading}
        title="Submit"
        type="submit"
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
  </form>
</div>
