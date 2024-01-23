import { writeFile } from 'fs/promises';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';
import { rollup } from 'rollup';
import { defineConfig, Plugin, ResolvedConfig } from 'vite';

import vue from '@vitejs/plugin-vue';
import yaml from '@rollup/plugin-yaml';
import json from '@rollup/plugin-json';

import { merge_specs } from './util/openapi.ts';
import { build } from 'vite';

export default defineConfig({
	build: { target: 'esnext' },
	plugins: [vue(), yaml(), foo()],
});

const DIRNAME = dirname(fileURLToPath(import.meta.url));

function foo(): Plugin {
	let config: ResolvedConfig;

	return {
		name: 'foo',
		configResolved(_config) {
			config = _config
		},
		async writeBundle() {
			const output = await build({
				plugins: [yaml()],
				configFile: false,
				build: {
					target: 'esnext',
					write: false,
					lib: {
						entry: resolve(DIRNAME, 'config.ts'),
						fileName: 'config',
						formats: ['es'],
					},
				},
			});
			const code = output[0].output[0].code;
			const mod = await import(`data:text/javascript;charset=utf-8,${encodeURIComponent(code)}`);
			const spec = merge_specs(mod.specs);
			writeFile(resolve(config.root, config.build.outDir, 'openapi.json'), JSON.stringify(spec, null, 2));
		}
	}
}
