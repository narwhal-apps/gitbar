import type { AuthTokenOptions, GithubSettings, SettingsState, Account } from '../types';
import { getUserData } from './api';
import { disable, enable } from './auto-start';
import { clearState, loadState, saveState } from './storage';
import { createURL } from './url';

const GITHUB_AUTHORIZE_ENDPOINT = 'https://github.com/login/oauth/authorize';
const GITHUB_AUTH_SCOPES = ['repo', 'read:user'];

export class AuthStore {
  readonly defaultSettings: SettingsState = {
    openAtStartup: false,
    isCompactMode: false,
    fetchInterval: 30000,
  };

  readonly defaultGithubSettings: GithubSettings = {
    archive: false,
    type: 'review-requested',
    state: 'open',
  };

  account = $state<Account | undefined>(undefined);
  settings = $state<SettingsState>(this.defaultSettings);
  githubSettings = $state<GithubSettings>(this.defaultGithubSettings);

  constructor() {
    const prevState = loadState();
    if (prevState.account) {
      this.account = prevState.account;
    }
    if (prevState.settings) {
      this.settings = prevState.settings;
    }
    if (prevState.githubSettings) {
      this.githubSettings = prevState.githubSettings;
    }
  }


  createAuthURL(port: number): string {
    const GITHUB_AUTH_QUERIES = {
      client_id: import.meta.env.VITE_CLIENT_ID,
      scope: GITHUB_AUTH_SCOPES.join(' '),
      redirect_uri: `http://localhost:${port}/callback`,
    };

    return createURL({ url: GITHUB_AUTHORIZE_ENDPOINT, query: GITHUB_AUTH_QUERIES });
  }

  async signIn({ token, hostname }: AuthTokenOptions): Promise<void> {
    const user = await getUserData(token, hostname);
    if (user) {
      this.account = {
        token,
        hostname,
        user,
      };
      saveState(this.account, this.settings, this.githubSettings);
    }
  }

  signOut(): void {
    this.account = undefined;
    clearState();
  }

  updateSettings(data: Partial<SettingsState>): void {
    if ('openAtStartup' in data) {
      data.openAtStartup ? enable() : disable();
    }

    this.settings = {
      ...this.settings,
      ...data,
    };
    
    if (this.account) {
      saveState(this.account, this.settings, this.githubSettings);
    }
  }

  updateGithubSettings(data: Partial<GithubSettings>): void {
    this.githubSettings = {
      ...this.githubSettings,
      ...data,
    };
    
    if (this.account) {
      saveState(this.account, this.settings, this.githubSettings);
    }
  }

  get isAuthenticated(): boolean {
    return Boolean(this.account);
  }
}

// Create a singleton instance
export const authStore = new AuthStore();

