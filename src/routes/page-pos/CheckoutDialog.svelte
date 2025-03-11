<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { RadioGroup, RadioGroupItem } from '$lib/components/ui/radio-group';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';
	import { CreditCard, Banknote, Printer, Check } from 'lucide-svelte';
	import { cartStore } from './CartStore.svelte';
	import * as m from '$lib/paraglide/messages.js';
	import { numberWithCurrency } from '$lib/tools/numbering';
	// States
	let isOpen = $state(false);
	let selectedPaymentMethod = $state('cash');
	let isProcessing = $state(false);
	let isCompleted = $state(false);
	let paidAmount = $state(0);
	let enableCheckout = $derived(cartStore.cart.length > 0);

	// Payment methods with enhanced metadata
	const resetDialog = () => {
		isOpen = false;
		isProcessing = false;
		isCompleted = false;
		toast.dismiss();
	};

	const paymentMethods = [
		{
			id: 'cash',
			name: m.pos_payment_cash(),
			icon: Banknote,
			description: m.pos_payment_cash_description()
		},
		{
			id: 'credit_card',
			name: m.pos_payment_credit_card(),
			icon: CreditCard,
			description: m.pos_payment_credit_card_description()
		},
		{
			id: 'bsc_usdt',
			name: m.pos_payment_bsc_usdt(),
			icon: Banknote,
			description: m.pos_payment_bsc_usdt_description()
		}
	];

	const processPayment = async () => {
		isProcessing = true;
		paidAmount = cartStore.total;

		// TODO: Add payment processing logic

		cartStore.checkout();
		isProcessing = false;
		isCompleted = true;
	};

	const printReceipt = () => {
		toast.success(m.pos_print_receipt());
	};

	const paymentComplete = () => {
		resetDialog();
	};
</script>

<Dialog.Root bind:open={isOpen}>
	<Button
		class="flex w-full items-center justify-center rounded-lg bg-blue-700 py-2 text-white hover:bg-blue-800 {cartStore
			.cart.length === 0
			? 'opacity-50'
			: ''}"
		disabled={!enableCheckout}
		onclick={() => (isOpen = true)}
	>
		<CreditCard class="mr-2 h-5 w-5" />
		{m.pos_button_checkout()}
	</Button>
	<Dialog.Content class="max-w-sm rounded-lg sm:max-w-xl">
		<Dialog.Header>
			<Dialog.Title class="text-2xl font-bold">{m.pos_complete_purchase()}</Dialog.Title>
		</Dialog.Header>

		{#if !isCompleted}
			<div class="py-6">
				<div class="mb-8 rounded-lg bg-blue-50/10 p-4">
					<div class="mb-1 text-sm text-muted-foreground">{m.pos_total_amount()}</div>
					<div class="text-3xl font-bold text-green-500">{cartStore.total.toFixed(2)}</div>
				</div>

				<div class="space-y-4">
					<h3 class="text-lg font-medium">{m.pos_select_payment_method()}</h3>

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

			<Dialog.Footer class="gap-2 sm:gap-0">
				<Button variant="outline" onclick={() => (isOpen = false)}>{m.pos_button_cancel()}</Button>
				<Button
					onclick={processPayment}
					disabled={isProcessing}
					class="bg-blue-600 text-white transition-colors hover:bg-blue-700"
				>
					{#if isProcessing}
						<div
							class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
						></div>
						{m.pos_button_processing()}
					{:else}
						{m.pos_button_confirm_payment()}
					{/if}
				</Button>
			</Dialog.Footer>
		{/if}

		{#if isCompleted}
			<div class="flex flex-col items-center justify-center py-8 text-center">
				<div class="mb-6 flex h-16 w-16 items-center justify-center rounded-full bg-green-500/20">
					<Check class="h-8 w-8 text-green-500" />
				</div>
				<h3 class="mb-2 text-2xl font-bold text-green-500">{m.pos_payment_successful()}</h3>
				<p class="mb-8 text-muted-foreground">
					{@html m.pos_payment_successful_description({ amount: numberWithCurrency(paidAmount) })}
				</p>

				<div class="flex w-full gap-3">
					<Button
						variant="outline"
						class="flex-1 border-dashed hover:border-blue-500 hover:bg-blue-500/5"
						onclick={printReceipt}
					>
						<Printer class="mr-2 h-4 w-4" />
						{m.pos_button_print_receipt()}
					</Button>
					<Button
						class="flex-1 bg-blue-600 text-white transition-colors hover:bg-blue-700"
						onclick={paymentComplete}
					>
						{m.pos_button_complete_purchase()}
					</Button>
				</div>
			</div>
		{/if}
	</Dialog.Content>
</Dialog.Root>
