<script lang="ts">
  import { open } from '@tauri-apps/api/shell';
  import { Validators, type ValidatorFn, type ValidatorResult } from '../lib/validators';
  import { auth, createAuthURL, defaultGithubSettings, defaultSettings } from '../lib/auth';
  import { onDestroy, onMount } from 'svelte';
  import { getServerPort } from '../lib/app';
  import { invoke } from '@tauri-apps/api/tauri';
  import { getAccessToken, getUserData } from '../lib/api';
  import { listen } from '@tauri-apps/api/event';
  import { saveState } from '../lib/storage';
  import { Button } from '$lib/components/ui/button';
  import { cn } from '$lib/utils';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';

  const defaultHost = 'github.com';
  let errors: { [inputName: string]: ValidatorResult } = {};
  let loading = false;
  let processing = false;
  let port: number;
  let unlistenFn: () => void;

  let form: {
    [inputName: string]: {
      validators: ValidatorFn[];
    };
  } = {
    token: {
      validators: [Validators.required],
    },
    hostname: {
      validators: [Validators.required],
    },
  };

  function isFormValid(): boolean {
    return !Object.values(errors).some(field => Object.values(field).some(errorObject => errorObject.error));
  }

  function validateForm(data: { [inputName: string]: any }): void {
    Object.keys(data).forEach(field => validateField(field, data[field]));
  }

  function validateField(field: string, value: string) {
    form[field]?.validators &&
      form[field].validators.forEach(fn => {
        const error = fn(value);
        errors[field] = { ...errors[field], ...error };
      });
  }

  function onChange(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
    validateField(e.currentTarget.name, e.currentTarget.value);
  }

  function onSubmit(e: SubmitEvent & { currentTarget: HTMLFormElement }) {
    const formData = new FormData(e.currentTarget);

    const data: any = {};
    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    validateForm(data);

    if (isFormValid()) {
      loading = true;
      $auth.signIn(data).finally(() => (loading = false));
    } else {
      console.log('Invalid Form');
    }
  }

  function handleToken() {
    open(createAuthURL(port));
  }

  onMount(async () => {
    await invoke('start_server');
    port = await getServerPort();
    unlistenFn = await listen('code', async (event: { payload: string }) => {
      processing = true;
      try {
        const {
          data: { access_token },
        } = await getAccessToken({
          clientId: import.meta.env.VITE_CLIENT_ID,
          clientSecret: import.meta.env.VITE_CLIENT_SECRET,
          code: event.payload,
          hostname: defaultHost,
        });

        const user = await getUserData(access_token, defaultHost);
        if (user) {
          const account = {
            token: access_token,
            hostname: defaultHost,
            user,
          };
          auth.update(prevAuth => ({
            ...prevAuth,
            account,
          }));
          saveState(account, defaultSettings, defaultGithubSettings);
        }
        await invoke('stop_server');
      } finally {
        processing = false;
      }
    });
  });

  onDestroy(() => {
    unlistenFn();
  });
</script>

<div class="m-8">
  <form on:submit|preventDefault={onSubmit}>
    <div class="pb-2">
      <Label for="token">Token</Label>
      <Input
        type="text"
        name="token"
        id="token"
        on:input={onChange}
        placeholder="The 40 characters token generated on GitHub"
        class="w-full"
      />
      {#if errors?.token?.required?.error}
        <p class="text-red-400 dark:text-red-300 text-sm">Token is required</p>
      {/if}
      <span class="text-sm">
        To generate a token, go to GitHub,
        <button
          class="underline hover:text-gray-500 dark:hover:text-gray-300 cursor-pointer"
          on:click={() =>
            open(
              'https://github.com/settings/tokens/new?description=gitbar&default_expires_at=none&scopes=repo,read:org'
            )}
        >
          personal access tokens
        </button>
      </span>
    </div>
    <div class="pb-2">
      <Label for="hostname">Hostname</Label>
      <Input
        type="text"
        name="hostname"
        placeholder="github.company.com"
        id="hostname"
        value={defaultHost}
        on:input={onChange}
        class="w-full"
      />
      {#if errors?.hostname?.required?.error}
        <p class="text-red-400 dark:text-red-300">Password is required</p>
      {/if}
      <span class="text-sm">
        Defaults to {defaultHost}. Change only if you are using GitHub for Enterprise.
      </span>
    </div>
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
            class="animate-spin h-4 w-4 text-white ml-3"
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
        on:click={handleToken}
        title="Login via GitHub"
        >Login via GitHub
        {#if processing}
          <svg
            class="animate-spin h-4 w-4 text-white ml-3"
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
