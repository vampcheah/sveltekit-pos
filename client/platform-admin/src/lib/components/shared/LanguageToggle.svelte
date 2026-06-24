<!--
  LanguageToggle — dropdown that lists the available locales (from i18n) and
  switches the active one via `setLocale`. A check marks the current locale.
-->
<script lang="ts">
	import Languages from '@lucide/svelte/icons/languages';
	import Check from '@lucide/svelte/icons/check';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { LOCALES, setLocale, i18n, type Locale } from '$lib/i18n';

	function select(value: Locale) {
		setLocale(value);
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button variant="ghost" size="icon" aria-label="Change language" {...props}>
				<Languages class="size-4" aria-hidden="true" />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="min-w-40">
		{#each LOCALES as locale (locale.value)}
			<DropdownMenu.Item onSelect={() => select(locale.value)} class="justify-between">
				<span>{locale.label}</span>
				{#if i18n.locale === locale.value}
					<Check class="size-4" aria-hidden="true" />
				{/if}
			</DropdownMenu.Item>
		{/each}
	</DropdownMenu.Content>
</DropdownMenu.Root>
