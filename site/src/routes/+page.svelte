<script lang="ts">
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	
	// Service categories
	const serviceCategories = [
		{ value: 'mechanics', label: 'Mechanics', icon: '🔧', description: 'Find professional mechanics for your vehicle repairs and maintenance' },
		{ value: 'towing', label: 'Towing Services', icon: '🚗', description: 'Get towing assistance for breakdowns and emergencies' },
		{ value: 'cleaning', label: 'Cleaning', icon: '✨', description: 'Professional cleaning services for your home or office' },
		{ value: 'delivery', label: 'Delivery', icon: '📦', description: 'Fast and reliable delivery services for all your needs' }
	];
	
	let selectedCategory = $state('');
	let searchQuery = $state('');
	
	function handleCategorySearch() {
		if (selectedCategory) {
			window.location.href = `/services?type=${selectedCategory}`;
		}
	}
	
	function handleTextSearch() {
		if (searchQuery.trim()) {
			window.location.href = `/services?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}
	
	function handleLogin() {
		userStore.startGoogleAuth();
	}
</script>

<div class="min-h-[80vh] flex flex-col items-center justify-center py-12">
	<!-- Hero Section -->
	<div class="text-center mb-12">
		<h1 class="text-5xl md:text-6xl font-bold dreamy-text mb-4">
			Find the Perfect Service
		</h1>
		<p class="text-xl text-gray-700 max-w-3xl mx-auto">
			Discover top-rated services that meet your needs. From mechanics to delivery, we've got you covered.
		</p>
	</div>
	
	<!-- Search Section -->
	<div class="card-glass w-full max-w-2xl mx-auto mb-12 p-6">
		<div class="mb-4">
			<label for="search" class="block text-gray-700 mb-2">Search Services</label>
			<div class="flex">
				<input 
					type="text" 
					id="search"
					bind:value={searchQuery}
					placeholder="What service are you looking for?"
					class="flex-1 neumorphic p-3 rounded-l-lg"
				/>
				<button 
					on:click={handleTextSearch}
					class="btn-primary rounded-r-lg px-6"
				>
					Search
				</button>
			</div>
		</div>
		
		<div>
			<p class="text-gray-700 mb-2">Or browse by category:</p>
			<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
				{#each serviceCategories as category}
					<button 
						class="card-glass p-4 text-center transition-all hover:scale-105"
						on:click={() => {
							selectedCategory = category.value;
							handleCategorySearch();
						}}
					>
						<div class="text-3xl mb-2">{category.icon}</div>
						<div class="font-medium">{category.label}</div>
					</button>
				{/each}
			</div>
		</div>
	</div>
	
	<!-- Service Categories -->
	<div class="w-full max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
		{#each serviceCategories as category}
			<div class="card-glass p-6 flex gap-4">
				<div class="glass-sm h-16 w-16 rounded-full flex items-center justify-center text-2xl">
					{category.icon}
				</div>
				<div>
					<h3 class="text-xl font-semibold dreamy-text mb-2">{category.label}</h3>
					<p class="text-gray-600 mb-3">{category.description}</p>
					<a 
						href={`/services?type=${category.value}`}
						class="text-primary font-medium hover:underline"
					>
						Explore {category.label} →
					</a>
				</div>
			</div>
		{/each}
	</div>
	
	<!-- CTA Section -->
	{#if !$isAuthenticated}
		<div class="text-center">
			<button 
				on:click={handleLogin}
				class="btn-primary text-lg px-8 py-4"
			>
				Sign in to Access All Services
			</button>
			<p class="mt-4 text-gray-500">Free and secure access to all service providers</p>
		</div>
	{:else}
		<div class="text-center">
			<a 
				href="/services"
				class="btn-primary text-lg px-8 py-4 inline-block"
			>
				Browse All Services
			</a>
		</div>
	{/if}
</div>

<!-- Static Background Elements (non-floating) -->
<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-red-200/30 blur-3xl"></div>
<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-yellow-200/30 blur-3xl"></div>
<div class="fixed -z-10 top-1/2 right-1/3 w-48 h-48 rounded-full bg-purple-200/30 blur-3xl"></div> 