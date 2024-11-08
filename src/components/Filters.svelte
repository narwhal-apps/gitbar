<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '../lib/auth';
  import { getOrganizations } from '../lib/api';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components/ui/button';
  import { Switch } from '$lib/components/ui/switch';
  import { Label } from '$lib/components/ui/label';
  import { Skeleton } from '$lib/components/ui/skeleton';

  interface Props {
    modalVisible: boolean;
    onSaved: () => void;
  }

  let { modalVisible = $bindable(), onSaved }: Props = $props();
  let showArchive = $state($auth.githubSettings.archive);
  let ghState = $state<string>($auth.githubSettings.state || 'open');
  let ghType = $state<string>($auth.githubSettings.type || 'review-requested');
  let selectedOrganizations = $state<string[]>($auth.githubSettings.organizations || []);
  let organizationOptions: Array<{ value: string; label: string }> = $state([]);
  let loadingOrganizations = $state(false);

  const changeShowArchive = (checked: boolean) => {
    showArchive = checked;
  };

  const onSave = () => {
    $auth.updateGithubSettings({
      archive: showArchive,
      state: ghState as typeof $auth.githubSettings.state,
      type: ghType as typeof $auth.githubSettings.type,
      organizations: selectedOrganizations,
    });
    modalVisible = false;
    onSaved();
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

  onMount(() => {
    if (!$auth.account) return;
    loadingOrganizations = true;
    getOrganizations($auth.account)
      .then(orgs => {
        organizationOptions = orgs.map(org => ({ value: org, label: org }));
      })
      .finally(() => {
        loadingOrganizations = false;
      });
  });
</script>

<div class="space-y-6">
  <div class="flex flex-col gap-4">
    <div class="flex items-center space-x-2">
      <Switch id="archive" name="archive" checked={showArchive} onCheckedChange={changeShowArchive} />
      <Label for="archive">Show archived</Label>
    </div>
    <div class="flex items-center justify-between gap-4">
      <Select.Root type="single" items={typeOptions} onValueChange={v => (ghType = v)}>
        <Select.Trigger>
          {getTriggerContent(ghType)}
        </Select.Trigger>
        <Select.Content>
          {#each typeOptions as { value, label }}
            <Select.Item {value} {label}>{label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
      <Select.Root items={stateOptions} type="single" onValueChange={v => (ghState = v)}>
        <Select.Trigger>
          {getTriggerContent(ghState)}
        </Select.Trigger>
        <Select.Content>
          {#each stateOptions as { value, label }}
            <Select.Item {value} {label}>{label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
    <div class="h-10">
      {#if loadingOrganizations}
        <Skeleton class="h-10 w-full" />
      {:else if organizationOptions.length !== 0}
        <Select.Root
          type="multiple"
          items={organizationOptions}
          bind:value={selectedOrganizations}
          onValueChange={v => (selectedOrganizations = v)}
        >
          <Select.Trigger class="w-full">
            {getTriggerContent(selectedOrganizations, 'Select organizations', 'organizations selected')}
          </Select.Trigger>
          <Select.Content>
            {#each organizationOptions as { value, label }}
              <Select.Item {value} {label}>{label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      {/if}
    </div>
  </div>
  <div class="relative mt-4 flex items-end justify-end">
    <Button type="button" size="sm" onclick={onSave}>Save</Button>
  </div>
</div>
