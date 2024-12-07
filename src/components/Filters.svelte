<script lang="ts">
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components/ui/button';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';
  import { appState } from '$lib/appState.svelte';

  interface Props {
    modalVisible: boolean;
    onSaved: () => void;
  }

  let { modalVisible = $bindable(), onSaved }: Props = $props();

  const handleOnSave = () => {
    appState.updateGithubSettings();
    onSaved();
    modalVisible = false;
  };

  const typeOptions = [
    { value: 'review-requested', label: 'Reviews' },
    { value: 'author', label: 'Created' },
    { value: 'mentions', label: 'Mentions' },
    { value: 'assignee', label: 'Assigned' },
  ];

  const stateOptions = [
    { value: 'open', label: 'Open' },
    { value: 'closed', label: 'Closed' },
    { value: 'all', label: 'All' },
  ];

  const getTriggerContent = (value: string[] | string, placeholder?: string, multipleMessage?: string) => {
    if (Array.isArray(value)) {
      if (value.length === 0) return placeholder;
      if (value.length === 1) return value[0];
      return value.length + ' ' + multipleMessage;
    } else {
      return [...typeOptions, ...stateOptions].find(option => option.value === value)?.label || placeholder;
    }
  };
</script>

<div class="space-y-6">
  <div class="flex flex-col gap-4">
    <div class="flex items-center space-x-2">
      <Switch
        id="archive"
        name="archive"
        checked={appState.github.archive}
        onCheckedChange={v => (appState.github.archive = v)}
      />
      <Label for="archive">Show archived</Label>
    </div>
    <div class="flex items-center justify-between gap-4">
      <Select.Root
        type="single"
        bind:value={appState.github.type}
        items={typeOptions}
        onValueChange={(value: string) =>
          (appState.github.type = value as 'review-requested' | 'author' | 'mentions' | 'assignee')}
      >
        <Select.Trigger>
          {getTriggerContent(appState.github.type)}
        </Select.Trigger>
        <Select.Content>
          {#each typeOptions as { value, label }}
            <Select.Item {value} {label}>{label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
      <Select.Root
        bind:value={appState.github.state}
        items={stateOptions}
        type="single"
        onValueChange={(value: string) => (appState.github.state = value as 'open' | 'closed' | 'all')}
      >
        <Select.Trigger>
          {getTriggerContent(appState.github.state)}
        </Select.Trigger>
        <Select.Content>
          {#each stateOptions as { value, label }}
            <Select.Item {value} {label}>{label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
    <div class="h-10">
      {#if appState.availableOrgs.length !== 0}
        <Select.Root
          type="multiple"
          bind:value={appState.github.organizations}
          items={appState.availableOrgs}
          onValueChange={v => (appState.github.organizations = v)}
        >
          <Select.Trigger class="w-full">
            {getTriggerContent(appState.github.organizations ?? [], 'Select organizations', 'organizations selected')}
          </Select.Trigger>
          <Select.Content>
            {#each appState.availableOrgs as { value, label }}
              <Select.Item {value} {label}>{label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      {/if}
    </div>
  </div>
  <div class="relative mt-4 flex items-end justify-end">
    <Button type="button" size="sm" onclick={handleOnSave}>Save</Button>
  </div>
</div>
