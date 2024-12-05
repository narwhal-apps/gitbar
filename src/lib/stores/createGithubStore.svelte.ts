import type { GithubSettings, Review, SettingsState } from '../../types.ts';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import { getOrganizations, getReviews } from '../api';
import { invoke } from '@tauri-apps/api/core';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { getAuthContext } from './contexts';
import { loadState, saveState } from '$lib/storage';
import { defaultGithubSettings, defaultSettings } from './constants';

const inititalReviews = {
  count: 0,
  data: [] as Review['edges'],
};

type Reviews = typeof inititalReviews;

const initialState = loadState();

export function createGithubStore() {
  const { account } = getAuthContext();
  let settings = $state<SettingsState>(initialState.settings ?? defaultSettings);
  let githubSettings = $state<GithubSettings>(initialState.githubSettings ?? defaultGithubSettings);
  let reviews = $state(inititalReviews);
  let loading = $state(false);
  let availableOrgs = $state<{ value: string; label: string }[]>([]);

  async function notification(text: string) {
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

  async function fetchReviews() {
    if (account) {
      const res = await getReviews(account, githubSettings);

      if (res.issueCount > reviews.count) {
        const title = res.edges[0].node.title;
        notification(`New review request: ${title}`);
      }

      if (res.issueCount !== reviews.count) {
        reviews.count = res.issueCount;
        reviews.data = res.edges;
        invoke('set_review_count', { count: String(res.issueCount) });
      }
    }
  }

  async function fetchInitialReviews() {
    if (account) {
      loading = true;
      await fetchReviews().finally(() => (loading = false));
    }
  }

  $effect(() => {
    const interval = setInterval(() => {
      if (account) {
        fetchReviews();
      }
    }, settings.fetchInterval * 1000);
    return () => clearInterval(interval);
  });

  $effect(() => {
    if (account) {
      getOrganizations(account).then(orgs => {
        availableOrgs = orgs.map(org => ({ value: org, label: org }));
      });
    }
  });

  function updateGithubSettings() {
    const toggle = settings.openAtStartup ? enable : disable;
    toggle();
    saveState(account, settings, githubSettings);
  }

  return {
    get settings() {
      return settings;
    },
    set settings(value: SettingsState) {
      settings = value;
    },
    get githubSettings() {
      return githubSettings;
    },
    set githubSettings(value: GithubSettings) {
      githubSettings = value;
    },
    get reviews() {
      return reviews;
    },
    set reviews(value: Reviews) {
      reviews = value;
    },
    get loading() {
      return loading;
    },
    get availableOrgs() {
      return availableOrgs;
    },
    fetchReviews,
    fetchInitialReviews,
    updateGithubSettings,
  };
}

export type GithubStore = ReturnType<typeof createGithubStore>;
