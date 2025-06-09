<script lang="ts">
	import { isAuthenticated, userStore } from '$lib/stores/userStore';
	import { onMount } from 'svelte';
	import AnimatedCard from '$lib/components/AnimatedCard.svelte';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { 
		dreamyBackgroundFlow, 
		pulsateEffect, 
		textReveal, 
		slideInFromLeft, 
		slideInFromRight, 
		slideInFromBottom,
		createScrollObserver,
		staggeredElementsOnScroll,
		glowEffect
	} from '$lib/animations.js';
	
	let searchQuery = $state('');
	
	let heroHeading: any;
	let ctaButton: any;
	let backgroundElement: any;
	let featureSection: any;
	let featureCards: any;
	let benefitSection: any;
	
	function handleTextSearch() {
		if (searchQuery.trim()) {
			window.location.href = `/i?q=${encodeURIComponent(searchQuery.trim())}`;
		}
	}
	
	function handleLogin() {
		userStore.startGoogleAuth();
	}
	
	onMount(() => {
		if (heroHeading) textReveal(heroHeading);
		if (ctaButton) pulsateEffect(ctaButton);
		if (backgroundElement) dreamyBackgroundFlow(backgroundElement);
		
		// Apply slide-in animations
		const heroContent = document.querySelector('.hero-content');
		if (heroContent) slideInFromBottom(heroContent, 300);
		
		// Create scroll-based animations
		createScrollObserver('.feature-card', slideInFromBottom, { once: true, staggerDelay: 150 });
		createScrollObserver('.benefit-item', slideInFromLeft, { once: true });
		createScrollObserver('.example-card', slideInFromRight, { once: true });
		createScrollObserver('.testimonial', slideInFromBottom, { once: true });
		
		// Apply glow effect to accent elements
		document.querySelectorAll('.accent-glow').forEach(el => glowEffect(el));
		
		// Staggered animations for feature cards when they come into view
		if (featureCards) {
			const observer = new IntersectionObserver((entries) => {
				if (entries[0].isIntersecting) {
					staggeredElementsOnScroll(featureCards.querySelectorAll('.feature-card'));
					observer.disconnect();
				}
			});
			observer.observe(featureSection);
		}
	});
</script>

