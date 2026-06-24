import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		// SPA：登录后纯客户端管理台，无需 SSR；产物为静态文件，由 Caddy 托管。
		adapter: adapter({ fallback: 'index.html' })
	}
};

export default config;
