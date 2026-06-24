// Mock product catalog for the Tables demo.

export interface DemoProduct {
	id: string;
	name: string;
	category: string;
	price: number;
	stock: number;
	status: 'in_stock' | 'low' | 'out';
}

export const demoProducts: DemoProduct[] = [
	{
		id: 'prd_01',
		name: 'Aurora Desk Lamp',
		category: 'Lighting',
		price: 79.0,
		stock: 8,
		status: 'low'
	},
	{
		id: 'prd_02',
		name: 'Nimbus Wireless Keyboard',
		category: 'Accessories',
		price: 129.0,
		stock: 142,
		status: 'in_stock'
	},
	{
		id: 'prd_03',
		name: 'Halo Noise-Cancelling Headphones',
		category: 'Audio',
		price: 249.0,
		stock: 0,
		status: 'out'
	},
	{
		id: 'prd_04',
		name: 'Drift Ergonomic Mouse',
		category: 'Accessories',
		price: 59.0,
		stock: 87,
		status: 'in_stock'
	},
	{
		id: 'prd_05',
		name: 'Lumen 27" 4K Monitor',
		category: 'Displays',
		price: 419.0,
		stock: 23,
		status: 'in_stock'
	},
	{
		id: 'prd_06',
		name: 'Pulse Mechanical Keypad',
		category: 'Accessories',
		price: 44.0,
		stock: 6,
		status: 'low'
	},
	{
		id: 'prd_07',
		name: 'Cascade USB-C Hub',
		category: 'Accessories',
		price: 39.0,
		stock: 210,
		status: 'in_stock'
	},
	{
		id: 'prd_08',
		name: 'Verge Standing Desk',
		category: 'Furniture',
		price: 549.0,
		stock: 0,
		status: 'out'
	},
	{
		id: 'prd_09',
		name: 'Atlas Laptop Stand',
		category: 'Furniture',
		price: 69.0,
		stock: 64,
		status: 'in_stock'
	},
	{
		id: 'prd_10',
		name: 'Echo Smart Speaker',
		category: 'Audio',
		price: 99.0,
		stock: 4,
		status: 'low'
	},
	{
		id: 'prd_11',
		name: 'Glint Webcam 1080p',
		category: 'Video',
		price: 89.0,
		stock: 51,
		status: 'in_stock'
	},
	{
		id: 'prd_12',
		name: 'Strata Cable Organizer',
		category: 'Accessories',
		price: 19.0,
		stock: 320,
		status: 'in_stock'
	},
	{
		id: 'prd_13',
		name: 'Orbit Wireless Charger',
		category: 'Power',
		price: 49.0,
		stock: 9,
		status: 'low'
	},
	{
		id: 'prd_14',
		name: 'Quartz Mechanical Keyboard',
		category: 'Accessories',
		price: 159.0,
		stock: 38,
		status: 'in_stock'
	},
	{
		id: 'prd_15',
		name: 'Vertex Gaming Chair',
		category: 'Furniture',
		price: 329.0,
		stock: 0,
		status: 'out'
	},
	{
		id: 'prd_16',
		name: 'Beam LED Light Bar',
		category: 'Lighting',
		price: 54.0,
		stock: 76,
		status: 'in_stock'
	},
	{
		id: 'prd_17',
		name: 'Tide Portable SSD 1TB',
		category: 'Storage',
		price: 119.0,
		stock: 5,
		status: 'low'
	},
	{
		id: 'prd_18',
		name: 'Crisp Bluetooth Earbuds',
		category: 'Audio',
		price: 89.0,
		stock: 198,
		status: 'in_stock'
	},
	{
		id: 'prd_19',
		name: 'Forge Laptop Sleeve',
		category: 'Accessories',
		price: 34.0,
		stock: 112,
		status: 'in_stock'
	},
	{
		id: 'prd_20',
		name: 'Solace White Noise Machine',
		category: 'Audio',
		price: 45.0,
		stock: 0,
		status: 'out'
	}
];
