<script lang="ts">
	import { onMount } from 'svelte';
	import { isAuthenticated } from '$lib/stores/userStore';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { slideInFromBottom, createScrollObserver, staggeredElementsOnScroll } from '$lib/animations.js';
	import axios from 'axios';
	
	// Types
	interface Item {
		id: string;
		name: string;
		description: string;
		type: 'product' | 'service';
		primaryImage: string;
		additionalImages: string[];
		userId: string;
		userName: string;
		userImage: string;
		customFields: Array<{
			id: string;
			label: string;
			type: 'text' | 'number';
			value: string | number;
		}>;
		createdAt: string;
	}
	
	// State
	let searchQuery = $state('');
	let items = $state<Item[]>([]);
	let filteredItems = $state<Item[]>([]);
	let loading = $state(true);
	let error = $state('');
	let searchType = $state<'product' | 'service' | 'all'>('all');
	let priceRange = $state<[number, number]>([0, 10000]);
	let showFilters = $state(false);
	
	// Animation references
	let resultsContainer: HTMLElement;
	
	// Get search query from URL
	onMount(async () => {
		const url = new URL(window.location.href);
		const queryParam = url.searchParams.get('q');
		const itemIdParam = url.searchParams.get('i');
		
		if (itemIdParam) {
			// If an item ID is provided, redirect to the item detail page
			window.location.href = `/i?i=${itemIdParam}`;
			return;
		}
		
		if (queryParam) {
			searchQuery = queryParam;
		}
		
		try {
			const response = await axios.get('https://api.apexlinks.org/items');
			items = response.data.items || [];
			performSearch();
		} catch (err) {
			console.error('Error fetching items:', err);
			error = 'Failed to load items. Please try again later.';
		} finally {
			loading = false;
			
			// Apply animations
			if (resultsContainer) {
				staggeredElementsOnScroll(resultsContainer.querySelectorAll('.item-card'));
			}
			
			// Setup scroll-based animations
			createScrollObserver('.item-card', slideInFromBottom, { once: true, staggerDelay: 100 });
		}
	});
	
	// Filter items based on search query and filters
	function performSearch() {
		if (!items.length) {
			filteredItems = [];
			return;
		}
		
		// Apply all filters
		filteredItems = items.filter(item => {
			// Filter by search query
			const matchesQuery = !searchQuery.trim() || 
				item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				item.description.toLowerCase().includes(searchQuery.toLowerCase());
			
			// Filter by type (product/service/all)
			const matchesType = searchType === 'all' || item.type === searchType;
			
			// Filter by price range (if price custom field exists)
			let matchesPrice = true;
			const priceField = item.customFields.find(field => 
				field.label.toLowerCase() === 'price' || 
				field.label.toLowerCase().includes('price')
			);
			
			if (priceField) {
				const price = parseFloat(priceField.value.toString());
				matchesPrice = price >= priceRange[0] && price <= priceRange[1];
			}
			
			return matchesQuery && matchesType && matchesPrice;
		});
	}
	
	// Handle search form submission
	function handleSearch() {
		performSearch();
		
		// Update URL with search query
		const url = new URL(window.location.href);
		if (searchQuery.trim()) {
			url.searchParams.set('q', searchQuery);
		} else {
			url.searchParams.delete('q');
		}
		window.history.pushState({}, '', url.toString());
	}
	
	// Toggle filter panel
	function toggleFilters() {
		showFilters = !showFilters;
	}
	
	// Reset all filters
	function resetFilters() {
		searchType = 'all';
		priceRange = [0, 10000];
		performSearch();
	}
	
	// Helper function to truncate text
	function truncateText(text: string, length: number): string {
		if (!text) return '';
		return text.length > length ? text.substring(0, length) + '...' : text;
	}
</script>

