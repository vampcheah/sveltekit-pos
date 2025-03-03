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
	import { X, Trash2 } from 'lucide-svelte';

	let {
		editingWeightItem = $bindable(),
		weightInputValue = $bindable(),
		onConfirm,
		onCancel
	} = $props();

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
			weightInputValue = parts[0] + '.' + parts.slice(1).join('');
		} else {
			weightInputValue = filtered;
		}
	}

	// Function to clear the input
	function clearInput() {
		weightInputValue = '';
	}

	// Function to handle keypad input
	function appendDigit(digit: string) {
		if (isNewOpen) {
			weightInputValue = digit;
			isNewOpen = false;
		} else {
			weightInputValue += digit;
		}
	}

	function appendDecimal() {
		if (!weightInputValue.includes('.')) {
			weightInputValue += '.';
		}
	}

	function backspace() {
		weightInputValue = weightInputValue.slice(0, -1);
	}
</script>

<Dialog open={!!editingWeightItem} onOpenChange={(open) => !open && onCancel()}>
	<DialogContent class="w-full sm:max-w-md">
		<DialogHeader>
			<DialogTitle class="text-xl">Enter Weight</DialogTitle>
			<DialogDescription>
				{#if editingWeightItem}
					Please enter the weight for {editingWeightItem.name}
				{/if}
			</DialogDescription>
		</DialogHeader>

		<div class="flex flex-col gap-4 py-4">
			<div class="flex items-center gap-2">
				<div class="relative flex-1">
					<Input
						type="text"
						inputmode="decimal"
						bind:value={weightInputValue}
						oninput={handleInput}
						class="h-14 pr-12 text-center text-xl font-semibold"
						placeholder="0.00"
						autocomplete="off"
					/>
					<div class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground">
						{#if editingWeightItem?.unit}
							{editingWeightItem.unit}
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
			<Button variant="outline" onclick={onCancel}>Cancel</Button>
			<Button
				class="bg-blue-700 text-white hover:bg-blue-800"
				onclick={onConfirm}
				disabled={!weightInputValue || parseFloat(weightInputValue) <= 0}
			>
				Confirm
			</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
