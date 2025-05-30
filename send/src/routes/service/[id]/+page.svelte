<script lang="ts">
	import { onMount } from 'svelte';
	import { getServices } from '$lib/api';
	import RatingStars from '$lib/components/RatingStars.svelte';
	
	interface Service {
		id: string;
		name: string;
		rating: number;
		image: string;
		description?: string;
	}
	
	let service: Service | null = null;
	let loading = true;
	let error = false;
	
	onMount(async () => {
		try {
			const id = window.location.pathname.split('/').pop();
			// In a real app, we'd fetch just this service by ID
			// This is a workaround since we only have the getServices endpoint
			const services = await getServices('all');
			service = services.find((s: Service) => s.id === id) || null;
			
			if (!service) {
				error = true;
			}
		} catch (e) {
			error = true;
			console.error('Error fetching service:', e);
		} finally {
			loading = false;
		}
	});
</script>

<div>
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
	{:else if error || !service}
		<div class="neumorphic p-8 text-center">
			<p>Service not found or an error occurred.</p>
			<a href="/" class="btn mt-4 inline-block">Go Home</a>
		</div>
	{:else}
		<div class="neumorphic p-6">
			<div class="h-48 bg-gradient-dark rounded-lg mb-4"></div>
			
			<h1 class="text-2xl font-bold mb-2">{service.name}</h1>
			
			<div class="mb-6">
				<RatingStars rating={service.rating} />
			</div>
			
			<p class="text-secondary mb-6">
				{service.description || 'Professional service provider specializing in vehicle maintenance and repairs. Available for emergency calls and scheduled appointments.'}
			</p>
			
			<button class="btn w-full">Request</button>
		</div>
	{/if}
</div> 