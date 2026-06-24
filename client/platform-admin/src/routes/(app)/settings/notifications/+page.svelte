<!--
  Notification settings. Preference toggles grouped into Email / Push / SMS
  cards. Everything lives in local $state (no backend) and Save shows a toast.
-->
<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { Separator } from '$lib/components/ui/separator';
	import { toast } from 'svelte-sonner';
	import type { Component } from 'svelte';
	import Mail from '@lucide/svelte/icons/mail';
	import Smartphone from '@lucide/svelte/icons/smartphone';
	import MessageSquare from '@lucide/svelte/icons/message-square';
	import Save from '@lucide/svelte/icons/save';

	interface Preference {
		id: string;
		label: string;
		description: string;
		enabled: boolean;
	}

	interface PreferenceGroup {
		id: string;
		title: string;
		description: string;
		icon: Component;
		preferences: Preference[];
	}

	// Grouped notification preferences with mock defaults.
	let groups = $state<PreferenceGroup[]>([
		{
			id: 'email',
			title: 'Email',
			description: 'Updates delivered to your inbox.',
			icon: Mail,
			preferences: [
				{
					id: 'email-product',
					label: 'Product updates',
					description: 'News about features and improvements.',
					enabled: true
				},
				{
					id: 'email-security',
					label: 'Security alerts',
					description: 'Sign-in attempts and password changes.',
					enabled: true
				},
				{
					id: 'email-marketing',
					label: 'Marketing',
					description: 'Tips, offers, and announcements.',
					enabled: false
				}
			]
		},
		{
			id: 'push',
			title: 'Push',
			description: 'Real-time alerts on your devices.',
			icon: Smartphone,
			preferences: [
				{
					id: 'push-activity',
					label: 'Account activity',
					description: 'Mentions, comments, and assignments.',
					enabled: true
				},
				{
					id: 'push-reminders',
					label: 'Reminders',
					description: 'Scheduled tasks and follow-ups.',
					enabled: false
				}
			]
		},
		{
			id: 'sms',
			title: 'SMS',
			description: 'Critical messages sent by text.',
			icon: MessageSquare,
			preferences: [
				{
					id: 'sms-security',
					label: 'Security codes',
					description: 'One-time codes for verification.',
					enabled: true
				},
				{
					id: 'sms-alerts',
					label: 'Urgent alerts',
					description: 'High-priority account warnings.',
					enabled: false
				}
			]
		}
	]);

	function setPreference(groupId: string, prefId: string, value: boolean): void {
		const group = groups.find((g) => g.id === groupId);
		const pref = group?.preferences.find((p) => p.id === prefId);
		if (pref) pref.enabled = value;
	}

	function savePreferences(): void {
		toast.success('Preferences saved', {
			description: 'Your notification preferences have been updated.'
		});
	}
</script>

<svelte:head>
	<title>Notifications · Admin Starter</title>
</svelte:head>

{#each groups as group (group.id)}
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-3">
				<div
					class="bg-muted text-muted-foreground flex size-9 shrink-0 items-center justify-center rounded-md"
				>
					<group.icon class="size-4" />
				</div>
				<div>
					<Card.Title>{group.title}</Card.Title>
					<Card.Description>{group.description}</Card.Description>
				</div>
			</div>
		</Card.Header>
		<Card.Content class="space-y-1">
			{#each group.preferences as pref, i (pref.id)}
				{#if i > 0}
					<Separator class="my-1" />
				{/if}
				<div class="flex items-center justify-between gap-4 py-2">
					<div class="min-w-0 space-y-0.5">
						<p class="text-sm font-medium">{pref.label}</p>
						<p class="text-muted-foreground text-sm">{pref.description}</p>
					</div>
					<Switch
						checked={pref.enabled}
						onCheckedChange={(value) => setPreference(group.id, pref.id, value)}
					/>
				</div>
			{/each}
		</Card.Content>
	</Card.Root>
{/each}

<div class="flex justify-end">
	<Button onclick={savePreferences}>
		<Save class="size-4" />
		Save preferences
	</Button>
</div>
