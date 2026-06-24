<script lang="ts">
	import { cn } from '$lib/utils';

	interface Props {
		checked?: boolean;
		disabled?: boolean;
		onCheckedChange?: (checked: boolean) => void;
		class?: string;
	}

	let { checked = false, disabled = false, onCheckedChange, class: className }: Props = $props();

	function handleClick() {
		if (disabled) return;
		const newValue = !checked;
		checked = newValue;
		onCheckedChange?.(newValue);
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleClick();
		}
	}
</script>

<button
	type="button"
	role="switch"
	aria-checked={checked}
	aria-label="Toggle switch"
	{disabled}
	class={cn(
		'peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors',
		'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background',
		'disabled:cursor-not-allowed disabled:opacity-50',
		checked ? 'bg-primary' : 'bg-input',
		className
	)}
	onclick={handleClick}
	onkeydown={handleKeyDown}
>
	<span
		class={cn(
			'pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform',
			checked ? 'translate-x-5' : 'translate-x-0'
		)}
	></span>
</button>
