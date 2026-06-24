<!--
  ConfirmDialog — a yes/no confirmation built on the AlertDialog primitive.
  `open` is bindable. `onConfirm` may be async; while it runs the confirm
  button shows a spinner and both actions are disabled. On success the dialog
  closes. The destructive variant styles the confirm button for delete flows.
-->
<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import Spinner from './Spinner.svelte';
	import { cn } from '$lib/utils';

	interface Props {
		open?: boolean;
		title: string;
		description?: string;
		confirmText?: string;
		cancelText?: string;
		variant?: 'default' | 'destructive';
		onConfirm: () => void | Promise<void>;
	}

	let {
		open = $bindable(false),
		title,
		description,
		confirmText = 'Confirm',
		cancelText = 'Cancel',
		variant = 'default',
		onConfirm
	}: Props = $props();

	let pending = $state(false);

	async function handleConfirm() {
		if (pending) return;
		try {
			pending = true;
			await onConfirm();
			open = false;
		} finally {
			pending = false;
		}
	}

	function cancel() {
		if (pending) return;
		open = false;
	}
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{title}</AlertDialog.Title>
			{#if description}
				<AlertDialog.Description>{description}</AlertDialog.Description>
			{/if}
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<Button variant="outline" onclick={cancel} disabled={pending}>
				{cancelText}
			</Button>
			<Button
				variant={variant === 'destructive' ? 'destructive' : 'default'}
				onclick={handleConfirm}
				disabled={pending}
				class={cn(pending && 'pointer-events-none')}
			>
				{#if pending}
					<Spinner class="size-4" />
				{/if}
				{confirmText}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
