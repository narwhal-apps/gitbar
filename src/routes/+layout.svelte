<script lang="ts">
  import AuthGuard from '../components/AuthGuard.svelte';
  import '../app.css';
  let { children } = $props();
</script>

<AuthGuard>
    {@render children?.()}
</AuthGuard>
