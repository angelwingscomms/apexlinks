<script lang="ts">
	import '../app.css';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { handleAuthCallback } from '$lib/api';
	import { onMount } from 'svelte';

	let { children } = $props();
	
	// Get current path for active tab
	let path = '';
	let activeTab = $state('home');
	
	$effect(() => {
		path = window.location.pathname;
		activeTab = path === '/' ? 'home' : 
			path.startsWith('/contact') ? 'contact' : 
			path.startsWith('/more') ? 'more' : 'home';
	});
	
	onMount(() => {
		// Handle Google OAuth callback if present
		handleAuthCallback();
	});
</script>

<div class="min-h-screen flex flex-col p-4 max-w-screen-lg mx-auto">
	<Header {activeTab} />
	<main class="flex-1">
		{@render children()}
	</main>
	<Footer />
</div>
