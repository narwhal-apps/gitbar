<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import type { ComponentProps } from 'svelte';
  import SidebarOrg from './SidebarOrg.svelte';
  import SidebarUser from './SidebarUser.svelte';
  import Command from 'lucide-svelte/icons/command';
  import GalleryVerticalEnd from 'lucide-svelte/icons/gallery-vertical-end';
  import AudioWaveform from "lucide-svelte/icons/audio-waveform";
  import { auth } from '../../lib/auth';


  let { ref = $bindable(null), collapsible = "icon", ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();

  const data = {
    user: {
      name: $auth.account?.user?.name!,
      email: $auth.account?.user?.email!,
      avatar: $auth.account?.user?.avatar_url!,
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

<Sidebar.Root bind:ref {...restProps} {collapsible} >
    <Sidebar.Header >
      <SidebarOrg teams={data.teams} />
    </Sidebar.Header>
    <Sidebar.Content>
      <div>
        
      </div>
    </Sidebar.Content>
    <Sidebar.Footer>
      <SidebarUser user={data.user} />
    </Sidebar.Footer>
</Sidebar.Root>
