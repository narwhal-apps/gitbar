import { getContext, setContext } from 'svelte';
import type { AuthStore } from './createAuthStore.svelte';
import type { GithubStore } from './createGithubStore.svelte';

const AUTH_CONTEXT_KEY = Symbol('auth');

export function setAuthContext(store: AuthStore) {
  setContext(AUTH_CONTEXT_KEY, store);
}

export function getAuthContext(): AuthStore {
  const context = getContext<AuthStore>(AUTH_CONTEXT_KEY);

  if (!context) {
    throw new Error('Auth context must be used within an AuthProvider');
  }

  return context;
}

const GITHUB_CONTEXT_KEY = Symbol('github');

export function setGithubContext(store: GithubStore) {
  setContext(GITHUB_CONTEXT_KEY, store);
}

export function getGithubContext(): GithubStore {
  const context = getContext<GithubStore>(GITHUB_CONTEXT_KEY);

  if (!context) {
    throw new Error('Auth context must be used within an AuthProvider');
  }

  return context;
}
