<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion, getName } from '@tauri-apps/api/app';
  import { auth, defaultSettings } from '../lib/auth';
  import Range from './Range.svelte';
  import Theme from './Theme.svelte';
  import { isEnabled } from '../lib/auto-start';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';

  interface Props {
    modalVisible: boolean;
  }

  let { modalVisible = $bindable() }: Props = $props();
  let openAtStartup = $state($auth.settings?.openAtStartup);
  let isCompactMode = $state($auth.settings?.isCompactMode);
  let fetchInterval = $state(($auth.settings.fetchInterval || defaultSettings.fetchInterval) / 1000);
  let app = $state({ name: '', version: '' });

  const changeAutoStart = (checked: boolean) => {
    openAtStartup = checked;
  };
  const changeCompactMode = (checked: boolean) => {
    isCompactMode = checked;
  };

  const onSave = () => {
    $auth.updateSettings({
      openAtStartup,
      isCompactMode,
      fetchInterval: fetchInterval * 1000,
    });
    modalVisible = false;
  };
  onMount(() => {
    Promise.all([getName(), getVersion()]).then(values => {
      const [name, version] = values;
      app = { name, version };
    });
    isEnabled().then(active => {
      openAtStartup = active;
    });
  });
</script>

<div class="space-y-6">
  <Theme />
  <!-- <Toggle name="open_at_start" checked={openAtStartup} label="Auto start Gitbar" on:change={changeAutoStart} /> -->
  <div class="flex flex-row gap-2 items-center justify-between rounded-lg border p-4">
    <div class="flex items-center space-x-2">
      <Switch id="open_at_start" name="open_at_start" checked={openAtStartup} onCheckedChange={changeAutoStart} />
      <Label for="open_at_start">Auto start Gitbar</Label>
    </div>
    <div class="flex items-center space-x-2">
      <Switch id="compact_mode" name="compact_mode" checked={isCompactMode} onCheckedChange={changeCompactMode} />
      <Label for="compact_mode">Compact mode</Label>
    </div>
    <!-- <Toggle name="compact_mode" checked={isCompactMode} label="Compact mode" on:change={changeCompactMode} /> -->
  </div>
  <div class="rounded-lg border p-4">
    <label for="fetch_interval" class="block text-sm font-bold text-gray-700 dark:text-gray-100 mb-4">
      Fetch interval <strong>{fetchInterval} sec</strong>
    </label>
    <Range
      on:change={e => (fetchInterval = e.detail.value)}
      initialValue={fetchInterval}
      min={5}
      max={60}
      id="fetch_interval"
    />
  </div>
  <div class="relative mt-4 flex items-center justify-between">
    <Badge variant="outline" class="opacity-40">{app.name}@{app.version}</Badge>
    <Button type="button" size="sm" on:click={onSave}>Save</Button>
  </div>
</div>
