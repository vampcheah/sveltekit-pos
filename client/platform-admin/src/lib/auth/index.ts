// Public barrel for the auth module.

export { auth } from './auth.svelte';
export type { User } from './types';
// 接入 pos-server 的真实后端 provider。
export { authProvider, apiAuthProvider, type AuthProvider, type AuthResult } from './provider';
