<script lang="ts">
	import { googleAuth } from '$lib/api';

	let selectedService = 'mechanics';
	const services = [
		{ value: 'mechanics', label: 'Mechanics' },
		{ value: 'towing', label: 'Towing' },
		{ value: 'carwash', label: 'Car Wash' },
		{ value: 'fuel', label: 'Fuel Delivery' }
	];
	
	const handleSearch = () => {
		window.location.href = `/search?type=${selectedService}`;
	};
	
	const handleLogin = () => {
		googleAuth();
	};
</script>

<div class="flex flex-col items-center">
	<div class="neumorphic w-16 h-16 flex items-center justify-center mb-6 rounded-full">
		<span class="text-2xl font-bold">SM</span>
	</div>
	
	<h1 class="text-3xl font-bold mb-6 text-center">
		SendMe, get fast reliable vehicles services
	</h1>
	
	<div class="neumorphic p-6 w-full max-w-md mb-8">
		<div class="mb-4">
			<label for="service-select" class="block mb-2 text-sm">Select Service</label>
			<select 
				id="service-select"
				bind:value={selectedService}
				class="select-flowbite"
			>
				{#each services as service}
					<option value={service.value}>{service.label}</option>
				{/each}
			</select>
		</div>
		
		<button on:click={handleSearch} class="btn w-full">
			Search Near Me
		</button>
	</div>
	
	<div class="neumorphic p-6 w-full max-w-md">
		<h2 class="text-xl font-bold mb-3">About Us</h2>
		<p class="text-secondary mb-4">
			SendMe connects you with trusted vehicle service providers in your area. 
			From emergency repairs to routine maintenance, we've got you covered.
		</p>
		<button on:click={handleLogin} class="btn w-full">
			Sign in with Google
		</button>
	</div>
</div>
