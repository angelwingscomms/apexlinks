<script lang="ts">
	import { onMount } from 'svelte';
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { slideInFromBottom, slideInFromLeft, createScrollObserver } from '$lib/animations.js';
	import axios from 'axios';
	
	// Types for custom fields
	interface CustomField {
		id: string;
		label: string;
		type: 'text' | 'number';
		value: string | number;
	}
	
	// Form data
	let itemType = $state('product'); // Default to product
	let itemName = $state('');
	let itemDescription = $state('');
	let primaryImage = $state<File | null>(null);
	let additionalImages = $state<File[]>([]);
	let customFields = $state<CustomField[]>([]);
	let isSubmitting = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');
	
	// References for animations
	let formElement: HTMLElement;
	let headerElement: HTMLElement;
	
	// Fetch user's custom fields on mount
	onMount(async () => {
		if ($isAuthenticated) {
			try {
				const response = await axios.get(`https://api.apexlinks.org/users/${$userStore.user?.id}/custom-fields`);
				if (response.data.fields) {
					customFields = response.data.fields.map((field: any) => ({
						...field,
						value: '' // Initialize with empty value
					}));
				}
			} catch (error) {
				console.error('Error fetching custom fields:', error);
			}
			
			// Initialize animations
			if (headerElement) slideInFromBottom(headerElement);
			if (formElement) slideInFromLeft(formElement);
			
			// Setup scroll-based animations
			createScrollObserver('.form-section', slideInFromBottom, { once: true });
		}
	});
	
	// Handle primary image selection
	function handlePrimaryImageChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			primaryImage = file;
		}
	}
	
	// Handle additional images selection
	function handleAdditionalImagesChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const files = target.files;
		if (files && files.length > 0) {
			additionalImages = [...additionalImages, ...Array.from(files)];
		}
	}
	
	// Remove an additional image
	function removeAdditionalImage(index: number): void {
		additionalImages = additionalImages.filter((_, i) => i !== index);
	}
	
	// Handle custom field value change
	function updateCustomFieldValue(index: number, value: string): void {
		customFields = customFields.map((field, i) => 
			i === index ? { ...field, value } : field
		);
	}
	
	// Handle form submission
	async function handleSubmit(): Promise<void> {
		if (!$isAuthenticated) {
			errorMessage = 'You must be logged in to add an item';
			return;
		}
		
		if (!itemName.trim()) {
			errorMessage = 'Item name is required';
			return;
		}
		
		if (!primaryImage) {
			errorMessage = 'Primary image is required';
			return;
		}
		
		try {
			isSubmitting = true;
			errorMessage = '';
			
			// Create form data for API submission
			const formData = new FormData();
			formData.append('userId', $userStore.user?.id || '');
			formData.append('type', itemType);
			formData.append('name', itemName);
			formData.append('description', itemDescription);
			formData.append('primaryImage', primaryImage);
			
			// Append additional images
			additionalImages.forEach((image, index) => {
				formData.append(`additionalImages[${index}]`, image);
			});
			
			// Append custom fields
			formData.append('customFields', JSON.stringify(
				customFields.map(field => ({
					id: field.id,
					label: field.label,
					type: field.type,
					value: field.value
				}))
			));
			
			// Submit to API
			const response = await axios.post('https://api.apexlinks.org/items', formData, {
				headers: {
					'Content-Type': 'multipart/form-data'
				}
			});
			
			if (response.data && response.data.id) {
				successMessage = 'Item added successfully!';
				setTimeout(() => {
					window.location.href = `/i?i=${response.data.id}`;
				}, 1500);
			}
		} catch (error) {
			console.error('Error adding item:', error);
			errorMessage = 'Failed to add item. Please try again later.';
		} finally {
			isSubmitting = false;
		}
	}
	
	// Redirect to login if not authenticated
	$effect(() => {
		if (!$isAuthenticated && typeof window !== 'undefined') {
			window.location.href = '/auth/login?redirect=/i/add';
		}
	});
</script>

