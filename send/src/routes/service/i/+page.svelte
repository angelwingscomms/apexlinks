<script lang="ts">
	import { onMount } from 'svelte';
	import { getServiceById } from '$lib/api';
	import Header from '$lib/components/Header.svelte';
	import RatingStars from '$lib/components/RatingStars.svelte';
	
	// Using $props() for Svelte 5 compatibility
	const props = $props();
	
	// Make interface match what's returned from API
	interface ServiceDetails {
		id: string;
		name: string;
		description: string;
		rating: number;
		price: number;
		image: string;
		location: string;
		type: string;
		hours?: string;
		phone?: string;
		address?: string;
		services?: string[];
	}
	
	let service: ServiceDetails | null = $state(null);
	let loading = $state(true);
	let error = $state('');
	
	onMount(async () => {
		try {
			// Get the 'i' parameter from the URL query string
			const params = new URLSearchParams(window.location.search);
			const id = params.get('i');
			if (!id) {
				error = 'Service ID not found';
				loading = false;
				return;
			}
			
			const result = await getServiceById(id);
			if (!result) {
				error = 'Service not found';
			} else {
				// Cast the result to our interface
				service = result as unknown as ServiceDetails;
			}
		} catch (err) {
			console.error('Error loading service:', err);
			error = 'Failed to load service details';
		} finally {
			loading = false;
		}
	});
	
	const handleRequest = () => {
		alert('Service request feature will be implemented soon!');
	};
</script>

<Header activeTab="none" />

<div class="container mx-auto px-4 py-8">
	<a href="/search" class="flex items-center gap-2 mb-6 text-secondary hover:text-primary transition-all">
		<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
		</svg>
		Back to Search
	</a>
	
	{#if loading}
		<div class="neumorphic p-8 flex justify-center">
			<div class="animate-pulse">Loading service details...</div>
		</div>
	{:else if error}
		<div class="neumorphic p-8 text-center">
			<p class="text-secondary">{error}</p>
			<a href="/" class="btn mt-4">Go Home</a>
		</div>
	{:else if service}
		<div class="neumorphic p-6 mb-6">
			<div class="flex flex-col md:flex-row gap-6">
				<div class="w-full md:w-1/3 lg:w-1/4">
					<div class="aspect-square bg-gradient-dark rounded"></div>
				</div>
				
				<div class="flex-1">
					<h1 class="text-2xl font-bold mb-2">{service.name}</h1>
					<div class="flex items-center gap-2 mb-4">
						<RatingStars rating={service.rating} />
						<span class="pill bg-gradient-dark px-3 py-1 text-xs">
							${service.price}
						</span>
					</div>
					
					<p class="text-secondary mb-6">{service.description}</p>
					
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
						<div>
							<h3 class="text-sm font-semibold mb-1">Location</h3>
							<p class="text-secondary">{service.address}</p>
							<p class="text-xs text-secondary">({service.location})</p>
						</div>
						
						<div>
							<h3 class="text-sm font-semibold mb-1">Hours</h3>
							<p class="text-secondary">{service.hours}</p>
						</div>
						
						<div>
							<h3 class="text-sm font-semibold mb-1">Contact</h3>
							<p class="text-secondary">{service.phone}</p>
						</div>
						
						<div>
							<h3 class="text-sm font-semibold mb-1">Type</h3>
							<p class="text-secondary capitalize">{service.type}</p>
						</div>
					</div>
					
					<button onclick={handleRequest} class="btn w-full md:w-auto">
						Request Service
					</button>
				</div>
			</div>
		</div>
		
		{#if service.services && service.services.length > 0}
			<div class="neumorphic p-6">
				<h2 class="text-xl font-bold mb-4">Services Offered</h2>
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
					{#each service.services as item}
						<div class="pill bg-gradient-dark p-3 text-center">
							{item}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</div> 