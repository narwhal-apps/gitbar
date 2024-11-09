import { invoke } from '@tauri-apps/api/core';

export async function isEnabled(): Promise<boolean> {
  return await invoke('plugin:autostart|is_enabled');
}

export async function enable(): Promise<void> {
  await invoke('plugin:autostart|enable');
}

export async function disable(): Promise<void> {
  await invoke('plugin:autostart|disable');
}
