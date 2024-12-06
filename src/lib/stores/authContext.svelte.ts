import type { AuthState, AuthTokenOptions } from '../../types.ts';
import { getUserData } from '../api';
import { clearState, saveState } from '../storage';

export function initContext() {
  let account = $state<AuthState | undefined>(undefined);
  const isAuthenticated = $derived(!!account?.user);

  async function signIn({ token, hostname = 'github.com' }: AuthTokenOptions): Promise<void> {
    const user = await getUserData(token, hostname);
    if (user) {
      const acc: AuthState = {
        token,
        hostname,
        user,
      };
      account = acc;
      saveState(acc);
    }
  }

  async function signOut(): Promise<void> {
    account = undefined;
    clearState();
  }
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

export type AuthContext = ReturnType<typeof initContext>;
