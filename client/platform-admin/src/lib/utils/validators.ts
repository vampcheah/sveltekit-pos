// Reusable zod schemas and a helper for surfacing per-field error messages.
// Shared by auth and user forms across the app.

import { z } from 'zod';

/** A trimmed, well-formed email address. */
export const emailSchema: z.ZodString = z
	.string()
	.trim()
	.min(1, 'Email is required')
	.email('Enter a valid email address');

/** A password of at least 8 characters. */
export const passwordSchema: z.ZodString = z
	.string()
	.min(8, 'Password must be at least 8 characters');

/** Credentials accepted by the login form. 后端用用户名登录（字段名沿用 email 以免改页面绑定）。 */
export const loginSchema = z.object({
	email: z.string().trim().min(1, '请输入用户名'),
	password: z.string().min(1, '请输入密码')
});

/** Shape of a user record edited via the users form. */
export const userSchema = z.object({
	name: z.string().trim().min(1, 'Name is required'),
	email: emailSchema,
	role: z.enum(['admin', 'editor', 'viewer'])
});

export type LoginInput = z.infer<typeof loginSchema>;
export type UserInput = z.infer<typeof userSchema>;

/**
 * Look up the first validation message for a given field `path` (dot-notation
 * for nested fields) from a `ZodError`. Returns `undefined` when there is no
 * error or no matching issue.
 */
export function fieldError(err: z.ZodError | null, path: string): string | undefined {
	if (!err) return undefined;
	const issue = err.issues.find((i) => i.path.join('.') === path);
	return issue?.message;
}
