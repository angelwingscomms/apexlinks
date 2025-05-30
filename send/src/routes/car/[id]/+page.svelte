<script lang="ts">
	import { onMount } from 'svelte';
	
	interface Car {
		id: string;
		name: string;
		year: string;
		model: string;
		specs: {
			engine: string;
			transmission: string;
			mileage: string;
			lastService: string;
		};
	}
	
	// Mocked car data - would come from API in real app
	let car: Car = {
		id: '1',
		name: 'My Toyota',
		year: '2020',
		model: 'Camry',
		specs: {
			engine: 'V6 3.5L',
			transmission: 'Automatic',
			mileage: '35,000 mi',
			lastService: '2023-12-15'
		}
	};
	
	let loading = true;
	
	onMount(() => {
		// Simulate API fetch
		setTimeout(() => {
			loading = false;
		}, 500);
	});
</script>

<div>
	<a href="/" class="flex items-center gap-2 mb-6 text-secondary hover:text-primary transition-all">
		<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
		</svg>
		Back to Home
	</a>
	
	{#if loading}
		<div class="neumorphic p-8 flex justify-center">
			<div class="animate-pulse">Loading car profile...</div>
		</div>
	{:else}
		<div class="neumorphic p-6">
			<div class="h-48 bg-gradient-dark rounded-lg mb-4"></div>
			
			<h1 class="text-2xl font-bold mb-2">{car.name}</h1>
			<p class="text-secondary mb-6">{car.year} {car.model}</p>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
				<div class="neumorphic p-4">
					<h3 class="text-sm text-secondary mb-1">Engine</h3>
					<p>{car.specs.engine}</p>
				</div>
				
				<div class="neumorphic p-4">
					<h3 class="text-sm text-secondary mb-1">Transmission</h3>
					<p>{car.specs.transmission}</p>
				</div>
				
				<div class="neumorphic p-4">
					<h3 class="text-sm text-secondary mb-1">Mileage</h3>
					<p>{car.specs.mileage}</p>
				</div>
				
				<div class="neumorphic p-4">
					<h3 class="text-sm text-secondary mb-1">Last Service</h3>
					<p>{car.specs.lastService}</p>
				</div>
			</div>
			
			<div class="flex gap-4">
				<a href="/service/maintenance" class="btn flex-1">Maintenance</a>
				<a href="/service/repair" class="btn flex-1">Repairs</a>
			</div>
		</div>
	{/if}
</div> 