import type { AuthState, GithubSettings, SettingsState } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { disable } from './auto-start';
import { defaultGithubSettings, defaultSettings } from './stores/constants';

export const loadState = (): {
  account?: AuthState;
  settings?: SettingsState;
  githubSettings?: GithubSettings;
} => {
  const existing = localStorage.getItem('gitbar-storage');
  const { auth: account, settings, githubSettings } = (existing && JSON.parse(existing)) || {};
  return { account, settings, githubSettings };
};

export const saveState = (
  account: AuthState | undefined,
  settings: SettingsState | undefined = defaultSettings,
  githubSettings: GithubSettings | undefined = defaultGithubSettings,
): void => {
  const settingsString = JSON.stringify({ auth: account, settings, githubSettings });
  localStorage.setItem('gitbar-storage', settingsString);
};

export const clearState = (): void => {
  invoke('set_review_count', { count: '' });
  localStorage.clear();
  disable();
};
