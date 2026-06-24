<!--
  Register page. Collects name/email/password/confirm with light client-side
  validation, then calls the mock `auth.register` and routes to the dashboard.
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { toast } from 'svelte-sonner';
	import Eye from '@lucide/svelte/icons/eye';
	import EyeOff from '@lucide/svelte/icons/eye-off';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Spinner } from '$lib/components/shared';
	import { auth } from '$lib/auth';
	import { emailSchema, fieldError } from '$lib/utils/validators';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirm = $state('');
	let showPassword = $state(false);
	let submitting = $state(false);

	let errors = $state<{ name?: string; email?: string; password?: string; confirm?: string }>({});
	let formError = $state('');

	function validate(): boolean {
		const next: typeof errors = {};
		if (!name.trim()) next.name = 'Name is required';

		const emailResult = emailSchema.safeParse(email);
		if (!emailResult.success) next.email = fieldError(emailResult.error, '');

		if (password.length < 8) next.password = 'Password must be at least 8 characters';
		if (confirm !== password) next.confirm = 'Passwords do not match';

		errors = next;
		return Object.keys(next).length === 0;
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		formError = '';
		if (!validate()) return;

		submitting = true;
		const result = await auth.register(name, email, password);
		submitting = false;

		if (result.ok) {
			toast.success('Account created. Welcome aboard!');
			goto(resolve('/dashboard'));
		} else {
			formError = result.error ?? 'Unable to create account. Please try again.';
			toast.error(formError);
		}
	}
</script>

<svelte:head>
	<title>Create account · Admin Starter</title>
</svelte:head>

<Card.Root class="shadow-sm">
	<Card.Header class="space-y-1 text-center">
		<Card.Title class="text-xl">Create your account</Card.Title>
		<Card.Description>Get started with the admin starter</Card.Description>
	</Card.Header>

	<Card.Content>
		<form class="space-y-4" onsubmit={handleSubmit} novalidate>
			<div class="space-y-2">
				<Label for="name">Full name</Label>
				<Input
					id="name"
					type="text"
					autocomplete="name"
					placeholder="Jane Doe"
					bind:value={name}
					aria-invalid={errors.name ? 'true' : undefined}
				/>
				{#if errors.name}
					<p class="text-xs text-red-500">{errors.name}</p>
				{/if}
			</div>

			<div class="space-y-2">
				<Label for="email">Email</Label>
				<Input
					id="email"
					type="email"
					autocomplete="email"
					placeholder="you@example.com"
					bind:value={email}
					aria-invalid={errors.email ? 'true' : undefined}
				/>
				{#if errors.email}
					<p class="text-xs text-red-500">{errors.email}</p>
				{/if}
			</div>

			<div class="space-y-2">
				<Label for="password">Password</Label>
				<div class="relative">
					<Input
						id="password"
						type={showPassword ? 'text' : 'password'}
						autocomplete="new-password"
						placeholder="At least 8 characters"
						class="pr-10"
						bind:value={password}
						aria-invalid={errors.password ? 'true' : undefined}
					/>
					<button
						type="button"
						onclick={() => (showPassword = !showPassword)}
						class="absolute inset-y-0 right-0 flex w-10 items-center justify-center text-muted-foreground transition-colors hover:text-foreground"
						aria-label={showPassword ? 'Hide password' : 'Show password'}
					>
						{#if showPassword}
							<EyeOff class="size-4" aria-hidden="true" />
						{:else}
							<Eye class="size-4" aria-hidden="true" />
						{/if}
					</button>
				</div>
				{#if errors.password}
					<p class="text-xs text-red-500">{errors.password}</p>
				{/if}
			</div>

			<div class="space-y-2">
				<Label for="confirm">Confirm password</Label>
				<Input
					id="confirm"
					type={showPassword ? 'text' : 'password'}
					autocomplete="new-password"
					placeholder="Re-enter your password"
					bind:value={confirm}
					aria-invalid={errors.confirm ? 'true' : undefined}
				/>
				{#if errors.confirm}
					<p class="text-xs text-red-500">{errors.confirm}</p>
				{/if}
			</div>

			{#if formError}
				<p class="rounded-md bg-destructive/10 px-3 py-2 text-xs text-red-500">{formError}</p>
			{/if}

			<Button type="submit" class="w-full" disabled={submitting}>
				{#if submitting}
					<Spinner class="size-4 text-primary-foreground" />
					Creating account…
				{:else}
					Create account
				{/if}
			</Button>
		</form>
	</Card.Content>

	<Card.Footer class="justify-center">
		<p class="text-sm text-muted-foreground">
			Already have an account?
			<a
				href={resolve('/login')}
				class="font-medium text-foreground underline-offset-4 hover:underline"
			>
				Sign in
			</a>
		</p>
	</Card.Footer>
</Card.Root>
