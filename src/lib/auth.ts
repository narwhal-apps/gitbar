import { writable } from 'svelte/store';
import type { AuthTokenOptions, GithubSettings, SettingsState } from '../types';
import { getUserData } from './api';
import { disable, enable } from './auto-start';
import { clearState, loadState, saveState } from './storage';
import { createURL } from './url';

export const defaultSettings: SettingsState = {
  openAtStartup: false,
  isCompactMode: false,
  fetchInterval: 30000,
};

export const defaultGithubSettings: GithubSettings = {
  archive: false,
  type: 'review-requested',
  state: 'open',
};

const GITHUB_AUTHORIZE_ENDPOINT = 'https://github.com/login/oauth/authorize';
const GITHUB_AUTH_SCOPES = ['repo', 'read:user'];

export function createAuthURL(port: number) {
  const GITHUB_AUTH_QUERIES = {
    client_id: import.meta.env.VITE_CLIENT_ID,
    scope: GITHUB_AUTH_SCOPES.join(' '),
    redirect_uri: `http://localhost:${port}/callback`,
  };

  return createURL({ url: GITHUB_AUTHORIZE_ENDPOINT, query: GITHUB_AUTH_QUERIES });
}

const signIn = async ({ token, hostname }: AuthTokenOptions) => {
  const user = await getUserData(token, hostname);
  if (user) {
    const account = {
      token,
      hostname,
      user,
    };
    auth.update(prevAuth => ({
      ...prevAuth,
      account,
    }));
    saveState(account, defaultSettings, defaultGithubSettings);
  }
};

const signOut = () => {
  auth.update(prevAuth => ({
    ...prevAuth,
    account: undefined,
  }));
  clearState();
};

const updateSettings = (data: SettingsState) => {
  switch (data['openAtStartup'] as boolean) {
    case true:
      enable();
      break;
    case false:
      disable();
  }
  auth.update(prevAuth => {
    const newSettings = {
      ...prevAuth.settings,
      ...data,
    };
    if (prevAuth.account) {
      saveState(prevAuth.account, newSettings, prevAuth.githubSettings);
    }
    return {
      ...prevAuth,
      settings: newSettings,
    };
  });
};

const updateGithubSettings = (data: GithubSettings) => {
  auth.update(prevAuth => {
    const newGHSettings = {
      ...prevAuth.githubSettings,
      ...data,
    };
    if (prevAuth.account) {
      saveState(prevAuth.account, prevAuth.settings, newGHSettings);
    }
    return {
      ...prevAuth,
      githubSettings: newGHSettings,
    };
  });
};

const prevState = loadState();

export const auth = writable({
  signIn,
  signOut,
  account: prevState.account,
  settings: prevState.settings || defaultSettings,
  githubSettings: prevState.githubSettings || defaultGithubSettings,
  updateSettings,
  updateGithubSettings,
});
