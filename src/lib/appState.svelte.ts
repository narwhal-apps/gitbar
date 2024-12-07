import { invoke } from '@tauri-apps/api/core';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import { getUserData, getOrganizations, getReviews } from '$lib/api';
import { clearState, loadState, saveState } from '$lib/storage';
import type { GithubSettings, SettingsState, AuthState, AuthTokenOptions, Review } from '../types';
import { defaultSettings, defaultGithubSettings } from './constants';

const initialState = loadState();

class AppState {
  private _auth: AuthState | undefined = $state(initialState.account ?? undefined);
  private _github: GithubSettings = $state(initialState.githubSettings ?? defaultGithubSettings);
  private _reviews: Review['edges'] = $state([]);
  private _reviewCount: number = $state(0);
  private _availableOrgs: { value: string; label: string }[] = $state([]);
  private _theme: 'light' | 'dark' = $state('dark');
  private _isDark: boolean = $derived(this._theme === 'dark');
  private _settings: SettingsState = $state(initialState.settings ?? defaultSettings);

  async signIn({ token, hostname = 'github.com' }: AuthTokenOptions): Promise<void> {
    const user = await getUserData(token, hostname);
    if (user) {
      const acc: AuthState = {
        token,
        hostname,
        user,
      };
      this._auth = acc;
      saveState(acc);
    }
  }

  async signOut(): Promise<void> {
    this._auth = undefined;
    clearState();
  }

  async notification(text: string): Promise<void> {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification({
        title: 'GitBar',
        body: text,
      });
    }
  }

  async fetchReviews(): Promise<void> {
    if (this._auth) {
      const res = await getReviews(this._auth, this._github);

      if (res.issueCount > this._reviewCount) {
        const title = res.edges[0].node.title;
        this.notification(`New review request: ${title}`);
      }

      if (res.issueCount !== this._reviewCount) {
        this._reviewCount = res.issueCount;
        this._reviews = res.edges;
        invoke('set_review_count', { count: String(res.issueCount) });
      }

      const orgs = await getOrganizations(this._auth);
      this._availableOrgs = orgs.map(org => ({ value: org, label: org }));
    }
  }

  updateGithubSettings(): void {
    const toggle = this._settings.openAtStartup ? enable : disable;
    toggle();
    saveState(this._auth, this._settings, this._github);
  }

  toggleTheme(): void {
    this._theme = this.isDark ? 'light' : 'dark';
  }

  get isAuthenticated() {
    return !!this._auth?.user;
  }

  get isDark() {
    return this._isDark;
  }

  get auth() {
    return this._auth;
  }

  set auth(value) {
    this._auth = value;
  }

  get github() {
    return this._github;
  }

  set github(value) {
    this._github = value;
  }

  get reviews() {
    return this._reviews;
  }

  set reviews(value) {
    this._reviews = value;
  }

  get reviewCount() {
    return this._reviewCount;
  }

  set reviewCount(value) {
    this._reviewCount = value;
  }

  get availableOrgs() {
    return this._availableOrgs;
  }

  set availableOrgs(value) {
    this._availableOrgs = value;
  }

  get theme() {
    return this._theme;
  }

  set theme(value) {
    this._theme = value;
  }

  get settings() {
    return this._settings;
  }

  set settings(value) {
    this._settings = value;
  }
}

export const appState = new AppState();
