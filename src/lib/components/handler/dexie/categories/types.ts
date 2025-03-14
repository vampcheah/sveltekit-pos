/**
 * @typedef {Object} Category
 * @property {number} [id] - 分类ID（自动生成）
 * @property {string} name - 分类名称
 * @property {string} description - 分类描述
 * @property {Date} createdAt - 创建时间
 * @property {Date} updatedAt - 更新时间
 */

/**
 * @type {Category}
 */

export interface Category {
	id?: number;
	code: string;
	name: string;
}

export const defaultCategory = {
	code: '',
	name: ''
};

export default defaultCategory;
