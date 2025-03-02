/**
 * @typedef {Object} Product
 * @property {number} [id] - 产品ID（自动生成）
 * @property {string} name - 产品名称
 * @property {number|null} categoryId - 分类ID
 * @property {number} price - 销售价格
 * @property {number} cost - 成本价格
 * @property {string} barcode - 条形码
 * @property {string} sku - 库存单位
 * @property {string} description - 产品描述
 * @property {string} imageUrl - 产品图片URL
 * @property {number} stock - 库存数量
 * @property {Date} createdAt - 创建时间
 * @property {Date} updatedAt - 更新时间
 */

/**
 * @type {Product}
 */

export interface Product {
	id?: number;
	name: string;
	categoryId: number;
	price: number;
	cost: number;
	barcode: string;
	sku: string;
	description: string;
	imageUrl: string;
	stock: number;
	createdAt: Date;
	updatedAt: Date;
}

export const defaultProduct = {
	name: '',
	categoryId: null,
	price: 0,
	cost: 0,
	barcode: '',
	sku: '',
	description: '',
	imageUrl: '',
	stock: 0,
	createdAt: new Date(),
	updatedAt: new Date()
};

export default defaultProduct;
