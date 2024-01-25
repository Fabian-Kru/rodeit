import { resolve } from 'path';
import { fileURLToPath } from 'url';
import { defineConfig } from "@solidjs/start/config";
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import autoprefixer from 'autoprefixer';

const DIRNAME = resolve(fileURLToPath(import.meta.url));

export default defineConfig({
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
				find: '/^src/',
				replacement: resolve(DIRNAME, 'src'),
			},
		],
	},
});
