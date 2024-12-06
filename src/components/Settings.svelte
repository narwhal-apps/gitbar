<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion, getName } from '@tauri-apps/api/app';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import Theme from './Theme.svelte';
  import { isEnabled } from '@tauri-apps/plugin-autostart';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';
  import { getGithubContext } from '$lib/stores/contexts';

  interface Props {
    modalVisible: boolean;
  }

  let { modalVisible = $bindable() }: Props = $props();

  let ghCtx = getGithubContext();

  let app = $state({ name: '', version: '' });

  const onSave = () => {
    ghCtx.updateGithubSettings();
    modalVisible = false;
  };
  onMount(() => {
    Promise.all([getName(), getVersion()]).then(values => {
      const [name, version] = values;
      app = { name, version };
    });
    // Not sure why invoke('plugin:autostart|is_enabled') returns false when local storage is set to true
    // Application priviliges in dev perhaps?
    isEnabled().then(active => {
      ghCtx.settings.openAtStartup = active;
    });
  });
</script>

<div class="space-y-6">
  <Theme />
  <!-- <Toggle name="open_at_start" checked={openAtStartup} label="Auto start Gitbar" on:change={changeAutoStart} /> -->
  <div class="flex flex-row items-center justify-between gap-2 rounded-lg border p-4">
    <div class="flex items-center space-x-2">
      <Switch
        id="open_at_start"
        name="open_at_start"
        checked={ghCtx.settings.openAtStartup}
        onCheckedChange={v => (ghCtx.settings.openAtStartup = v)}
      />
      <Label for="open_at_start">Auto start Gitbar</Label>
    </div>
    <div class="flex items-center space-x-2">
      <Switch
        id="compact_mode"
        name="compact_mode"
        checked={ghCtx.settings.isCompactMode}
        onCheckedChange={v => (ghCtx.settings.isCompactMode = v)}
      />
      <Label for="compact_mode">Compact mode</Label>
    </div>
    <!-- <Toggle name="compact_mode" checked={isCompactMode} label="Compact mode" on:change={changeCompactMode} /> -->
  </div>
  <div class="rounded-lg border p-4">
    <label for="fetch_interval" class="mb-4 block text-sm font-bold text-gray-700 dark:text-gray-100">
      Fetch interval <strong>{ghCtx.settings.fetchInterval} sec</strong>
    </label>
    <Slider
      onValueChange={v => (ghCtx.settings.fetchInterval = v[0])}
      value={[ghCtx.settings.fetchInterval]}
      min={5}
      max={60}
      id="fetch_interval"
    />
  </div>
  <div class="relative mt-4 flex items-center justify-between">
    <Badge variant="outline" class="opacity-40">{app.name}@{app.version}</Badge>
    <Button type="button" size="sm" onclick={onSave}>Save</Button>
  </div>
</div>
