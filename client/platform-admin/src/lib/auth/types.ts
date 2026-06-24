// Auth domain types for the mock authentication layer.

export interface User {
	id: string;
	name: string;
	email: string;
	role: 'admin' | 'editor' | 'viewer';
	avatarUrl?: string;
	/** 后端授予的权限码，用于侧栏/操作的按权限展示。 */
	permissions?: string[];
}
