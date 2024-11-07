<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import type { ComponentProps } from 'svelte';
  import SidebarOrg from './SidebarOrg.svelte';
  import SidebarUser from './SidebarUser.svelte';
  import Command from 'lucide-svelte/icons/command';
  import GalleryVerticalEnd from 'lucide-svelte/icons/gallery-vertical-end';
  import AudioWaveform from "lucide-svelte/icons/audio-waveform";


  let { ref = $bindable(null), collapsible = 'icon', ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();

  const data = {
    user: {
      name: 'shadcn',
      email: 'm@example.com',
      avatar: '/avatars/shadcn.jpg',
    },
    teams: [
      {
        name: 'Acme Inc',
        logo: GalleryVerticalEnd,
        plan: 'Enterprise',
      },
      {
        name: 'Acme Corp.',
        logo: AudioWaveform,
        plan: 'Startup',
      },
      {
        name: 'Evil Corp.',
        logo: Command,
        plan: 'Free',
      },
    ],
  };
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
  <Sidebar.Header>
    <SidebarOrg teams={data.teams} />
  </Sidebar.Header>
  <Sidebar.Content>
    
  </Sidebar.Content>
	<Sidebar.Footer>
		<SidebarUser user={data.user} />
	</Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
