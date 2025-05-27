<script lang="ts">
	import '../app.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import { onMount } from 'svelte';
	import { userStore } from '$lib/stores/userStore';

	let { children } = $props();
	
	onMount(() => {
		// Initialize user store to check for existing session
		userStore.initialize();
	});
</script>

<div class="dreamy-bg min-h-screen">
	<Navbar />
	
	<main class="container mx-auto px-4 py-8">
		{@render children()}
	</main>
	
	<footer class="mt-20 py-8 bg-gray-50/50 border-t border-gray-100">
		<div class="container mx-auto px-4 text-center text-gray-600">
			<p>© {new Date().getFullYear()} ApexLinks - Connect with like-minded individuals</p>
		</div>
	</footer>
</div>
