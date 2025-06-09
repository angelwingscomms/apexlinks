<script lang="ts">
	import '../app.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import UpdateNotification from '$lib/components/UpdateNotification.svelte';
	import OfflineIndicator from '$lib/components/OfflineIndicator.svelte';
	import InstallPrompt from '$lib/components/InstallPrompt.svelte';
	import { onMount } from 'svelte';
	import { userStore } from '$lib/stores/userStore';
	import { initWebVitals } from '$lib/utils/webVitals';
	import AnimationInit from '$lib/components/AnimationInit.svelte';

	let { children } = $props();
	
	onMount(() => {
		// Initialize user store to check for existing session
		userStore.initialize();
		
		// Initialize web vitals tracking
		initWebVitals();
	});
</script>

<div class="site-container">
	<AnimationInit />
	<Navbar />
	<OfflineIndicator />
	
	<main class="container mx-auto px-4 py-8">
		{@render children()}
	</main>
	
	<footer class="py-8 px-4 bg-transparent">
		<div class="container mx-auto flex flex-col items-center">
			<div class="mb-4">
				<a href="/" class="text-2xl font-bold dreamy-text">ApexLinks</a>
			</div>
			<nav class="mb-4">
				<ul class="flex flex-wrap justify-center gap-6">
					<li><a href="/about" class="text-gray-700 hover:text-primary">About</a></li>
					<li><a href="/legal/privacy" class="text-gray-700 hover:text-primary">Privacy</a></li>
					<li><a href="/legal/terms" class="text-gray-700 hover:text-primary">Terms</a></li>
					<li><a href="/contact" class="text-gray-700 hover:text-primary">Contact</a></li>
				</ul>
			</nav>
			<p class="text-gray-600 mb-2">© {new Date().getFullYear()} ApexLinks - Connect with like-minded individuals</p>
		</div>
	</footer>
	
	<UpdateNotification />
	<InstallPrompt />
</div>

<style>
	:global(html, body) {
		margin: 0;
		padding: 0;
		min-height: 100vh;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen-Sans, Ubuntu, Cantarell, 'Helvetica Neue', sans-serif;
		background-image: url('/images/background.jpg');
		background-size: cover;
		background-position: center;
		background-attachment: fixed;
		color: var(--color-text-primary);
	}

	:global(body) {
		transition: background-image 1s ease;
	}

	.site-container {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}
</style>