<PageTransition>
	<div class="relative min-h-screen bg-[#151A33] text-[#FDFDFF]">
		<!-- Dreamy animated background -->
		<div 
			class="absolute inset-0 -z-10 bg-gradient-to-br from-[#1C1F4A] to-[#2A2E6E]" 
			bind:this={backgroundElement}
		></div>
		
		<!-- Animated background elements -->
		<div class="fixed -z-5 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#E5D9FF]/10 blur-3xl"></div>
		<div class="fixed -z-5 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		<div class="fixed -z-5 top-1/2 right-1/3 w-48 h-48 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		
		<header class="container mx-auto px-4 py-16 text-center">
			<div class="hero-content">
				<h1 
					class="text-4xl md:text-6xl font-bold text-[#FDFDFF] mb-6"
					bind:this={heroHeading}
				>
					Showcase Your Products on <span class="text-[#A3F1FF] accent-glow">ApexLinks</span>
				</h1>
				
				<p class="text-xl text-[#C9CCF5] max-w-2xl mx-auto mb-12">
					Create stunning product catalogs with AI-powered search, custom theming, and intelligent customer assistance. Let your products and services shine with our premium catalog platform.
				</p>
				
				<div class="flex justify-center">
					<a 
						href="/i/add" 
						class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium"
						bind:this={ctaButton}
					>
						Start Your Catalog
					</a>
				</div>
			</div>
		</header>
		
		<main class="container mx-auto px-4 py-16">
			<!-- Key Features Section -->
			<section class="mb-20" bind:this={featureSection}>
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-8 text-center">Why Choose ApexLinks?</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-3 gap-8" bind:this={featureCards}>
					<div class="glass p-8 text-center feature-card">
						<div class="text-4xl mb-4 text-[#A3F1FF]">✨</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">AI-Powered Search</h3>
						<p class="text-[#C9CCF5]">Let customers find exactly what they need with intelligent search capabilities that understand natural language</p>
					</div>
					
					<div class="glass p-8 text-center feature-card">
						<div class="text-4xl mb-4 text-[#A3F1FF]">🎨</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">Custom Theming</h3>
						<p class="text-[#C9CCF5]">Match your brand with customizable colors, fonts and styles that perfectly represent your business identity</p>
					</div>
					
					<div class="glass p-8 text-center feature-card">
						<div class="text-4xl mb-4 text-[#A3F1FF]">🤖</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">AI Assistance</h3>
						<p class="text-[#C9CCF5]">Help customers find the right products with conversational AI that answers questions and provides recommendations</p>
					</div>
				</div>
			</section>
			
			<!-- Benefits Section -->
			<section class="mb-20" bind:this={benefitSection}>
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-12 text-center">Elevate Your Business</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-12">
					<div class="glass p-8 benefit-item">
						<h3 class="text-2xl font-bold text-[#F4E6FF] mb-4">Powerful Cataloging</h3>
						<ul class="space-y-3">
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Organize products with custom fields and categories</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Upload multiple high-quality images per item</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Add detailed descriptions and specifications</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Set custom filters that make sense for your business</span>
							</li>
						</ul>
					</div>
					
					<div class="glass p-8 benefit-item">
						<h3 class="text-2xl font-bold text-[#F4E6FF] mb-4">Enhanced Customer Experience</h3>
						<ul class="space-y-3">
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>AI-powered product recommendations</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Similarity search to find related products</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Conversational AI to answer customer questions</span>
							</li>
							<li class="flex items-start">
								<span class="text-[#A3F1FF] mr-2">✓</span>
								<span>Beautiful, responsive catalog across all devices</span>
							</li>
						</ul>
					</div>
				</div>
			</section>
			
			<!-- How It Works Section -->
			<section class="mb-20">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-8 text-center">How It Works</h2>
				
				<div class="flex flex-col md:flex-row justify-center items-center md:space-x-8 space-y-8 md:space-y-0">
					<div class="glass p-6 w-full md:w-1/3 text-center example-card">
						<div class="text-3xl mb-4 text-[#A3F1FF]">1</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">Create Your Account</h3>
						<p class="text-[#C9CCF5]">Sign up and set up your business profile with your logo and brand colors</p>
					</div>
					
					<div class="glass p-6 w-full md:w-1/3 text-center example-card">
						<div class="text-3xl mb-4 text-[#A3F1FF]">2</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">Add Your Products</h3>
						<p class="text-[#C9CCF5]">Upload product details, images, and customize your catalog structure</p>
					</div>
					
					<div class="glass p-6 w-full md:w-1/3 text-center example-card">
						<div class="text-3xl mb-4 text-[#A3F1FF]">3</div>
						<h3 class="text-xl font-semibold mb-2 text-[#F4E6FF]">Share With Customers</h3>
						<p class="text-[#C9CCF5]">Share your catalog link and let customers explore your offerings</p>
					</div>
				</div>
			</section>
			
			<!-- Testimonial Section -->
			<section class="mb-16">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-8 text-center">What Our Users Say</h2>
				
				<div class="glass p-8 testimonial">
					<div class="flex flex-col md:flex-row items-center md:items-start gap-6">
						<div class="w-24 h-24 rounded-full glass-sm flex-shrink-0 flex items-center justify-center">
							<span class="text-4xl">👩‍💼</span>
						</div>
						<div>
							<p class="text-lg italic mb-4 text-[#C9CCF5]">"ApexLinks transformed how we showcase our handcrafted jewelry. The AI-powered search and beautiful interface have increased our online engagement by 70%. Customers love how easy it is to find exactly what they're looking for."</p>
							<p class="font-bold text-[#F4E6FF]">Sarah Johnson</p>
							<p class="text-sm text-[#B2B5E0]">Founder, Artisan Gems</p>
						</div>
					</div>
				</div>
			</section>
			
			<!-- CTA Section -->
			<section class="text-center py-12">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-6">Ready to Elevate Your Product Catalog?</h2>
				<p class="text-xl text-[#C9CCF5] max-w-2xl mx-auto mb-8">
					Join thousands of businesses that use ApexLinks to create stunning product showcases
				</p>
				
				{#if !$isAuthenticated}
					<div class="flex flex-col md:flex-row justify-center gap-4">
						<button 
							on:click={handleLogin}
							class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium accent-glow"
						>
							Sign Up Free
						</button>
						<a 
							href="/about" 
							class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium"
						>
							Learn More
						</a>
					</div>
				{:else}
					<div class="flex flex-col md:flex-row justify-center gap-4">
						<a 
							href="/i/add"
							class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium accent-glow"
						>
							Add Your First Item
						</a>
						<a 
							href="/profile" 
							class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium"
						>
							View Your Profile
						</a>
					</div>
				{/if}
			</section>
		</main>
	</div>
</PageTransition>

<style>
	@theme {
		--glass-opacity: 0.15;
		--glass-blur: 15px;
		--glass-border: 1px solid rgba(255, 255, 255, 0.2);
		--glass-shadow: 0 8px 32px rgba(163, 241, 255, 0.2);
	}
	
	@layer components {
		.glass-button {
			background: rgba(42, 46, 110, 0.25);
			backdrop-filter: blur(15px);
			border: 1px solid rgba(163, 241, 255, 0.3);
			box-shadow: 0 8px 16px rgba(42, 46, 110, 0.15);
			border-radius: 1rem;
			transition: all 0.3s ease;
		}
		
		.glass-button:hover {
			background: rgba(42, 46, 110, 0.35);
			box-shadow: 0 8px 25px rgba(163, 241, 255, 0.25);
			transform: translateY(-2px);
		}
	}
	
	.accent-glow {
		position: relative;
	}
	
	.accent-glow::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		border-radius: inherit;
		box-shadow: 0 0 15px rgba(163, 241, 255, 0.5);
		opacity: 0.5;
		z-index: -1;
	}
</style> 