<PageTransition>
	<div class="min-h-screen bg-[#151A33] text-[#FDFDFF] pb-20">
		<!-- Background elements -->
		<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		
		<div class="container mx-auto px-4 py-8">
			<header class="mb-12 text-center" bind:this={headerElement}>
				<h1 class="text-4xl font-bold text-[#FDFDFF] mb-4">Add New Item</h1>
				<p class="text-xl text-[#C9CCF5]">Showcase your product or service with detailed information and images</p>
			</header>
			
			<div class="max-w-3xl mx-auto">
				<form 
					bind:this={formElement} 
					on:submit|preventDefault={handleSubmit}
					class="glass p-8 rounded-2xl"
				>
					{#if errorMessage}
						<div class="bg-red-500/20 border border-red-500/50 text-[#FDFDFF] p-4 rounded-lg mb-6">
							{errorMessage}
						</div>
					{/if}
					
					{#if successMessage}
						<div class="bg-green-500/20 border border-green-500/50 text-[#FDFDFF] p-4 rounded-lg mb-6">
							{successMessage}
						</div>
					{/if}
					
					<!-- Item Type Selection -->
					<div class="form-section mb-8">
						<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Type of Item</h2>
						
						<div class="flex gap-4">
							<label class="flex-1">
								<input 
									type="radio" 
									name="itemType" 
									value="product" 
									bind:group={itemType}
									class="sr-only"
								/>
								<div class={`p-4 rounded-lg border-2 cursor-pointer flex items-center justify-center ${itemType === 'product' ? 'border-[#A3F1FF] bg-[#2A2E6E]/50' : 'border-[#B2B5E0]/30 bg-[#1C1F4A]/30'}`}>
									<span class="text-2xl mr-2">📦</span>
									<span class="font-medium">Product</span>
								</div>
							</label>
							
							<label class="flex-1">
								<input 
									type="radio" 
									name="itemType" 
									value="service" 
									bind:group={itemType}
									class="sr-only"
								/>
								<div class={`p-4 rounded-lg border-2 cursor-pointer flex items-center justify-center ${itemType === 'service' ? 'border-[#A3F1FF] bg-[#2A2E6E]/50' : 'border-[#B2B5E0]/30 bg-[#1C1F4A]/30'}`}>
									<span class="text-2xl mr-2">🔧</span>
									<span class="font-medium">Service</span>
								</div>
							</label>
						</div>
					</div>
					
					<!-- Basic Information -->
					<div class="form-section mb-8">
						<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Basic Information</h2>
						
						<div class="mb-4">
							<label for="itemName" class="block mb-2 text-[#C9CCF5]">Name <span class="text-[#A3F1FF]">*</span></label>
							<input 
								type="text" 
								id="itemName" 
								bind:value={itemName}
								required
								placeholder="Enter item name"
								class="w-full p-3 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
							/>
						</div>
						
						<div class="mb-4">
							<label for="itemDescription" class="block mb-2 text-[#C9CCF5]">Description</label>
							<textarea 
								id="itemDescription" 
								bind:value={itemDescription}
								rows="5"
								placeholder="Describe your item in detail"
								class="w-full p-3 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
							></textarea>
						</div>
					</div>
					
					<!-- Images -->
					<div class="form-section mb-8">
						<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Images</h2>
						
						<div class="mb-6">
							<label class="block mb-2 text-[#C9CCF5]">Primary Image <span class="text-[#A3F1FF]">*</span></label>
							
							{#if primaryImage}
								<div class="mb-4 relative">
									<img 
										src={URL.createObjectURL(primaryImage)} 
										alt="Primary Image Preview" 
										class="h-40 object-contain rounded-lg border border-[#B2B5E0]/30 p-2 bg-[#1C1F4A]/30"
									/>
									<button 
										type="button"
										class="absolute top-2 right-2 bg-red-500/80 text-white w-8 h-8 rounded-full flex items-center justify-center"
										on:click={() => primaryImage = null}
									>
										✕
									</button>
								</div>
							{:else}
								<div class="border-2 border-dashed border-[#B2B5E0]/30 rounded-lg p-8 text-center mb-4">
									<label for="primaryImage" class="cursor-pointer block">
										<span class="text-4xl block mb-2">📷</span>
										<span class="text-[#C9CCF5]">Click to upload primary image</span>
										<input 
											type="file"
											id="primaryImage"
											accept="image/*"
											on:change={handlePrimaryImageChange}
											class="hidden"
										/>
									</label>
								</div>
							{/if}
						</div>
						
						<div>
							<label class="block mb-2 text-[#C9CCF5]">Additional Images</label>
							
							<div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
								{#each additionalImages as image, index}
									<div class="relative">
										<img 
											src={URL.createObjectURL(image)} 
											alt={`Additional Image ${index+1}`} 
											class="h-24 w-full object-cover rounded-lg border border-[#B2B5E0]/30"
										/>
										<button 
											type="button"
											class="absolute top-1 right-1 bg-red-500/80 text-white w-6 h-6 rounded-full flex items-center justify-center text-sm"
											on:click={() => removeAdditionalImage(index)}
										>
											✕
										</button>
									</div>
								{/each}
								
								<label for="additionalImages" class="cursor-pointer border-2 border-dashed border-[#B2B5E0]/30 rounded-lg p-4 flex flex-col items-center justify-center">
									<span class="text-2xl block mb-1">+</span>
									<span class="text-sm text-[#C9CCF5]">Add More</span>
									<input 
										type="file"
										id="additionalImages"
										accept="image/*"
										multiple
										on:change={handleAdditionalImagesChange}
										class="hidden"
									/>
								</label>
							</div>
						</div>
					</div>
					
					<!-- Custom Fields -->
					{#if customFields.length > 0}
						<div class="form-section mb-8">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Additional Details</h2>
							
							<div class="space-y-4">
								{#each customFields as field, index}
									<div class="mb-4">
										<label for={`field-${index}`} class="block mb-2 text-[#C9CCF5]">{field.label}</label>
										
										{#if field.type === 'number'}
											<input 
												type="number" 
												id={`field-${index}`} 
												value={field.value}
												on:input={(e) => updateCustomFieldValue(index, (e.target as HTMLInputElement).value)}
												placeholder={`Enter ${field.label.toLowerCase()}`}
												class="w-full p-3 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
											/>
										{:else}
											<input 
												type="text" 
												id={`field-${index}`} 
												value={field.value}
												on:input={(e) => updateCustomFieldValue(index, (e.target as HTMLInputElement).value)}
												placeholder={`Enter ${field.label.toLowerCase()}`}
												class="w-full p-3 rounded-lg bg-[#2A2E6E]/30 border border-[#B2B5E0]/30 text-[#FDFDFF] focus:border-[#A3F1FF] focus:ring-1 focus:ring-[#A3F1FF]"
											/>
										{/if}
									</div>
								{/each}
							</div>
						</div>
					{/if}
					
					<!-- Submit Button -->
					<div class="flex justify-center mt-8">
						<button 
							type="submit"
							disabled={isSubmitting}
							class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium relative overflow-hidden"
						>
							{#if isSubmitting}
								<span class="flex items-center">
									<span class="animate-spin mr-2">⟳</span> 
									Processing...
								</span>
							{:else}
								Add Item
							{/if}
						</button>
					</div>
				</form>
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
	
	:global(.glass-button:disabled) {
		opacity: 0.7;
		cursor: not-allowed;
	}
</style> 