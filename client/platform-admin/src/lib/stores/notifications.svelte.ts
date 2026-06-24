// In-memory notifications store (mock). Seeded with a few demo items so the
// header bell has something to show. Runes singleton — import `notifications`.

export interface AppNotification {
	id: string;
	title: string;
	body: string;
	time: string;
	read: boolean;
	type: 'info' | 'success' | 'warning';
}

const seed: AppNotification[] = [
	{
		id: 'n1',
		title: 'New user registered',
		body: 'Mia Chen just created an account and is awaiting a role assignment.',
		time: '2 minutes ago',
		read: false,
		type: 'info'
	},
	{
		id: 'n2',
		title: 'Payment received',
		body: 'Invoice #INV-2043 was paid successfully.',
		time: '1 hour ago',
		read: false,
		type: 'success'
	},
	{
		id: 'n3',
		title: 'Low stock warning',
		body: 'Aurora Desk Lamp has dropped below the reorder threshold.',
		time: '3 hours ago',
		read: false,
		type: 'warning'
	},
	{
		id: 'n4',
		title: 'Weekly report ready',
		body: 'Your analytics summary for last week is available to review.',
		time: 'Yesterday',
		read: true,
		type: 'info'
	}
];

class NotificationsStore {
	#items = $state<AppNotification[]>(seed);

	get items(): AppNotification[] {
		return this.#items;
	}

	get unread(): number {
		return this.#items.reduce((count, item) => count + (item.read ? 0 : 1), 0);
	}

	markRead(id: string): void {
		this.#items = this.#items.map((item) => (item.id === id ? { ...item, read: true } : item));
	}

	markAllRead(): void {
		this.#items = this.#items.map((item) => (item.read ? item : { ...item, read: true }));
	}

	remove(id: string): void {
		this.#items = this.#items.filter((item) => item.id !== id);
	}
}

export const notifications = new NotificationsStore();
