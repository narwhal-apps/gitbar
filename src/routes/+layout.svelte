<script lang="ts">
  import AuthGuard from '../components/AuthGuard.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { Separator } from '$lib/components/ui/separator';
  import AppSidebar from '../components/AppSidebar.svelte';
  import '../app.css';
  let { children } = $props();
</script>

<AuthGuard>
  <Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset>
      <header
        class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
      >
        <div class="flex items-center gap-2 px-4">
          <Sidebar.Trigger class="-ml-1" />
          <Separator orientation="vertical" class="mr-2 h-4" />
        </div>
      </header>
      {@render children?.()}
    </Sidebar.Inset>
  </Sidebar.Provider>
</AuthGuard>
