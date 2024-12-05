import { getContext, setContext } from 'svelte';
import type { AuthStore } from './createAuthStore.svelte';
import type { GithubStore } from './createGithubStore.svelte';
import type { ThemeStore } from './createThemeStore.svelte';

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
    throw new Error('Github context must be used within an GithubProvider');
  }

  return context;
}

const THEME_CONTEXT_KEY = Symbol('theme');

export function setThemeContext(store: ThemeStore) {
  setContext(THEME_CONTEXT_KEY, store);
}

export function getThemeContext(): ThemeStore {
  const context = getContext<ThemeStore>(THEME_CONTEXT_KEY);

  if (!context) {
    throw new Error('Theme context must be used within an ThemeProvider');
  }

  return context;
}
