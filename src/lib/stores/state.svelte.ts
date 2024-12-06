import { getUserData } from '$lib/api';
import { clearState, loadState, saveState } from '$lib/storage';
import type { GithubSettings, SettingsState, AuthState, AuthTokenOptions } from '../../types';
import { defaultSettings, defaultGithubSettings } from './constants';

const initialState = loadState();

type GlobalState = {
  auth: AuthState | undefined;
  isAuthenticated: boolean;
  github: GithubSettings;
  theme: unknown;
  isDark: boolean;
  settings: SettingsState;
  signIn: (options: AuthTokenOptions) => Promise<void>;
  signOut: () => Promise<void>;
};

let auth = $state<AuthState | undefined>(initialState.account ?? undefined);
const isAuthenticated = $derived(!!auth?.user);

const github = $state<GithubSettings>(initialState.githubSettings ?? defaultGithubSettings);

const theme = $state<'light' | 'dark'>('dark');
const isDark = $derived(theme === 'dark');

const settings = $state<SettingsState>(initialState.settings ?? defaultSettings);

async function signIn({ token, hostname = 'github.com' }: AuthTokenOptions): Promise<void> {
  const user = await getUserData(token, hostname);
  if (user) {
    const acc: AuthState = {
      token,
      hostname,
      user,
    };
    auth = acc;
    saveState(acc);
  }
}

async function signOut(): Promise<void> {
  auth = undefined;
  clearState();
}

export const appState = $state<GlobalState>({
  auth,
  isAuthenticated,
  github,
  theme,
  isDark,
  settings,
  signIn,
  signOut,
});
