import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import { compression } from 'vite-plugin-compression2';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit(),
		devtoolsJson(),
		wasm(),
		topLevelAwait(),
		compression({
			algorithms: ['gzip', 'brotli'],
			include: /\.(js|mjs|json|css|html|wasm)$/,
			threshold: 1024 // Only compress files larger than 1KB
		})
	]
});
