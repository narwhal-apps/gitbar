import type { AuthState, GithubSettings, SettingsState } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { github } from './github';
import { disable } from './auto-start';

export const loadState = (): {
  account?: AuthState;
  settings?: SettingsState;
  githubSettings?: GithubSettings;
} => {
  const existing = localStorage.getItem('gitbar-storage');
  const { auth: account, settings, githubSettings } = (existing && JSON.parse(existing)) || {};
  return { account, settings, githubSettings };
};

export const saveState = (account: AuthState, settings: SettingsState, githubSettings: GithubSettings): void => {
  const settingsString = JSON.stringify({ auth: account, settings, githubSettings });
  localStorage.setItem('gitbar-storage', settingsString);
};

export const clearState = (): void => {
  invoke('set_review_count', { count: '' });
  github.update(prev => ({
    ...prev,
    reviews: {
      count: 0,
      data: [],
    },
  }));
  localStorage.clear();
  disable();
};
