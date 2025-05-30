<script lang="ts">
	import { onMount } from 'svelte';
	import { getServices } from '$lib/api';
	import ServiceCard from '$lib/components/ServiceCard.svelte';
	
	interface Service {
		id: string;
		name: string;
		rating: number;
		image: string;
	}
	
	let services: Service[] = [];
	let type = '';
	let loading = true;
	
	onMount(async () => {
		const url = new URL(window.location.href);
		type = url.searchParams.get('type') || 'mechanics';
		
		try {
			services = await getServices(type);
		} catch (error) {
			console.error('Failed to fetch services:', error);
		} finally {
			loading = false;
		}
	});
</script>

<div>
	<a href="/" class="flex items-center gap-2 mb-6 text-secondary hover:text-primary transition-all">
		<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
		</svg>
		Search Results
	</a>
	
	<h1 class="text-2xl font-bold mb-6">
		{type.charAt(0).toUpperCase() + type.slice(1)} Services
	</h1>
	
	{#if loading}
		<div class="neumorphic p-8 flex justify-center">
			<div class="animate-pulse">Loading services...</div>
		</div>
	{:else if services.length === 0}
		<div class="neumorphic p-8 text-center">
			<p>No services found. Try a different category.</p>
		</div>
	{:else}
		{#each services as service}
			<ServiceCard {service} />
		{/each}
	{/if}
</div> 