<PageTransition>
	<div class="min-h-screen bg-[#151A33] text-[#FDFDFF] pb-16">
		<!-- Background elements -->
		<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		
		<div class="container mx-auto px-4 py-8">
			<header class="mb-12 text-center">
				<h1 class="text-4xl font-bold text-[#FDFDFF] mb-4">Discover Items</h1>
				<p class="text-xl text-[#C9CCF5] max-w-2xl mx-auto">
					Find products and services that match your needs
				</p>
			</header>
			
			<!-- Search Form -->
			<div class="max-w-3xl mx-auto mb-10">
				<div class="glass p-6 rounded-2xl">
					<div class="flex flex-col md:flex-row gap-4">
						<div class="flex-1">
							<input 
								type="text" 
								bind:value={searchQuery}
								placeholder="Search for products or services..."
								class="w-full p-4 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
								on:keydown={(e) => e.key === 'Enter' && handleSearch()}
							/>
						</div>
						<div class="flex gap-2">
							<button 
								on:click={handleSearch}
								class="glass-button px-6 py-3 text-[#FDFDFF] font-medium"
							>
								Search
							</button>
							<button 
								on:click={toggleFilters}
								class="glass-button px-4 py-3 text-[#FDFDFF]"
							>
								<span class="sr-only">Filters</span>
								<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
									<path fill-rule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v3a1 1 0 01-.293.707L12 11.414V15a1 1 0 01-.293.707l-2 2A1 1 0 018 17v-5.586L3.293 6.707A1 1 0 013 6V3z" clip-rule="evenodd" />
								</svg>
							</button>
						</div>
					</div>
					
					{#if showFilters}
						<div class="mt-6 pt-6 border-t border-[#B2B5E0]/30 grid grid-cols-1 md:grid-cols-2 gap-6">
							<div>
								<h3 class="text-lg font-medium text-[#F4E6FF] mb-3">Item Type</h3>
								<div class="flex gap-3">
									<label class="flex items-center">
										<input 
											type="radio" 
											name="searchType" 
											value="all" 
											bind:group={searchType} 
											class="mr-2"
										/>
										<span>All</span>
									</label>
									<label class="flex items-center">
										<input 
											type="radio" 
											name="searchType" 
											value="product" 
											bind:group={searchType} 
											class="mr-2"
										/>
										<span>Products</span>
									</label>
									<label class="flex items-center">
										<input 
											type="radio" 
											name="searchType" 
											value="service" 
											bind:group={searchType} 
											class="mr-2"
										/>
										<span>Services</span>
									</label>
								</div>
							</div>
							
							<div>
								<h3 class="text-lg font-medium text-[#F4E6FF] mb-3">Price Range</h3>
								<div class="flex items-center gap-3">
									<input 
										type="number" 
										bind:value={priceRange[0]} 
										min="0" 
										class="w-24 p-2 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF]"
									/>
									<span>to</span>
									<input 
										type="number" 
										bind:value={priceRange[1]} 
										min="0" 
										class="w-24 p-2 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF]"
									/>
								</div>
							</div>
							
							<div class="md:col-span-2 flex justify-end">
								<button 
									on:click={resetFilters}
									class="text-[#A3F1FF] hover:underline mr-4"
								>
									Reset Filters
								</button>
								<button 
									on:click={() => {
										performSearch();
										showFilters = false;
									}}
									class="glass-button px-4 py-2 text-[#FDFDFF]"
								>
									Apply Filters
								</button>
							</div>
						</div>
					{/if}
				</div>
			</div>
			
			<!-- Search Results -->
			<div bind:this={resultsContainer}>
				{#if loading}
					<div class="flex justify-center items-center py-20">
						<div class="animate-spin mr-3">⟳</div>
						<span>Loading items...</span>
					</div>
				{:else if error}
					<div class="glass p-8 text-center">
						<p class="text-xl text-red-400 mb-4">{error}</p>
						<button 
							on:click={() => window.location.reload()}
							class="glass-button px-4 py-2 text-[#FDFDFF]"
						>
							Try Again
						</button>
					</div>
				{:else if filteredItems.length === 0}
					<div class="glass p-8 text-center">
						<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">No Items Found</h2>
						<p class="text-[#C9CCF5] mb-6">
							{searchQuery 
								? `We couldn't find any items matching "${searchQuery}". Try different keywords or adjust your filters.` 
								: 'No items are available at the moment. Be the first to add one!'}
						</p>
						<a 
							href="/i/add" 
							class="glass-button px-6 py-3 text-[#FDFDFF] inline-block"
						>
							Add an Item
						</a>
					</div>
				{:else}
					<div class="mb-6 flex justify-between items-center">
						<h2 class="text-2xl font-semibold text-[#F4E6FF]">
							{filteredItems.length} {filteredItems.length === 1 ? 'Item' : 'Items'} Found
						</h2>
						{#if $isAuthenticated}
							<a 
								href="/i/add" 
								class="glass-button px-4 py-2 text-[#FDFDFF]"
							>
								+ Add Item
							</a>
						{/if}
					</div>
					
					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
						{#each filteredItems as item}
							<a 
								href={`/i?i=${item.id}`}
								class="item-card glass hover:scale-105 transition-transform duration-300 overflow-hidden rounded-2xl flex flex-col"
							>
								<div class="h-48 overflow-hidden">
									<img 
										src={item.primaryImage} 
										alt={item.name} 
										class="w-full h-full object-cover"
									/>
								</div>
								<div class="p-5 flex-1 flex flex-col">
									<div class="flex justify-between items-start mb-2">
										<h3 class="text-xl font-semibold text-[#F4E6FF] truncate">
											{item.name}
										</h3>
										<span class={`text-xs px-2 py-1 rounded-full ${item.type === 'product' ? 'bg-[#A3F1FF]/20 text-[#A3F1FF]' : 'bg-[#E5D9FF]/20 text-[#E5D9FF]'}`}>
											{item.type}
										</span>
									</div>
									<p class="text-[#C9CCF5] text-sm mb-4 flex-1">
										{truncateText(item.description, 120)}
									</p>
									<div class="flex items-center">
										<img 
											src={item.userImage} 
											alt={item.userName} 
											class="w-8 h-8 rounded-full mr-2"
										/>
										<span class="text-sm text-[#B2B5E0]">
											{item.userName}
										</span>
									</div>
								</div>
							</a>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
</PageTransition>

<style>
	@theme {
		--glass-opacity: 0.15;
		--glass-blur: 15px;
		--glass-border: 1px solid rgba(255, 255, 255, 0.2);
		--glass-shadow: 0 8px 32px rgba(163, 241, 255, 0.2);
	}
	
	:global(.glass-button) {
		background: rgba(42, 46, 110, 0.25);
		backdrop-filter: blur(15px);
		border: 1px solid rgba(163, 241, 255, 0.3);
		box-shadow: 0 8px 16px rgba(42, 46, 110, 0.15);
		border-radius: 1rem;
		transition: all 0.3s ease;
	}
	
	:global(.glass-button:hover:not(:disabled)) {
		background: rgba(42, 46, 110, 0.35);
		box-shadow: 0 8px 25px rgba(163, 241, 255, 0.25);
		transform: translateY(-2px);
	}
</style> 