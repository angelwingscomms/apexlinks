import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const config = {
	preprocess: vitePreprocess(),
	kit: { 
		adapter: adapter(),
		prerender: {
			handleMissingId: ({ id, path, referrers, message }) => {
				// Allow missing navigation IDs (about, programs, facilities, contact) on class pages
				const navigationIds = ['about', 'programs', 'facilities', 'contact', 'classes'];
				const isClassPage = path.startsWith('/classes/');
				
				if (isClassPage && navigationIds.includes(id)) {
					// Suppress error for navigation links on class pages
					console.warn(`Missing navigation ID '${id}' on ${path} - this is expected for class pages`);
					return;
				}
				
				// For other missing IDs, throw the error
				throw new Error(message);
			}
		}
	}
};

export default config;
