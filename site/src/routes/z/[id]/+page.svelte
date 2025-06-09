<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { slideInFromBottom, slideInFromLeft, slideInFromRight, createScrollObserver } from '$lib/animations.js';
	import axios from 'axios';
	
	// Extend window interface for Google Maps
	declare global {
		interface Window {
			google?: any;
			initMap?: () => void;
		}
	}
	
	// Types
	interface Item {
		id: string;
		name: string;
		description: string;
		type: 'product' | 'service';
		primaryImage: string;
		userId: string;
		userName: string;
	}
	
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
		createdAt: string;
		items: Item[];
	}
	
	// State
	let zone = $state<Zone | null>(null);
	let loading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let filteredItems = $state<Item[]>([]);
	
	// Animation references
	let zoneHeader: HTMLElement;
	let zoneInfo: HTMLElement;
	let itemsSection: HTMLElement;
	let mapContainer: HTMLElement;
	
	// Handle not found - redirect to search page
	function handleNotFound() {
		window.location.href = '/z';
	}
	
	// Filter items based on search query
	function filterItems() {
		if (!zone?.items.length) {
			filteredItems = [];
			return;
		}
		
		if (!searchQuery.trim()) {
			filteredItems = zone.items;
			return;
		}
		
		filteredItems = zone.items.filter(item => {
			return item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				item.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
				item.type.toLowerCase().includes(searchQuery.toLowerCase());
		});
	}
	
	// Initialize Google Map
	function initMap() {
		if (!zone || !mapContainer) return;
		
		// If Google Maps API is loaded
		if (window.google && window.google.maps) {
			const mapOptions = {
				center: { lat: zone.location.lat, lng: zone.location.lng },
				zoom: 15,
				styles: [
					{
						"featureType": "all",
						"elementType": "geometry",
						"stylers": [{ "color": "#242f3e" }]
					},
					{
						"featureType": "all",
						"elementType": "labels.text.fill",
						"stylers": [{ "color": "#746855" }]
					},
					{
						"featureType": "water",
						"elementType": "geometry",
						"stylers": [{ "color": "#1C1F4A" }]
					}
				]
			};
			
			const map = new window.google.maps.Map(mapContainer, mapOptions);
			
			new window.google.maps.Marker({
				position: { lat: zone.location.lat, lng: zone.location.lng },
				map,
				title: zone.name,
				animation: window.google.maps.Animation.DROP
			});
		}
	}
	
	// Get zone from the URL
	onMount(async () => {
		try {
			const url = new URL(window.location.href);
			const zoneId = url.searchParams.get('i');
			
			if (!zoneId) {
				error = 'No zone ID provided';
				handleNotFound();
				return;
			}
			
			// Fetch zone data
			const response = await axios.get(`https://api.apexlinks.org/zones/${zoneId}`);
			zone = response.data.zone;
			
			if (!zone) {
				error = 'Zone not found';
				handleNotFound();
				return;
			}
			
			// Initialize filtered items
			filteredItems = zone.items || [];
			
			// Apply animations
			if (zoneHeader) slideInFromBottom(zoneHeader);
			if (zoneInfo) slideInFromLeft(zoneInfo);
			if (itemsSection) slideInFromRight(itemsSection);
			
			// Setup scroll-based animations
			createScrollObserver('.item-card', slideInFromBottom, { once: true, staggerDelay: 100 });
			
			// Initialize map if Google Maps is available
			initMap();
			
			// If Google Maps isn't loaded yet, load it
			if (!window.google || !window.google.maps) {
				const script = document.createElement('script');
				script.src = `https://maps.googleapis.com/maps/api/js?key=YOUR_API_KEY&callback=initMap`;
				script.defer = true;
				script.async = true;
				window.initMap = initMap;
				document.head.appendChild(script);
			}
		} catch (err) {
			console.error('Error fetching zone:', err);
			error = 'Failed to load zone data. Please try again later.';
		} finally {
			loading = false;
		}
	});
	
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
		<div class="fixed -z-10 top-1/2 right-1/3 w-48 h-48 rounded-full bg-[#E5D9FF]/10 blur-3xl"></div>
		
		<div class="container mx-auto px-4 py-8">
			{#if loading}
				<div class="flex justify-center items-center py-20">
					<div class="animate-spin mr-3">⟳</div>
					<span>Loading zone details...</span>
				</div>
			{:else if error}
				<div class="glass p-8 text-center">
					<p class="text-xl text-red-400 mb-4">{error}</p>
					<button 
						on:click={() => window.location.href = '/z'}
						class="glass-button px-4 py-2 text-[#FDFDFF]"
					>
						Browse Zones
					</button>
				</div>
			{:else if zone}
				<!-- Zone Header -->
				<header class="relative mb-10" bind:this={zoneHeader}>
					<div class="h-64 md:h-80 w-full rounded-2xl overflow-hidden relative">
						<img 
							src={zone.imageUrl} 
							alt={zone.name} 
							class="w-full h-full object-cover"
							on:error={(e) => {
								const target = e.target as HTMLImageElement;
								target.src = "https://via.placeholder.com/1600x800/2A2E6E/FDFDFF?text=Zone";
							}}
						/>
						<div class="absolute inset-0 bg-gradient-to-b from-[#151A33]/30 to-[#151A33]/90"></div>
						<div class="absolute bottom-8 left-8 right-8">
							<h1 class="text-4xl md:text-5xl font-bold text-[#FDFDFF] mb-2">{zone.name}</h1>
							<p class="text-xl text-[#C9CCF5]">{zone.location.address}</p>
						</div>
					</div>
				</header>
				
				<div class="grid grid-cols-1 lg:grid-cols-3 gap-10 mb-16">
					<!-- Zone Information -->
					<div class="lg:col-span-2" bind:this={zoneInfo}>
						<div class="glass p-6 rounded-2xl mb-8">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">About this Zone</h2>
							<p class="text-[#C9CCF5] whitespace-pre-line mb-6">
								{zone.description || 'No description provided.'}
							</p>
							
							<div class="flex items-center mt-8">
								<div class="flex items-center">
									<img 
										src={zone.createdByImage} 
										alt={zone.createdByName} 
										class="w-12 h-12 rounded-full mr-3 border-2 border-[#A3F1FF]/20"
										on:error={(e) => {
											const target = e.target as HTMLImageElement;
											target.src = `https://ui-avatars.com/api/?name=${encodeURIComponent(zone.createdByName)}&background=2A2E6E&color=FDFDFF&size=96`;
										}}
									/>
									<div>
										<p class="text-[#F4E6FF] font-medium">{zone.createdByName}</p>
										<p class="text-sm text-[#B2B5E0]">Created {formatDate(zone.createdAt)}</p>
									</div>
								</div>
							</div>
						</div>
						
						<!-- Map -->
						<div class="glass p-6 rounded-2xl">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Location</h2>
							<div 
								bind:this={mapContainer}
								class="h-72 rounded-lg overflow-hidden bg-[#1C1F4A]"
							>
								<div class="w-full h-full flex items-center justify-center text-[#B2B5E0]">
									Loading map...
								</div>
							</div>
						</div>
					</div>
					
					<!-- Items in Zone -->
					<div bind:this={itemsSection}>
						<div class="glass p-6 rounded-2xl">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">
								Items in this Zone
								{#if zone.items?.length}
									<span class="text-[#A3F1FF] text-lg">({zone.items.length})</span>
								{/if}
							</h2>
							
							{#if zone.items?.length}
								<div class="mb-4">
									<input 
										type="text" 
										bind:value={searchQuery}
										placeholder="Search items..."
										class="w-full p-3 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
										on:input={filterItems}
									/>
								</div>
								
								<div class="space-y-4 max-h-[600px] overflow-y-auto pr-2">
									{#each filteredItems as item}
										<a 
											href={`/i?i=${item.id}`}
											class="item-card flex gap-4 p-3 rounded-lg hover:bg-[#2A2E6E]/30 transition-colors"
										>
											<img 
												src={item.primaryImage} 
												alt={item.name} 
												class="w-16 h-16 object-cover rounded-lg flex-shrink-0"
												on:error={(e) => {
													const target = e.target as HTMLImageElement;
													target.src = "https://via.placeholder.com/200/2A2E6E/FDFDFF?text=Item";
												}}
											/>
											<div>
												<h3 class="text-[#F4E6FF] font-medium">{item.name}</h3>
												<p class="text-sm text-[#B2B5E0] mb-1">{item.userName}</p>
												<span class={`text-xs px-2 py-0.5 rounded-full ${item.type === 'product' ? 'bg-[#A3F1FF]/20 text-[#A3F1FF]' : 'bg-[#E5D9FF]/20 text-[#E5D9FF]'}`}>
													{item.type}
												</span>
											</div>
										</a>
									{/each}
								</div>
							{:else}
								<p class="text-[#C9CCF5] text-center py-6">
									No items have been added to this zone yet.
								</p>
							{/if}
							
							{#if $isAuthenticated}
								<div class="mt-6 pt-4 border-t border-[#B2B5E0]/20">
									<a 
										href={`/i/add?zone=${zone.id}`}
										class="glass-button w-full py-3 text-center text-[#FDFDFF] font-medium block"
									>
										Add Item to Zone
									</a>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/if}
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
	
	/* Scrollbar styles */
	:global(::-webkit-scrollbar) {
		width: 8px;
	}
	
	:global(::-webkit-scrollbar-track) {
		background: rgba(42, 46, 110, 0.1);
		border-radius: 4px;
	}
	
	:global(::-webkit-scrollbar-thumb) {
		background: rgba(163, 241, 255, 0.2);
		border-radius: 4px;
	}
	
	:global(::-webkit-scrollbar-thumb:hover) {
		background: rgba(163, 241, 255, 0.4);
	}
</style> 