<script lang="ts">
	import {
		Dialog,
		DialogContent,
		DialogDescription,
		DialogFooter,
		DialogHeader,
		DialogTitle
	} from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Trash2 } from 'lucide-svelte';
	import { cartStore } from './CartStore.svelte';
	import * as m from '$lib/paraglide/messages.js';

	let isNewOpen = $state(true);

	// Function to handle numeric input
	function handleInput(e: Event) {
		const input = e.target as HTMLInputElement;
		const value = input.value;

		// Only allow numbers and a single decimal point
		const filtered = value.replace(/[^0-9.]/g, '');

		// Ensure only one decimal point
		const parts = filtered.split('.');
		if (parts.length > 2) {
			cartStore.weightInputValue = parts[0] + '.' + parts.slice(1).join('');
		} else {
			cartStore.weightInputValue = filtered;
		}
	}

	// Function to clear the input
	function clearInput() {
		cartStore.weightInputValue = '';
	}

	// Function to handle keypad input
	function appendDigit(digit: string) {
		if (isNewOpen) {
			cartStore.weightInputValue = digit;
			isNewOpen = false;
		} else {
			cartStore.weightInputValue += digit;
		}
	}

	function appendDecimal() {
		if (!cartStore.weightInputValue.includes('.')) {
			cartStore.weightInputValue += '.';
		}
	}

	function backspace() {
		cartStore.weightInputValue = cartStore.weightInputValue.slice(0, -1);
	}
</script>

<Dialog
	open={!!cartStore.editingWeightItem}
	onOpenChange={(open) => !open && cartStore.cancelWeightInput()}
>
	<DialogContent class="w-full sm:max-w-md">
		<DialogHeader>
			<DialogTitle class="text-xl">{m.pos_edit_weight()}</DialogTitle>
			<DialogDescription>
				{#if cartStore.editingWeightItem}
					{m.pos_edit_weight_description({ name: cartStore.editingWeightItem.name })}
				{/if}
			</DialogDescription>
		</DialogHeader>

		<div class="flex flex-col gap-4 py-4">
			<div class="flex items-center gap-2">
				<div class="relative flex-1">
					<Input
						type="text"
						inputmode="decimal"
						value={cartStore.weightInputValue}
						oninput={handleInput}
						class="h-14 pr-12 text-center text-xl font-semibold"
						placeholder={m.pos_edit_weight_placeholder()}
						autocomplete="off"
					/>
					<div class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground">
						{#if cartStore.editingWeightItem?.unit}
							{cartStore.editingWeightItem.unit}
						{/if}
					</div>
				</div>
				<Button variant="outline" size="icon" onclick={clearInput} class="h-14 w-14">
					<Trash2 class="h-5 w-5" />
				</Button>
			</div>

			<!-- Numeric keypad -->
			<div class="grid grid-cols-3 gap-2">
				{#each [1, 2, 3, 4, 5, 6, 7, 8, 9] as digit}
					<Button
						variant="outline"
						class="h-14 text-xl font-medium"
						onclick={() => appendDigit(digit.toString())}
					>
						{digit}
					</Button>
				{/each}
				<Button variant="outline" class="h-14 text-xl font-medium" onclick={appendDecimal}>
					.
				</Button>
				<Button variant="outline" class="h-14 text-xl font-medium" onclick={() => appendDigit('0')}>
					0
				</Button>
				<Button variant="outline" class="h-14 text-xl font-medium" onclick={backspace}>←</Button>
			</div>
		</div>

		<DialogFooter class="flex flex-col flex-col-reverse gap-2 sm:flex-row sm:justify-between">
			<Button variant="outline" onclick={cartStore.cancelWeightInput}>
				{m.pos_button_cancel()}
			</Button>
			<Button
				class="bg-blue-700 text-white hover:bg-blue-800"
				onclick={cartStore.confirmWeightInput}
				disabled={!cartStore.weightInputValue || parseFloat(cartStore.weightInputValue) <= 0}
			>
				{m.pos_button_confirm_weight()}
			</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
