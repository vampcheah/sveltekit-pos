// Mock dashboard metrics. Pages attach Lucide icons themselves — data stays
// icon-free so it can live in a plain `.ts` file.

export const stats: {
	title: string;
	value: string;
	change: number;
	trend: 'up' | 'down' | 'neutral';
}[] = [
	{ title: 'Total Revenue', value: '$48,920', change: 12.5, trend: 'up' },
	{ title: 'Active Users', value: '8,432', change: 4.2, trend: 'up' },
	{ title: 'New Orders', value: '1,290', change: -2.1, trend: 'down' },
	{ title: 'Conversion Rate', value: '3.84%', change: 0, trend: 'neutral' }
];

export const revenueSeries: { date: string; revenue: number; orders: number }[] = [
	{ date: '2024-07', revenue: 32400, orders: 820 },
	{ date: '2024-08', revenue: 35100, orders: 905 },
	{ date: '2024-09', revenue: 33980, orders: 870 },
	{ date: '2024-10', revenue: 38600, orders: 980 },
	{ date: '2024-11', revenue: 41250, orders: 1040 },
	{ date: '2024-12', revenue: 47800, orders: 1230 },
	{ date: '2025-01', revenue: 39900, orders: 990 },
	{ date: '2025-02', revenue: 42600, orders: 1075 },
	{ date: '2025-03', revenue: 45300, orders: 1160 },
	{ date: '2025-04', revenue: 44120, orders: 1110 },
	{ date: '2025-05', revenue: 47650, orders: 1245 },
	{ date: '2025-06', revenue: 48920, orders: 1290 }
];

export const recentActivity: {
	id: string;
	user: string;
	action: string;
	target: string;
	time: string;
}[] = [
	{
		id: 'act_1',
		user: 'Olivia Martin',
		action: 'created',
		target: 'Invoice #INV-2043',
		time: '2 minutes ago'
	},
	{
		id: 'act_2',
		user: 'Liam Johnson',
		action: 'updated',
		target: 'Product “Aurora Lamp”',
		time: '18 minutes ago'
	},
	{
		id: 'act_3',
		user: 'Sophia Miller',
		action: 'invited',
		target: 'emma.williams@example.com',
		time: '1 hour ago'
	},
	{
		id: 'act_4',
		user: 'Noah Brown',
		action: 'archived',
		target: 'Order #ORD-1187',
		time: '2 hours ago'
	},
	{
		id: 'act_5',
		user: 'Charlotte Anderson',
		action: 'commented on',
		target: 'Ticket #SUP-552',
		time: '4 hours ago'
	},
	{
		id: 'act_6',
		user: 'Daniel Martinez',
		action: 'exported',
		target: 'Q2 Revenue Report',
		time: 'Yesterday'
	},
	{
		id: 'act_7',
		user: 'Abigail Lewis',
		action: 'deleted',
		target: 'Draft “Spring Campaign”',
		time: 'Yesterday'
	},
	{
		id: 'act_8',
		user: 'Sebastian Lee',
		action: 'changed role of',
		target: 'Mia Moore',
		time: '2 days ago'
	}
];

export const trafficByChannel: { channel: string; visitors: number }[] = [
	{ channel: 'Organic Search', visitors: 18420 },
	{ channel: 'Direct', visitors: 11240 },
	{ channel: 'Social', visitors: 7650 },
	{ channel: 'Referral', visitors: 4310 },
	{ channel: 'Email', visitors: 2980 }
];
