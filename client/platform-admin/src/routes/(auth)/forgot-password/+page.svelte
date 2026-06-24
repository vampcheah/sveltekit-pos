<!--
  Forgot-password page. Accepts an email and shows a neutral success state that
  never reveals whether the account exists. Purely mock — no email is sent.
-->
<script lang="ts">
	import { resolve } from '$app/paths';
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import MailCheck from '@lucide/svelte/icons/mail-check';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Spinner } from '$lib/components/shared';
	import { emailSchema, fieldError } from '$lib/utils/validators';

	let email = $state('');
	let submitting = $state(false);
	let submitted = $state(false);
	let error = $state('');

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';

		const result = emailSchema.safeParse(email);
		if (!result.success) {
			error = fieldError(result.error, '') ?? 'Enter a valid email address';
			return;
		}

		submitting = true;
		// Simulate a request round-trip for a realistic mock experience.
		await new Promise((resolve) => setTimeout(resolve, 600));
		submitting = false;
		submitted = true;
	}
</script>

<svelte:head>
	<title>Reset password · Admin Starter</title>
</svelte:head>

<Card.Root class="shadow-sm">
	{#if submitted}
		<Card.Header class="items-center space-y-3 text-center">
			<div
				class="flex size-12 items-center justify-center rounded-full bg-emerald-500/10 text-emerald-500"
			>
				<MailCheck class="size-6" aria-hidden="true" />
			</div>
			<Card.Title class="text-xl">Check your inbox</Card.Title>
			<Card.Description>
				If an account exists for <span class="font-medium text-foreground">{email}</span>, we sent a
				reset link.
			</Card.Description>
		</Card.Header>
		<Card.Footer class="justify-center">
			<Button href="/login" variant="outline" class="w-full">
				<ArrowLeft class="size-4" aria-hidden="true" />
				Back to sign in
			</Button>
		</Card.Footer>
	{:else}
		<Card.Header class="space-y-1 text-center">
			<Card.Title class="text-xl">Forgot your password?</Card.Title>
			<Card.Description>Enter your email and we'll send you a link to reset it.</Card.Description>
		</Card.Header>

		<Card.Content>
			<form class="space-y-4" onsubmit={handleSubmit} novalidate>
				<div class="space-y-2">
					<Label for="email">Email</Label>
					<Input
						id="email"
						type="email"
						autocomplete="email"
						placeholder="you@example.com"
						bind:value={email}
						aria-invalid={error ? 'true' : undefined}
					/>
					{#if error}
						<p class="text-xs text-red-500">{error}</p>
					{/if}
				</div>

				<Button type="submit" class="w-full" disabled={submitting}>
					{#if submitting}
						<Spinner class="size-4 text-primary-foreground" />
						Sending…
					{:else}
						Send reset link
					{/if}
				</Button>
			</form>
		</Card.Content>

		<Card.Footer class="justify-center">
			<a
				href={resolve('/login')}
				class="inline-flex items-center gap-1 text-sm font-medium text-muted-foreground underline-offset-4 hover:text-foreground hover:underline"
			>
				<ArrowLeft class="size-4" aria-hidden="true" />
				Back to sign in
			</a>
		</Card.Footer>
	{/if}
</Card.Root>
