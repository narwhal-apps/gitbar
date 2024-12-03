import type { AuthTokenOptions, AuthState } from '../../types';
import { getUserData } from '../api';
// import { disable, enable } from '../auto-start';
import { clearState, loadState, saveState } from '../storage';
// import { createURL } from '../url';

// const GITHUB_AUTHORIZE_ENDPOINT = 'https://github.com/login/oauth/authorize';
// const GITHUB_AUTH_SCOPES = ['repo', 'read:user'];

export function createAuthStore() {
  let account = $state<AuthState | undefined>(undefined);

  // Initialize from previous state
  const prevState = loadState();
  if (prevState.account) {
    account = prevState.account;
  }

  async function signIn({ token, hostname = 'github.com' }: AuthTokenOptions): Promise<void> {
    const user = await getUserData(token, hostname);
    if (user) {
      const updatedAccount: AuthState = {
        token,
        hostname,
        user,
      };
      account = updatedAccount;
      saveState(updatedAccount);
    }
  }

  async function signOut(): Promise<void> {
    account = undefined;
    clearState();
  }

  const isAuthenticated = $derived(!!account?.user);

  return {
    get account() {
      return account;
    },
    set account(value: AuthState | undefined) {
      account = value;
    },
    get isAuthenticated() {
      return isAuthenticated;
    },
    signIn,
    signOut,
  };
}

export type AuthStore = ReturnType<typeof createAuthStore>;
