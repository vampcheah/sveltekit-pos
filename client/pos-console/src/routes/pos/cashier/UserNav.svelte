<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { User, Moon, Sun } from 'lucide-svelte';
	import { cn } from '$lib/utils.js';
	import { toggleMode } from 'mode-watcher';
	import { goto } from '$app/navigation';

	const logout = () => {
		goto('/');
	};
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class={cn(
			buttonVariants({
				variant: 'ghost',
				class:
					'relative size-10 items-center justify-center rounded-lg ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2'
			})
		)}
	>
		<Avatar.Root
			class="border-grey-500 flex size-10 items-center justify-center rounded-lg border p-2"
		>
			<User class="h-5 w-5" />
		</Avatar.Root>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content class="w-56" align="end">
		<DropdownMenu.Group>
			<DropdownMenu.GroupHeading class="font-normal">
				<div class="flex flex-col space-y-1">
					<p class="text-sm font-medium leading-none">svelte-pos</p>
					<p class="text-xs leading-none text-muted-foreground">m-cashier@example.com</p>
				</div>
			</DropdownMenu.GroupHeading>
			<DropdownMenu.Separator />
			<DropdownMenu.Group>
				<DropdownMenu.Item class="flex cursor-pointer items-center" onclick={toggleMode}>
					<div class="hidden items-center justify-start gap-2 dark:flex">
						<Sun class="h-[1.2rem] w-[1.2rem] rotate-0 transition-all dark:-rotate-90" /> Light Mode
					</div>
					<div class="flex items-center justify-start gap-2 dark:hidden">
						<Moon class="h-[1.2rem] w-[1.2rem] rotate-90 transition-all dark:rotate-0" /> Dark Mode
					</div>
					<span class="sr-only">Toggle theme</span>
				</DropdownMenu.Item>
			</DropdownMenu.Group>
			<DropdownMenu.Separator />
			<DropdownMenu.Item class="flex cursor-pointer items-center" onclick={logout}
				>Log out</DropdownMenu.Item
			>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
