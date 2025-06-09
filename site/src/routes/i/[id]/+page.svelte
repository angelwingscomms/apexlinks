<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { slideInFromBottom, slideInFromLeft, slideInFromRight } from '$lib/animations.js';
	import axios from 'axios';
	
	// Types
	interface CustomField {
		id: string;
		label: string;
		type: 'text' | 'number';
		value: string | number;
	}
	
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
		customFields: CustomField[];
		createdAt: string;
	}
	
	// State
	let item = $state<Item | null>(null);
	let loading = $state(true);
	let error = $state('');
	let selectedImage = $state('');
	let similarItems = $state<Item[]>([]);
	
	// Animation references
	let itemHeader: HTMLElement;
	let itemDetails: HTMLElement;
	let itemGallery: HTMLElement;
	let similarSection: HTMLElement;
	
	// Handle not found - redirect to search page
	function handleNotFound() {
		window.location.href = '/i';
	}
	
	// Get item from the URL
	onMount(async () => {
		try {
			const url = new URL(window.location.href);
			const itemId = url.searchParams.get('i');
			
			if (!itemId) {
				error = 'No item ID provided';
				handleNotFound();
				return;
			}
			
			// Fetch item data
			const response = await axios.get(`https://api.apexlinks.org/items/${itemId}`);
			item = response.data.item;
			
			if (!item) {
				error = 'Item not found';
				handleNotFound();
				return;
			}
			
			// Set the selected image to the primary image
			selectedImage = item.primaryImage;
			
			// Fetch similar items
			const similarResponse = await axios.get(`https://api.apexlinks.org/items/${itemId}/similar`);
			similarItems = similarResponse.data.items || [];
			
			// Apply animations
			if (itemHeader) slideInFromBottom(itemHeader);
			if (itemDetails) slideInFromLeft(itemDetails);
			if (itemGallery) slideInFromRight(itemGallery);
			
			// Set up intersection observer for similar items section
			if (similarSection) {
				const observer = new IntersectionObserver(
					(entries) => {
						if (entries[0].isIntersecting) {
							slideInFromBottom(similarSection);
							observer.disconnect();
						}
					},
					{ threshold: 0.1 }
				);
				observer.observe(similarSection);
			}
		} catch (err) {
			console.error('Error fetching item:', err);
			error = 'Failed to load item data. Please try again later.';
		} finally {
			loading = false;
		}
	});
	
	// Handle image selection
	function selectImage(imageUrl: string) {
		selectedImage = imageUrl;
	}
	
	// Contact owner
	function contactOwner() {
		if (!$isAuthenticated) {
			window.location.href = '/auth/login?redirect=' + encodeURIComponent(window.location.pathname + window.location.search);
			return;
		}
		
		// Redirect to chat with the item owner
		window.location.href = `/chat?userId=${item?.userId}`;
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
	
	// Check if the current user is the owner
	$effect(() => {
		if (item && $isAuthenticated && $userStore.user?.id === item.userId) {
			// User is the owner
		}
	});
</script>

<PageTransition>
	<div class="min-h-screen bg-[#151A33] text-[#FDFDFF] pb-16">
		<!-- Background elements -->
		<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		
		<div class="container mx-auto px-4 py-8">
			{#if loading}
				<div class="flex justify-center items-center py-20">
					<div class="animate-spin mr-3">⟳</div>
					<span>Loading item details...</span>
				</div>
			{:else if error}
				<div class="glass p-8 text-center">
					<p class="text-xl text-red-400 mb-4">{error}</p>
					<button 
						on:click={() => window.location.href = '/i'}
						class="glass-button px-4 py-2 text-[#FDFDFF]"
					>
						Browse Items
					</button>
				</div>
			{:else if item}
				<!-- Item Header -->
				<header class="mb-10" bind:this={itemHeader}>
					<div class="flex justify-between items-start">
						<div>
							<h1 class="text-4xl font-bold text-[#FDFDFF] mb-2">{item.name}</h1>
							<div class="flex items-center gap-4">
								<span class={`text-sm px-3 py-1 rounded-full ${item.type === 'product' ? 'bg-[#A3F1FF]/20 text-[#A3F1FF]' : 'bg-[#E5D9FF]/20 text-[#E5D9FF]'}`}>
									{item.type === 'product' ? 'Product' : 'Service'}
								</span>
								<span class="text-[#C9CCF5] text-sm">
									Listed {formatDate(item.createdAt)}
								</span>
							</div>
						</div>
						
						{#if $isAuthenticated && $userStore.user?.id === item.userId}
							<a 
								href={`/i/edit?i=${item.id}`}
								class="glass-button px-4 py-2 text-[#FDFDFF]"
							>
								Edit Item
							</a>
						{/if}
					</div>
				</header>
				
				<div class="grid grid-cols-1 lg:grid-cols-2 gap-10 mb-16">
					<!-- Image Gallery -->
					<div bind:this={itemGallery}>
						<div class="glass p-4 rounded-2xl">
							<div class="mb-4 h-80 overflow-hidden rounded-lg">
								<img 
									src={selectedImage} 
									alt={item.name} 
									class="w-full h-full object-contain"
								/>
							</div>
							
							{#if item.additionalImages.length > 0 || item.primaryImage}
								<div class="grid grid-cols-5 gap-2">
									<button 
										class={`thumbnail-btn rounded-lg overflow-hidden border-2 ${selectedImage === item.primaryImage ? 'border-[#A3F1FF]' : 'border-transparent'}`}
										on:click={() => selectImage(item.primaryImage)}
									>
										<img 
											src={item.primaryImage} 
											alt="Primary" 
											class="w-full h-16 object-cover"
										/>
									</button>
									
									{#each item.additionalImages as imgUrl, index}
										<button 
											class={`thumbnail-btn rounded-lg overflow-hidden border-2 ${selectedImage === imgUrl ? 'border-[#A3F1FF]' : 'border-transparent'}`}
											on:click={() => selectImage(imgUrl)}
										>
											<img 
												src={imgUrl} 
												alt={`Image ${index + 1}`} 
												class="w-full h-16 object-cover"
											/>
										</button>
									{/each}
								</div>
							{/if}
						</div>
					</div>
					
					<!-- Item Details -->
					<div bind:this={itemDetails}>
						<div class="glass p-6 rounded-2xl mb-6">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Description</h2>
							<p class="text-[#C9CCF5] whitespace-pre-line mb-4">
								{item.description || 'No description provided.'}
							</p>
							
							{#if item.customFields && item.customFields.length > 0}
								<div class="mt-8">
									<h3 class="text-xl font-semibold text-[#F4E6FF] mb-3">Specifications</h3>
									<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
										{#each item.customFields as field}
											<div class="flex justify-between p-3 border-b border-[#B2B5E0]/20">
												<span class="text-[#B2B5E0]">{field.label}</span>
												<span class="text-[#F4E6FF] font-medium">
													{field.type === 'number' && field.label.toLowerCase().includes('price') 
														? `$${field.value}` 
														: field.value}
												</span>
											</div>
										{/each}
									</div>
								</div>
							{/if}
						</div>
						
						<div class="glass p-6 rounded-2xl">
							<h2 class="text-2xl font-semibold text-[#F4E6FF] mb-4">Seller Information</h2>
							<div class="flex items-center gap-4 mb-6">
								<img 
									src={item.userImage} 
									alt={item.userName} 
									class="w-16 h-16 rounded-full border-2 border-[#A3F1FF]/20"
								/>
								<div>
									<h3 class="text-xl font-medium text-[#F4E6FF]">{item.userName}</h3>
									<a 
										href={`/u?i=${item.userId}`}
										class="text-[#A3F1FF] hover:underline"
									>
										View Profile
									</a>
								</div>
							</div>
							
							{#if $userStore.user?.id !== item?.userId}
								<button 
									on:click={contactOwner}
									class="glass-button w-full py-3 text-[#FDFDFF] font-medium"
								>
									Contact Seller
								</button>
							{/if}
						</div>
					</div>
				</div>
				
				<!-- Similar Items Section -->
				{#if similarItems.length > 0}
					<section class="mt-16" bind:this={similarSection}>
						<h2 class="text-3xl font-semibold text-[#FDFDFF] mb-8">Similar Items</h2>
						
						<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
							{#each similarItems.slice(0, 3) as similarItem}
								<a 
									href={`/i?i=${similarItem.id}`}
									class="glass hover:scale-105 transition-transform duration-300 overflow-hidden rounded-2xl flex flex-col"
								>
									<div class="h-48 overflow-hidden">
										<img 
											src={similarItem.primaryImage} 
											alt={similarItem.name} 
											class="w-full h-full object-cover"
										/>
									</div>
									<div class="p-5">
										<div class="flex justify-between items-start mb-2">
											<h3 class="text-xl font-semibold text-[#F4E6FF] truncate">
												{similarItem.name}
											</h3>
											<span class={`text-xs px-2 py-1 rounded-full ${similarItem.type === 'product' ? 'bg-[#A3F1FF]/20 text-[#A3F1FF]' : 'bg-[#E5D9FF]/20 text-[#E5D9FF]'}`}>
												{similarItem.type}
											</span>
										</div>
										<p class="text-[#C9CCF5] text-sm mb-4">
											{similarItem.description.length > 100 
												? similarItem.description.substring(0, 100) + '...' 
												: similarItem.description}
										</p>
									</div>
								</a>
							{/each}
						</div>
					</section>
				{/if}
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
	
	.thumbnail-btn {
		transition: all 0.2s ease;
	}
	
	.thumbnail-btn:hover {
		transform: scale(1.05);
	}
</style> 