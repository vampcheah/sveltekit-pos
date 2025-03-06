<script lang="ts">
	import {
		Dialog,
		DialogContent,
		DialogHeader,
		DialogTitle,
		DialogFooter
	} from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { RadioGroup, RadioGroupItem } from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';
	import { CreditCard, Banknote, Printer, Check } from 'lucide-svelte';
	import { cartStore } from './CartStore.svelte';

	// Props
	let { onClose } = $props();

	// States
	let selectedPaymentMethod = $state('cash');
	let isProcessing = $state(false);
	let isCompleted = $state(false);
	let paidAmount = $state(0);

	// Payment methods with enhanced metadata
	const paymentMethods = [
		{
			id: 'cash',
			name: 'Cash',
			icon: Banknote,
			description: 'Pay with physical cash'
		},
		{
			id: 'credit_card',
			name: 'Credit Card',
			icon: CreditCard,
			description: 'Debit or credit card payment'
		},
		{
			id: 'bsc_usdt',
			name: 'Crypto',
			icon: Banknote,
			description: 'Pay with BSC-USDT'
		}
	];

	const processPayment = async () => {
		isProcessing = true;
		paidAmount = cartStore.total;
		await new Promise((resolve) => setTimeout(resolve, 200));
		cartStore.checkout();
		isProcessing = false;
		isCompleted = true;
	};

	const printReceipt = () => {
		toast.success('Receipt printed successfully!');
	};

	const completeCheckout = () => {
		onClose();
	};
</script>

<Dialog open={true} onOpenChange={onClose}>
	<DialogContent class="sm:max-w-xl">
		<DialogHeader>
			<DialogTitle class="text-2xl font-bold">Complete Purchase</DialogTitle>
		</DialogHeader>

		{#if !isCompleted}
			<div class="py-6">
				<div class="mb-8 rounded-lg bg-blue-50/10 p-4">
					<div class="mb-1 text-sm text-muted-foreground">Total Amount</div>
					<div class="text-3xl font-bold text-green-500">{cartStore.total.toFixed(2)}</div>
				</div>

				<div class="space-y-4">
					<h3 class="text-lg font-medium">Select Payment Method</h3>

					<RadioGroup value={selectedPaymentMethod} class="grid gap-4 md:grid-cols-3">
						{#each paymentMethods as method}
							<Button
								class="relative flex h-auto flex-col items-start gap-2 rounded-xl border bg-card p-4 text-left transition-all hover:bg-muted
								{selectedPaymentMethod === method.id
									? 'border-blue-500 bg-blue-500/5 ring-2 ring-blue-500/20'
									: 'hover:border-blue-500/30'}"
								onclick={() => (selectedPaymentMethod = method.id)}
							>
								<div class="flex w-full items-center gap-3">
									<div class="rounded-lg bg-blue-500/10 p-2">
										<method.icon
											class="h-6 w-6 {selectedPaymentMethod === method.id
												? 'text-blue-500'
												: 'text-muted-foreground'}"
										/>
									</div>
									<div class="flex-1 text-wrap">
										<Label for={method.id} class="cursor-pointer font-bold text-blue-500">
											{method.name}
										</Label>
										<p class="text-xs text-muted-foreground">{method.description}</p>
									</div>
								</div>
								<RadioGroupItem value={method.id} id={method.id} class="sr-only" />
							</Button>
						{/each}
					</RadioGroup>
				</div>
			</div>

			<DialogFooter class="gap-2 sm:gap-0">
				<Button variant="outline" onclick={onClose}>Cancel</Button>
				<Button
					onclick={processPayment}
					disabled={isProcessing}
					class="bg-blue-600 text-white transition-colors hover:bg-blue-700"
				>
					{#if isProcessing}
						<div
							class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
						></div>
						Processing...
					{:else}
						Confirm Payment
					{/if}
				</Button>
			</DialogFooter>
		{:else}
			<div class="flex flex-col items-center justify-center py-8 text-center">
				<div class="mb-6 flex h-16 w-16 items-center justify-center rounded-full bg-green-500/20">
					<Check class="h-8 w-8 text-green-500" />
				</div>
				<h3 class="mb-2 text-2xl font-bold text-green-500">Payment Successful!</h3>
				<p class="mb-8 text-muted-foreground">
					Payment of <span class="font-semibold text-white">{paidAmount.toFixed(2)}</span> has been processed
					successfully.
				</p>

				<div class="flex w-full gap-3">
					<Button
						variant="outline"
						class="flex-1 border-dashed hover:border-blue-500 hover:bg-blue-500/5"
						onclick={printReceipt}
					>
						<Printer class="mr-2 h-4 w-4" />
						Print Receipt
					</Button>
					<Button
						class="flex-1 bg-blue-600 text-white transition-colors hover:bg-blue-700"
						onclick={completeCheckout}
					>
						Complete
					</Button>
				</div>
			</div>
		{/if}
	</DialogContent>
</Dialog>
