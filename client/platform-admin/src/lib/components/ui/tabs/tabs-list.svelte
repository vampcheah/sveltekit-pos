<script lang="ts">
	import { Tabs as TabsPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	let { ref = $bindable(null), class: className, ...restProps }: TabsPrimitive.ListProps = $props();
</script>

<TabsPrimitive.List
	bind:ref
	data-slot="tabs-list"
	class={cn(
		'bg-muted text-muted-foreground grid w-full auto-cols-fr grid-flow-col gap-1 rounded-lg p-1',
		className
	)}
	{...restProps}
/>
