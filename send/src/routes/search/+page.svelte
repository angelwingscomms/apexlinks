<script lang="ts">
	import { onMount } from 'svelte';
	import { getServices, searchServices } from '$lib/api';
	import ServiceCard from '$lib/components/ServiceCard.svelte';
	
	interface Service {
		id: string;
		name: string;
		rating: number;
		image: string;
		description?: string;
	}
	
	let services: Service[] = [];
	let searchType = '';
	let searchQuery = '';
	let loading = $state(true);
	let filters = $state({
		minRating: 0,
		maxPrice: 0,
		category: ''
	});
	
	// Handle form submission for additional filtering
	const applyFilters = () => {
		performSearch();
	};
	
	// Clear all filters
	const clearFilters = () => {
		filters.minRating = 0;
		filters.maxPrice = 0;
		filters.category = '';
		performSearch();
	};
	
	// Main search function
	const performSearch = async () => {
		loading = true;
		
		try {
			if (searchQuery) {
				// Use semantic search with query
				const activeFilters = Object.fromEntries(
					Object.entries(filters).filter(([_, value]) => value !== 0 && value !== '')
				);
				services = await searchServices(searchQuery, activeFilters);
			} else if (searchType) {
				// Use category-based search
				services = await getServices(searchType);
			} else {
				// Default to showing mechanics if no query or type
				services = await getServices('mechanics');
				searchType = 'mechanics';
			}
		} catch (error) {
			console.error('Failed to fetch services:', error);
		} finally {
			loading = false;
		}
	};
	
	onMount(async () => {
		const url = new URL(window.location.href);
		searchType = url.searchParams.get('type') || '';
		searchQuery = url.searchParams.get('q') || '';
		
		if (url.searchParams.has('minRating')) {
			filters.minRating = Number(url.searchParams.get('minRating'));
		}
		
		if (url.searchParams.has('maxPrice')) {
			filters.maxPrice = Number(url.searchParams.get('maxPrice'));
		}
		
		if (url.searchParams.has('category')) {
			filters.category = url.searchParams.get('category') || '';
		}
		
		await performSearch();
	});
</script>

<div>
	<a href="/" class="flex items-center gap-2 mb-6 text-secondary hover:text-primary transition-all">
		<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
		</svg>
		Back to Home
	</a>
	
	<h1 class="text-2xl font-bold mb-6">
		{#if searchQuery}
			Search Results for "{searchQuery}"
		{:else}
			{searchType.charAt(0).toUpperCase() + searchType.slice(1)} Services
		{/if}
	</h1>
	
	<!-- Search filters -->
	<div class="neumorphic p-4 mb-6">
		<div class="flex flex-col md:grid-cols-2 gap-4">
			<div>
				<label for="minRating" class="block mb-1 text-sm">Minimum Rating</label>
				<select 
					id="minRating" 
					bind:value={filters.minRating}
					class="select-flowbite"
				>
					<option value="0">Any rating</option>
					<option value="3">3+ stars</option>
					<option value="4">4+ stars</option>
					<option value="5">5 stars</option>
				</select>
			</div>
			
			<div>
				<label for="maxPrice" class="block mb-1 text-sm">Max Price</label>
				<select 
					id="maxPrice" 
					bind:value={filters.maxPrice}
					class="select-flowbite"
				>
					<option value="0">Any price</option>
					<option value="50">Up to $50</option>
					<option value="100">Up to $100</option>
					<option value="200">Up to $200</option>
				</select>
			</div>
			
			<div>
				<label for="category" class="block mb-1 text-sm">Category</label>
				<select 
					id="category" 
					bind:value={filters.category}
					class="select-flowbite"
				>
					<option value="">All categories</option>
					<option value="mechanics">Mechanics</option>
					<option value="towing">Towing</option>
					<option value="carwash">Car Wash</option>
					<option value="fuel">Fuel Delivery</option>
				</select>
			</div>
			
			<div class="flex gap-2 items-end">
				<button on:click={applyFilters} class="btn flex-1">Apply Filters</button>
				<button on:click={clearFilters} class="btn">Clear</button>
			</div>
		</div>
	</div>
	
	{#if loading}
		<div class="neumorphic p-8 flex justify-center">
			<div class="animate-pulse">Loading services...</div>
		</div>
	{:else if services.length === 0}
		<div class="neumorphic p-8 text-center">
			<p>No services found. Try adjusting your search or filters.</p>
		</div>
	{:else}
		{#each services as service}
			<ServiceCard {service} />
		{/each}
	{/if}
</div> 