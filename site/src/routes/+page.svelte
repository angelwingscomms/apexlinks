<script lang="ts">
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	
	function handleLogin() {
		userStore.startGoogleAuth();
	}
</script>

<div class="min-h-[80vh] flex flex-col items-center justify-center py-12">
	<!-- Hero Section -->
	<div class="text-center mb-12">
		<h1 class="text-5xl md:text-6xl font-bold dreamy-text mb-4">
			Connect with Like-Minded People
		</h1>
		<p class="text-xl text-gray-700 max-w-3xl mx-auto">
			Discover new connections based on shared interests and compatibility. 
			Join RedMoon to find your perfect match in our community.
		</p>
	</div>
	
	<!-- Main Feature Cards -->
	<div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto mb-16">
		<!-- Feature Card 1 -->
		<div class="card-glass">
			<div class="glass-sm h-40 w-40 rounded-full mx-auto mb-6 overflow-hidden flex items-center justify-center">
				<img src="https://ui-avatars.com/api/?name=Find+Users&background=ef4444&color=fff&size=128" alt="Find Users" class="h-full w-full object-cover" />
			</div>
			<h3 class="text-xl font-semibold dreamy-text mb-2">Find Users</h3>
			<p class="text-gray-600">
				Search for users by name, interests, or profile details. 
				Our advanced search helps you discover the right connections.
			</p>
		</div>
		
		<!-- Feature Card 2 -->
		<div class="card-glass">
			<div class="glass-sm h-40 w-40 rounded-full mx-auto mb-6 overflow-hidden flex items-center justify-center">
				<img src="https://ui-avatars.com/api/?name=Compatibility&background=f97316&color=fff&size=128" alt="Compatibility" class="h-full w-full object-cover" />
			</div>
			<h3 class="text-xl font-semibold dreamy-text mb-2">Check Compatibility</h3>
			<p class="text-gray-600">
				See how well you match with other users through our unique compatibility algorithm.
				Find your perfect match based on shared attributes.
			</p>
		</div>
		
		<!-- Feature Card 3 -->
		<div class="card-glass">
			<div class="glass-sm h-40 w-40 rounded-full mx-auto mb-6 overflow-hidden flex items-center justify-center">
				<img src="https://ui-avatars.com/api/?name=Connect&background=b91c1c&color=fff&size=128" alt="Connect" class="h-full w-full object-cover" />
			</div>
			<h3 class="text-xl font-semibold dreamy-text mb-2">Connect</h3>
			<p class="text-gray-600">
				Join zones with like-minded people and build your network.
				Share interests and connect with others in your community.
			</p>
		</div>
	</div>
	
	<!-- CTA Section -->
	{#if !$isAuthenticated}
		<div class="text-center">
			<button 
				on:click={handleLogin}
				class="btn-primary text-lg px-8 py-4"
			>
				Get Started with Google
			</button>
			<p class="mt-4 text-gray-500">It's free and takes less than a minute</p>
		</div>
	{:else}
		<div class="text-center">
			<a 
				href="/users"
				class="btn-primary text-lg px-8 py-4 inline-block"
			>
				Start Exploring
			</a>
		</div>
	{/if}
</div>

<!-- Static Background Elements (non-floating) -->
<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-red-200/30 blur-3xl"></div>
<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-yellow-200/30 blur-3xl"></div>
<div class="fixed -z-10 top-1/2 right-1/3 w-48 h-48 rounded-full bg-purple-200/30 blur-3xl"></div> 