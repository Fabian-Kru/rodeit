import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';
import solid from '@astrojs/solid-js';
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import autoprefixer from 'autoprefixer';
import { defineConfig } from 'astro/config';
import node from '@astrojs/node';

const DIRNAME = dirname(fileURLToPath(import.meta.url));

// https://astro.build/config
export default defineConfig({
	output: 'server',
	integrations: [solid()],
	vite: {
		css: {
			postcss: {
				plugins: [autoprefixer()],
			},
		},
		plugins: [
			vanillaExtractPlugin({
				esbuildOptions: {
					loader: {
						'.svg': 'dataurl',
					},
				},
			}),
		],
		resolve: {
			alias: [
				{
					find: /^src/,
					replacement: resolve(DIRNAME, 'src'),
				},
			],
		},
	},
	adapter: node({
		mode: 'standalone',
	}),
});
