import { writable } from 'svelte/store';
import type { AuthState, GithubSettings, Review } from '../types';
import { getReviews } from './api';
import { invoke } from '@tauri-apps/api/core';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

const reviews = {
  count: 0,
  data: [] as Review['edges'],
};

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

const fetchReviews = async (account: AuthState, githubSettings: GithubSettings) => {
  const res = await getReviews(account, githubSettings);
  let prevCount: number = 0;
  github.subscribe(({ reviews }) => (prevCount = reviews.count));

  if (res.issueCount > prevCount) {
    const title = res.edges[0].node.title;
    const author = res.edges[0].node.author.login;
    notification(`${title} - @${author}`);
  }
  if (res.issueCount !== prevCount) {
    github.update(prev => ({
      ...prev,
      reviews: {
        count: res.issueCount,
        data: res.edges,
      },
    }));
    invoke('set_review_count', { count: String(res.issueCount) });
  }
};

export const github = writable({
  reviews,
  fetchReviews,
});
