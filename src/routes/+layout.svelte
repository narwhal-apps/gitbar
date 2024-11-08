<script lang="ts">
  import AuthGuard from '../components/AuthGuard.svelte';
  // import * as Sidebar from '$lib/components/ui/sidebar';
  // import AppSidebar from '../components/experiments/AppSidebar.svelte';
  import '../app.css';
  let { children } = $props();
  // let open = $state(false);
</script>

<AuthGuard>
    {@render children?.()}
</AuthGuard>
<!-- <AuthGuard>
  <Sidebar.Provider controlledOpen {open} onOpenChange={(value) => (open = value)}>
    <AppSidebar />
    <Sidebar.Inset>
      <Sidebar.Rail data-open={open} class="h-full w-2 pl-2 data-[open='true']:cursor-w-resize data-[open='false']:cursor-e-resize" onclick={() => open = !open}  />
        {@render children?.()}
      </Sidebar.Inset>
  </Sidebar.Provider>
</AuthGuard> -->
