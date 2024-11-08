<script lang="ts">
  import { cn } from '$lib/utils.js';
  import type { WithElementRef } from 'bits-ui';
  import type { HTMLAttributes } from 'svelte/elements';

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLElement>> = $props();
</script>

<main
  bind:this={ref}
  class={cn(
    'relative flex min-h-svh flex-1 flex-col bg-background',
    'peer-data-[variant=inset]:m-2 peer-data-[state=collapsed]:peer-data-[variant=inset]:ml-2 peer-data-[variant=inset]:ml-0 peer-data-[variant=inset]:rounded-xl peer-data-[variant=inset]:shadow',
    className
  )}
  {...restProps}
>
  {@render children?.()}
</main>
