import type { GithubSettings, SettingsState } from '../../types.ts';

export const defaultSettings: SettingsState = {
  openAtStartup: false,
  isCompactMode: false,
  fetchInterval: 30,
} as const;

export const defaultGithubSettings: GithubSettings = {
  archive: false,
  type: 'review-requested',
  state: 'open',
} as const;

export const AUTH_CONTEXT_KEY = Symbol('auth');
