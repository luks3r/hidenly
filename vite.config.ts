import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import svg from '@poppanator/sveltekit-svg';

/** @type {import('vite').UserConfig} */
export default defineConfig({
	plugins: [
		sveltekit(),
		svg({
			includePaths: ['./src/lib/images'],
			svgoOptions: {
				multipass: true,
				plugins: [
					{
						name: 'preset-default',
						params: {
							overrides: {
                                removeViewBox: false,
                            },
						}
					},
					{
						name: 'removeAttrs',
						params: {
							attrs: '(stroke|fill)'
						}
					}
				]
			}
		}),
	]
});
