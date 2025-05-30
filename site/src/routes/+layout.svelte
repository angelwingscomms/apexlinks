<script lang="ts">
	import '../app.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import UpdateNotification from '$lib/components/UpdateNotification.svelte';
	import OfflineIndicator from '$lib/components/OfflineIndicator.svelte';
	import InstallPrompt from '$lib/components/InstallPrompt.svelte';
	import { onMount } from 'svelte';
	import { userStore } from '$lib/stores/userStore';
	import { initWebVitals } from '$lib/utils/webVitals';

	let { children } = $props();
	
	onMount(() => {
		// Initialize user store to check for existing session
		userStore.initialize();
		
		// Initialize web vitals tracking
		initWebVitals();
	});
</script>

<div class="dreamy-bg min-h-screen">
	<Navbar />
	<OfflineIndicator />
	
	<main class="container mx-auto px-4 py-8">
		{@render children()}
	</main>
	
	<footer class="py-8 px-4 bg-transparent">
		<div class="container mx-auto flex flex-col items-center">
			<div class="mb-4">
				<a href="/" class="text-2xl font-bold dreamy-text">RedMoon</a>
			</div>
			<nav class="mb-4">
				<ul class="flex flex-wrap justify-center gap-6">
					<li><a href="/about" class="text-gray-700 hover:text-primary">About</a></li>
					<li><a href="/legal/privacy" class="text-gray-700 hover:text-primary">Privacy</a></li>
					<li><a href="/legal/terms" class="text-gray-700 hover:text-primary">Terms</a></li>
					<li><a href="/contact" class="text-gray-700 hover:text-primary">Contact</a></li>
				</ul>
			</nav>
			<p class="text-gray-600 mb-2">© {new Date().getFullYear()} RedMoon - Connect with like-minded individuals</p>
		</div>
	</footer>
	
	<UpdateNotification />
	<InstallPrompt />
</div>
