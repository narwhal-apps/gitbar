import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import type { GithubSettings, SettingsState, AuthState, Review, AppState } from '../types/index';

class StateManager {
  private _auth: AuthState | null = $state(null);
  private _isLoggedIn = $derived<boolean>(this._auth?.user !== undefined);
  private _github: GithubSettings | null = $state(null);
  private _reviews: Array<Review> = $state([]);
  private _issueCount: number = $state(0);
  private _availableOrgs: { value: string; label: string }[] = $state([]);
  private _theme: 'light' | 'dark' = $state('dark');
  private _isDark: boolean = $derived(this._theme === 'dark');
  private _settings: SettingsState | null = $state(null);

  async initialize(initialData: AppState) {
    this._auth = initialData.auth;
    this._github = initialData.github;
    this._reviews = initialData.reviews;
    this._issueCount = initialData.issueCount;
    this._availableOrgs = initialData.availableOrgs;
    this._theme = initialData.theme;
    this._settings = initialData.settings;
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

  updateGithubSettings(): void {
    const toggle = this._settings.openAtStartup ? enable : disable;
    toggle();
    // saveState(this._auth, this._settings, this._github);
  }

  toggleTheme(): void {
    this._theme = this.isDark ? 'light' : 'dark';
  }

  get isAuthenticated() {
    return !!this._auth?.user;
  }

  get isLoggedIn() {
    return this._isLoggedIn;
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
}

export const appState = new StateManager();
