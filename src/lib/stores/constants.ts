import type { GithubSettings, SettingsState } from '../../types';

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
