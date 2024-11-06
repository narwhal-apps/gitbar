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
  let state = $state($auth.githubSettings.state);
  let type = $state($auth.githubSettings.type);
  let selectedOrganizations = $state($auth.githubSettings.organizations || []);
  let organizationOptions: Array<{ value: string; label: string }> = $state([]);
  let loadingOrganizations = $state(false);

  const changeShowArchive = (checked: boolean) => {
    showArchive = checked;
  };

  const onSave = () => {
    $auth.updateGithubSettings({
      archive: showArchive,
      state,
      type,
      organizations: selectedOrganizations,
    });
    modalVisible = false;
    onSaved();
  };

  const updateSelectedOrganizations = (target: any) => {
    const incoming = target as { value: string; label: string }[];
    selectedOrganizations = incoming.map(i => i.value);
  };

  const updateType = (target: any) => (type = target.value);
  const updateState = (target: any) => (state = target.value);

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
      <Select.Root items={typeOptions} selected={typeOptions.find(v => v.value === type)} onSelectedChange={updateType}>
        <Select.Trigger>
          <Select.Value placeholder="Type" />
        </Select.Trigger>
        <Select.Content>
          {#each typeOptions as { value, label }}
            <Select.Item {value} {label}>{label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
      <Select.Root
        items={stateOptions}
        selected={stateOptions.find(v => v.value === state)}
        onSelectedChange={updateState}
      >
        <Select.Trigger>
          <Select.Value placeholder="State" />
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
          items={organizationOptions}
          selected={selectedOrganizations.map(v => ({ value: v, label: v }))}
          multiple
          onSelectedChange={updateSelectedOrganizations}
        >
          <Select.Trigger class="w-full">
            <Select.Value placeholder="Organizations" />
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
    <Button type="button" size="sm" on:click={onSave}>Save</Button>
  </div>
</div>
