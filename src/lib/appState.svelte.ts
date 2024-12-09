import { invoke } from '@tauri-apps/api/core';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import { getUserData, getOrganizations, getReviews } from '$lib/api';
import { clearState, saveState } from '$lib/storage';
import type { GithubSettings, SettingsState, AuthState, AuthTokenOptions, ReviewEdge } from '../types/index';
import { defaultSettings, defaultGithubSettings } from './constants';

class AppState {
  private _auth: AuthState | undefined = $state(undefined);
  private _github: GithubSettings = $state(defaultGithubSettings);
  private _reviews: Array<ReviewEdge> = $state([]);
  private _issueCount: number = $state(0);
  private _availableOrgs: { value: string; label: string }[] = $state([]);
  private _theme: 'light' | 'dark' = $state('dark');
  private _isDark: boolean = $derived(this._theme === 'dark');
  private _settings: SettingsState = $state(defaultSettings);
  private _initializing: boolean = $state(true);

  async initialize(initialData: {
    auth?: AuthState;
    github?: GithubSettings;
    reviews?: Array<ReviewEdge>;
    issueCount?: number;
    availableOrgs?: { value: string; label: string }[];
    theme?: 'light' | 'dark';
    settings?: SettingsState;
  }) {
    try {
      this._auth = initialData.auth ?? undefined;
      this._github = initialData.github ?? defaultGithubSettings;
      this._reviews = initialData.reviews ?? [];
      this._issueCount = initialData.reviews?.length ?? 0;
      this._availableOrgs = initialData.availableOrgs ?? [];
      this._theme = initialData.theme ?? 'dark';
      this._settings = initialData.settings ?? defaultSettings;
    } finally {
      this._initializing = false;
    }
  }

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

      if (res.issueCount > this._issueCount) {
        const title = res.edges[0].node.title;
        this.notification(`New review request: ${title}`);
      }

      if (res.issueCount !== this._issueCount) {
        this._issueCount = res.issueCount;
        this._reviews = res.edges;
        invoke('set_review_count', { count: String(res.issueCount) });
      }

      const orgs = await getOrganizations(this._auth);
      this._availableOrgs = orgs.map(org => ({ value: org, label: org }));
      invoke('update_state', { updatedState: this.getState });
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

  get issueCount() {
    return this._issueCount;
  }

  set issueCount(value) {
    this._issueCount = value;
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

  get initializing() {
    return this._initializing;
  }

  get getState() {
    return {
      auth: this._auth,
      github: this._github,
      settings: this._settings,
      issueCount: this._issueCount,
      reviews: this._reviews,
      availableOrgs: this._availableOrgs,
      theme: this._theme,
    };
  }
}

export const appState = new AppState();
