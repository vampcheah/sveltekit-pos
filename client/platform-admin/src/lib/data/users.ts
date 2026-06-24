// Mock user directory for the Users table demo. No backend involved.

export interface DemoUser {
	id: string;
	name: string;
	email: string;
	role: 'admin' | 'editor' | 'viewer';
	status: 'active' | 'invited' | 'suspended';
	createdAt: string;
	avatarUrl?: string;
}

export const demoUsers: DemoUser[] = [
	{
		id: 'usr_01',
		name: 'Olivia Martin',
		email: 'olivia.martin@example.com',
		role: 'admin',
		status: 'active',
		createdAt: '2024-01-12'
	},
	{
		id: 'usr_02',
		name: 'Liam Johnson',
		email: 'liam.johnson@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-01-28'
	},
	{
		id: 'usr_03',
		name: 'Emma Williams',
		email: 'emma.williams@example.com',
		role: 'viewer',
		status: 'invited',
		createdAt: '2024-02-03'
	},
	{
		id: 'usr_04',
		name: 'Noah Brown',
		email: 'noah.brown@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-02-15'
	},
	{
		id: 'usr_05',
		name: 'Ava Jones',
		email: 'ava.jones@example.com',
		role: 'viewer',
		status: 'suspended',
		createdAt: '2024-02-21'
	},
	{
		id: 'usr_06',
		name: 'William Garcia',
		email: 'william.garcia@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-03-04'
	},
	{
		id: 'usr_07',
		name: 'Sophia Miller',
		email: 'sophia.miller@example.com',
		role: 'admin',
		status: 'active',
		createdAt: '2024-03-11'
	},
	{
		id: 'usr_08',
		name: 'James Davis',
		email: 'james.davis@example.com',
		role: 'viewer',
		status: 'invited',
		createdAt: '2024-03-19'
	},
	{
		id: 'usr_09',
		name: 'Isabella Rodriguez',
		email: 'isabella.rodriguez@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-03-27'
	},
	{
		id: 'usr_10',
		name: 'Benjamin Wilson',
		email: 'benjamin.wilson@example.com',
		role: 'viewer',
		status: 'active',
		createdAt: '2024-04-02'
	},
	{
		id: 'usr_11',
		name: 'Mia Moore',
		email: 'mia.moore@example.com',
		role: 'editor',
		status: 'suspended',
		createdAt: '2024-04-09'
	},
	{
		id: 'usr_12',
		name: 'Lucas Taylor',
		email: 'lucas.taylor@example.com',
		role: 'viewer',
		status: 'active',
		createdAt: '2024-04-18'
	},
	{
		id: 'usr_13',
		name: 'Charlotte Anderson',
		email: 'charlotte.anderson@example.com',
		role: 'admin',
		status: 'active',
		createdAt: '2024-04-25'
	},
	{
		id: 'usr_14',
		name: 'Henry Thomas',
		email: 'henry.thomas@example.com',
		role: 'editor',
		status: 'invited',
		createdAt: '2024-05-01'
	},
	{
		id: 'usr_15',
		name: 'Amelia Jackson',
		email: 'amelia.jackson@example.com',
		role: 'viewer',
		status: 'active',
		createdAt: '2024-05-08'
	},
	{
		id: 'usr_16',
		name: 'Alexander White',
		email: 'alexander.white@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-05-16'
	},
	{
		id: 'usr_17',
		name: 'Harper Harris',
		email: 'harper.harris@example.com',
		role: 'viewer',
		status: 'suspended',
		createdAt: '2024-05-23'
	},
	{
		id: 'usr_18',
		name: 'Daniel Martinez',
		email: 'daniel.martinez@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-06-01'
	},
	{
		id: 'usr_19',
		name: 'Evelyn Thompson',
		email: 'evelyn.thompson@example.com',
		role: 'viewer',
		status: 'invited',
		createdAt: '2024-06-09'
	},
	{
		id: 'usr_20',
		name: 'Matthew Garcia',
		email: 'matthew.garcia@example.com',
		role: 'editor',
		status: 'active',
		createdAt: '2024-06-17'
	},
	{
		id: 'usr_21',
		name: 'Abigail Lewis',
		email: 'abigail.lewis@example.com',
		role: 'admin',
		status: 'active',
		createdAt: '2024-06-24'
	},
	{
		id: 'usr_22',
		name: 'Sebastian Lee',
		email: 'sebastian.lee@example.com',
		role: 'viewer',
		status: 'active',
		createdAt: '2024-07-02'
	},
	{
		id: 'usr_23',
		name: 'Elizabeth Walker',
		email: 'elizabeth.walker@example.com',
		role: 'editor',
		status: 'suspended',
		createdAt: '2024-07-10'
	},
	{
		id: 'usr_24',
		name: 'Jack Hall',
		email: 'jack.hall@example.com',
		role: 'viewer',
		status: 'active',
		createdAt: '2024-07-18'
	}
];
