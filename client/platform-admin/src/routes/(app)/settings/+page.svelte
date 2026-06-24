<!--
  General settings. An organization form (name, support email, timezone, UI
  language) plus a read-only account section sourced from the mock auth user.
  All mutations operate on local $state — there is no backend.
-->
<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { toast } from 'svelte-sonner';
	import { auth } from '$lib/auth';
	import { logoutDialog } from '$lib/shell';
	import { setLocale, LOCALES, i18n, type Locale } from '$lib/i18n';
	import { initials } from '$lib/utils/formatters';
	import Save from '@lucide/svelte/icons/save';

	// A small, static list of common timezones for the mock Select.
	const timezones = [
		{ value: 'UTC', label: 'UTC' },
		{ value: 'America/New_York', label: 'New York (UTC-05:00)' },
		{ value: 'America/Los_Angeles', label: 'Los Angeles (UTC-08:00)' },
		{ value: 'Europe/London', label: 'London (UTC+00:00)' },
		{ value: 'Europe/Berlin', label: 'Berlin (UTC+01:00)' },
		{ value: 'Asia/Shanghai', label: 'Shanghai (UTC+08:00)' },
		{ value: 'Asia/Tokyo', label: 'Tokyo (UTC+09:00)' }
	];

	// Local form state — seeded with sensible mock defaults.
	let orgName = $state('Acme Inc.');
	let supportEmail = $state('support@acme.example');
	let timezone = $state('UTC');

	// Language is bound to the live i18n locale and applied immediately on change.
	let language = $state<Locale>(i18n.locale);

	const timezoneLabel = $derived(
		timezones.find((tz) => tz.value === timezone)?.label ?? 'Select timezone'
	);
	const languageLabel = $derived(
		LOCALES.find((l) => l.value === language)?.label ?? 'Select language'
	);

	const user = $derived(auth.user);

	function onLanguageChange(value: string): void {
		language = value as Locale;
		setLocale(language);
	}

	function saveGeneral(event: SubmitEvent): void {
		event.preventDefault();
		// No backend — just confirm the (local) change.
		toast.success('Settings saved', {
			description: 'Your general settings have been updated.'
		});
	}
</script>

<svelte:head>
	<title>General Settings · Admin Starter</title>
</svelte:head>

<form onsubmit={saveGeneral} class="space-y-6">
	<Card.Root>
		<Card.Header>
			<Card.Title>Organization</Card.Title>
			<Card.Description>
				Details about your organization shown across the workspace.
			</Card.Description>
		</Card.Header>
		<Card.Content class="space-y-5">
			<div class="grid gap-2">
				<Label for="org-name">Organization name</Label>
				<Input id="org-name" bind:value={orgName} placeholder="Acme Inc." />
			</div>

			<div class="grid gap-2">
				<Label for="support-email">Support email</Label>
				<Input
					id="support-email"
					type="email"
					bind:value={supportEmail}
					placeholder="support@example.com"
				/>
				<p class="text-muted-foreground text-xs">
					Replies to customer notifications are sent to this address.
				</p>
			</div>

			<div class="grid gap-5 sm:grid-cols-2">
				<div class="grid gap-2">
					<Label for="timezone">Timezone</Label>
					<Select.Root type="single" bind:value={timezone}>
						<Select.Trigger id="timezone" class="w-full">
							{timezoneLabel}
						</Select.Trigger>
						<Select.Content>
							{#each timezones as tz (tz.value)}
								<Select.Item value={tz.value} label={tz.label}>
									{tz.label}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				<div class="grid gap-2">
					<Label for="language">Language</Label>
					<Select.Root type="single" value={language} onValueChange={onLanguageChange}>
						<Select.Trigger id="language" class="w-full">
							{languageLabel}
						</Select.Trigger>
						<Select.Content>
							{#each LOCALES as locale (locale.value)}
								<Select.Item value={locale.value} label={locale.label}>
									{locale.label}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					<p class="text-muted-foreground text-xs">
						Changes apply immediately across the interface.
					</p>
				</div>
			</div>
		</Card.Content>
		<Card.Footer class="justify-end border-t">
			<Button type="submit">
				<Save class="size-4" />
				Save changes
			</Button>
		</Card.Footer>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Account</Card.Title>
			<Card.Description>The account currently signed in.</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div class="flex items-center gap-4">
					<Avatar.Root class="size-12">
						{#if user?.avatarUrl}
							<Avatar.Image src={user.avatarUrl} alt={user.name} />
						{/if}
						<Avatar.Fallback>
							{user ? initials(user.name) : '?'}
						</Avatar.Fallback>
					</Avatar.Root>
					<div class="min-w-0">
						<div class="flex items-center gap-2">
							<p class="truncate font-medium">{user?.name ?? 'Guest'}</p>
							{#if user}
								<Badge variant="secondary" class="capitalize">{user.role}</Badge>
							{/if}
						</div>
						<p class="text-muted-foreground truncate text-sm">
							{user?.email ?? 'Not signed in'}
						</p>
					</div>
				</div>
				<Button
					type="button"
					variant="outline"
					class="text-destructive hover:text-destructive"
					onclick={() => (logoutDialog.open = true)}
				>
					Sign out
				</Button>
			</div>
			<Separator class="my-4" />
			<p class="text-muted-foreground text-xs">
				This is a demo account. Profile editing is available on the Profile page.
			</p>
		</Card.Content>
	</Card.Root>
</form>
