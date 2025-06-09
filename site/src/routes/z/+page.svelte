<script lang="ts">
	import { onMount } from 'svelte';
	import { isAuthenticated } from '$lib/stores/userStore';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { slideInFromBottom, createScrollObserver, staggeredElementsOnScroll } from '$lib/animations.js';
	import axios from 'axios';
	
	// Types
	interface Zone {
		id: string;
		name: string;
		description: string;
		imageUrl: string;
		location: {
			lat: number;
			lng: number;
			address: string;
		};
		createdBy: string;
		createdByName: string;
		createdByImage: string;
		itemCount: number;
		createdAt: string;
	}
	
	// State
	let searchQuery = $state('');
	let zones = $state<Zone[]>([]);
	let filteredZones = $state<Zone[]>([]);
	let loading = $state(true);
	let error = $state('');
	
	// Animation references
	let resultsContainer: HTMLElement;
	
	// Get search query from URL
	onMount(async () => {
		const url = new URL(window.location.href);
		const queryParam = url.searchParams.get('q');
		const zoneIdParam = url.searchParams.get('i');
		
		if (zoneIdParam) {
			// If a zone ID is provided, redirect to the zone detail page
			window.location.href = `/z?i=${zoneIdParam}`;
			return;
		}
		
		if (queryParam) {
			searchQuery = queryParam;
		}
		
		try {
			const response = await axios.get('https://api.apexlinks.org/zones');
			zones = response.data.zones || [];
			performSearch();
		} catch (err) {
			console.error('Error fetching zones:', err);
			error = 'Failed to load zones. Please try again later.';
		} finally {
			loading = false;
			
			// Apply animations
			if (resultsContainer) {
				staggeredElementsOnScroll(resultsContainer.querySelectorAll('.zone-card'));
			}
			
			// Setup scroll-based animations
			createScrollObserver('.zone-card', slideInFromBottom, { once: true, staggerDelay: 100 });
		}
	});
	
	// Filter zones based on search query
	function performSearch() {
		if (!zones.length) {
			filteredZones = [];
			return;
		}
		
		if (!searchQuery.trim()) {
			filteredZones = zones;
			return;
		}
		
		// Apply search filter
		filteredZones = zones.filter(zone => {
			return zone.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				zone.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
				zone.location.address.toLowerCase().includes(searchQuery.toLowerCase());
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
	
	// Helper function to truncate text
	function truncateText(text: string, length: number): string {
		if (!text) return '';
		return text.length > length ? text.substring(0, length) + '...' : text;
	}
	
	// Helper function to format date
	function formatDate(dateString: string): string {
		if (!dateString) return '';
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<PageTransition>
	<div class="min-h-screen bg-[#151A33] text-[#FDFDFF] pb-16">
		<!-- Background elements -->
		<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		
		<div class="container mx-auto px-4 py-8">
			<header class="mb-12 text-center">
				<h1 class="text-4xl font-bold text-[#FDFDFF] mb-4">Discover Zones</h1>
				<p class="text-xl text-[#C9CCF5] max-w-2xl mx-auto">
					Find local business zones and explore what they have to offer
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
								placeholder="Search for zones by name, description, or location..."
								class="w-full p-4 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
								on:keydown={(e) => e.key === 'Enter' && handleSearch()}
							/>
						</div>
						<button 
							on:click={handleSearch}
							class="glass-button px-6 py-3 text-[#FDFDFF] font-medium"
						>
							Search
						</button>
					</div>
				</div>
			</div>
			
			<!-- Search Results -->
			<div bind:this={resultsContainer}>
				{#if loading}
					<div class="flex justify-center items-center py-20">
						<div class="animate-spin mr-3">⟳</div>
						<span>Loading zones...</span>
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
				{:else if filteredZones.length === 0}
					<div class="glass p-8 text-center">
						<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">No Zones Found</h2>
						<p class="text-[#C9CCF5] mb-6">
							{searchQuery 
								? `We couldn't find any zones matching "${searchQuery}". Try different keywords.` 
								: 'No zones are available at the moment.'}
						</p>
						{#if $isAuthenticated}
							<a 
								href="/z/create" 
								class="glass-button px-6 py-3 text-[#FDFDFF] inline-block"
							>
								Create a Zone
							</a>
						{/if}
					</div>
				{:else}
					<div class="mb-6 flex justify-between items-center">
						<h2 class="text-2xl font-semibold text-[#F4E6FF]">
							{filteredZones.length} {filteredZones.length === 1 ? 'Zone' : 'Zones'} Found
						</h2>
						{#if $isAuthenticated}
							<a 
								href="/z/create" 
								class="glass-button px-4 py-2 text-[#FDFDFF]"
							>
								+ Create Zone
							</a>
						{/if}
					</div>
					
					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
						{#each filteredZones as zone}
							<a 
								href={`/z?i=${zone.id}`}
								class="zone-card glass hover:scale-105 transition-transform duration-300 overflow-hidden rounded-2xl flex flex-col"
							>
								<div class="h-48 overflow-hidden relative">
									<img 
										src={zone.imageUrl} 
										alt={zone.name} 
										class="w-full h-full object-cover"
										on:error={(e) => {
											const target = e.target as HTMLImageElement;
											target.src = "https://via.placeholder.com/800x400/2A2E6E/FDFDFF?text=Zone";
										}}
									/>
									<div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-[#151A33] to-transparent h-24"></div>
									<div class="absolute bottom-4 left-4 right-4">
										<h3 class="text-xl font-semibold text-[#FDFDFF] truncate">
											{zone.name}
										</h3>
										<p class="text-sm text-[#C9CCF5] truncate">
											{zone.location.address}
										</p>
									</div>
								</div>
								<div class="p-5 flex-1 flex flex-col">
									<p class="text-[#C9CCF5] text-sm mb-4 flex-1">
										{truncateText(zone.description, 120)}
									</p>
									<div class="flex items-center justify-between">
										<div class="flex items-center">
											<img 
												src={zone.createdByImage} 
												alt={zone.createdByName} 
												class="w-8 h-8 rounded-full mr-2"
												on:error={(e) => {
													const target = e.target as HTMLImageElement;
													target.src = `https://ui-avatars.com/api/?name=${encodeURIComponent(zone.createdByName)}&background=2A2E6E&color=FDFDFF&size=64`;
												}}
											/>
											<span class="text-sm text-[#B2B5E0]">
												{zone.createdByName}
											</span>
										</div>
										<span class="text-xs text-[#A3F1FF]">
											{zone.itemCount} items
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