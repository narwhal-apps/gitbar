<script lang="ts">
  import Footer from './Footer.svelte';
  import Reviews from './Reviews.svelte';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
</script>

<ScrollArea class="h-[344px]">
  <Reviews />
</ScrollArea>
<Footer />
