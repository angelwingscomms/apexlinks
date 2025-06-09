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
	<div class="relative min-h-screen bg-[#f5f5f7]">
		<!-- Light subtle background pattern -->
		<div class="absolute inset-0 opacity-5" style="background-image: url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHZpZXdCb3g9IjAgMCA2MCA2MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBmaWxsPSJub25lIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiPjxnIGZpbGw9IiMwMDAiIGZpbGwtb3BhY2l0eT0iLjAyIj48cGF0aCBkPSJNMzYgMzRjMC0yLjIgMS44LTQgNC00czQgMS44IDQgNC0xLjggNC00IDQtNC0xLjgtNC00bTAgLTI0YzAtMi4yIDEuOC00IDQtNHM0IDEuOCA0IDQtMS44IDQtNCA0LTQtMS44LTQtNG0tMjQgMGMwLTIuMiAxLjgtNCA0LTRzNCAxLjggNCA0LTEuOCA0LTQgNC00LTEuOC00LTRtMCAyNGMwLTIuMiAxLjgtNCA0LTRzNCAxLjggNCA0LTEuOCA0LTQgNC00LTEuOC00LTQiLz48L2c+PC9nPjwvc3ZnPg==');"></div>
		
		<header class="container mx-auto px-4 pt-28 pb-20">
			<div class="hero-content max-w-5xl mx-auto text-center">
				<h1 
					class="text-5xl md:text-7xl font-bold text-[#1d1d1f] mb-6 tracking-tight"
					bind:this={heroHeading}
				>
					Showcase Your <span class="text-gradient">Products</span> <br>Like Never Before
				</h1>
				
				<p class="text-xl md:text-2xl text-[#86868b] max-w-3xl mx-auto mb-12 leading-relaxed">
					Create stunning product catalogs with AI-powered search, custom theming, and intelligent customer assistance. Designed for the modern business.
				</p>
				
				<div class="flex flex-col sm:flex-row justify-center items-center gap-4 mb-20">
					<a 
						href="/i/add" 
						class="btn-primary px-8 py-4 text-xl inline-flex items-center gap-2"
						bind:this={ctaButton}
					>
						Start Your Catalog
						<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"></path><path d="m12 5 7 7-7 7"></path></svg>
					</a>
					
					{#if !$isAuthenticated}
						<button 
							on:click={handleLogin}
							class="btn-secondary px-8 py-4 text-xl"
						>
							Login with Google
						</button>
					{/if}
				</div>
				
				<!-- Product showcase mockup -->
				<div class="relative">
					<div class="neumorphic overflow-hidden rounded-2xl">
						<img 
							src="https://placehold.co/1600x900/f5f5f7/1d1d1f.png?text=ApexLinks+Catalog+Preview" 
							alt="Create a product catalog showcase on a sleek device mockup, featuring a minimalist user interface with product cards and AI search functionality"
							class="w-full h-auto rounded-2xl"
						/>
					</div>
					<!-- Floating elements -->
					<div class="absolute -top-10 -right-10 w-32 h-32 glass-sm rounded-2xl flex items-center justify-center animate-[float_4s_ease-in-out_infinite]" style="animation-delay: 0.5s">
						<span class="text-3xl">🔍</span>
					</div>
					<div class="absolute -bottom-5 -left-5 w-24 h-24 glass-sm rounded-2xl flex items-center justify-center animate-[float_4s_ease-in-out_infinite]">
						<span class="text-3xl">💫</span>
					</div>
				</div>
			</div>
		</header>
		
		<main>
			<!-- Key Features Section -->
			<section class="py-24 bg-white" bind:this={featureSection}>
				<div class="container mx-auto px-4">
					<h2 class="section-title text-center text-[#1d1d1f] mb-16">
						Powerful Features for Your Business
					</h2>
					
					<div class="grid grid-cols-1 md:grid-cols-3 gap-12" bind:this={featureCards}>
						<div class="neumorphic p-8 text-center feature-card">
							<div class="feature-icon mx-auto">
								<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></svg>
							</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">AI-Powered Search</h3>
							<p class="text-[#86868b]">Smart search capabilities that understand natural language and customer intent</p>
						</div>
						
						<div class="neumorphic p-8 text-center feature-card">
							<div class="feature-icon mx-auto">
								<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"></path><path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"></path><path d="M12 2v2"></path><path d="M12 22v-2"></path><path d="m17 20.66-1-1.73"></path><path d="M11 10.27 7 3.34"></path><path d="m20.66 17-1.73-1"></path><path d="m3.34 7 1.73 1"></path><path d="M14 12h8"></path><path d="M2 12h2"></path><path d="m20.66 7-1.73 1"></path><path d="m3.34 17 1.73-1"></path><path d="m17 3.34-1 1.73"></path><path d="m7 20.66 1-1.73"></path></svg>
							</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Custom Theming</h3>
							<p class="text-[#86868b]">Match your brand with customizable colors, fonts and styles that represent your identity</p>
						</div>
						
						<div class="neumorphic p-8 text-center feature-card">
							<div class="feature-icon mx-auto">
								<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"></path><rect width="16" height="12" x="4" y="8" rx="2"></rect><path d="M2 14h2"></path><path d="M20 14h2"></path><path d="M15 13v2"></path><path d="M9 13v2"></path></svg>
							</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">AI Assistance</h3>
							<p class="text-[#86868b]">Conversational AI that answers questions and provides smart product recommendations</p>
						</div>
					</div>
				</div>
			</section>
			
			<!-- Benefits Section -->
			<section class="py-24 bg-[#f5f5f7]" bind:this={benefitSection}>
				<div class="container mx-auto px-4">
					<h2 class="section-title text-center text-[#1d1d1f] mb-16">
						Elevate Your Business
					</h2>
					
					<div class="grid grid-cols-1 md:grid-cols-2 gap-12">
						<div class="neumorphic p-10 benefit-item">
							<h3 class="text-2xl font-semibold text-[#1d1d1f] mb-6">Powerful Cataloging</h3>
							<ul class="space-y-5">
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Organize products with custom fields and categories</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Upload multiple high-quality images per item</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Add detailed descriptions and specifications</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Set custom filters that make sense for your business</span>
								</li>
							</ul>
						</div>
						
						<div class="neumorphic p-10 benefit-item">
							<h3 class="text-2xl font-semibold text-[#1d1d1f] mb-6">Enhanced Customer Experience</h3>
							<ul class="space-y-5">
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">AI-powered product recommendations</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Similarity search to find related products</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Conversational AI to answer customer questions</span>
								</li>
								<li class="flex items-start">
									<span class="text-[#0a84ff] mr-3 mt-1">
										<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"></path></svg>
									</span>
									<span class="text-[#1d1d1f]">Beautiful, responsive catalog across all devices</span>
								</li>
							</ul>
						</div>
					</div>
				</div>
			</section>
			
			<!-- How It Works Section -->
			<section class="py-24 bg-white">
				<div class="container mx-auto px-4">
					<h2 class="section-title text-center text-[#1d1d1f] mb-16">
						Simple to Use
					</h2>
					
					<div class="flex flex-col md:flex-row justify-between items-stretch gap-8">
						<div class="neumorphic p-8 w-full md:w-1/3 text-center example-card flex flex-col">
							<div class="w-16 h-16 mx-auto rounded-full bg-[#0a84ff] text-white flex items-center justify-center text-xl font-medium mb-6">1</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Create Your Account</h3>
							<p class="text-[#86868b] flex-grow">Sign up and set up your business profile with your logo and brand colors</p>
							<img src="https://placehold.co/400x250/e8e8ed/86868b.png?text=Account+Setup" alt="Account setup illustration showing a simple profile creation interface" class="mt-6 rounded-xl w-full" />
						</div>
						
						<div class="neumorphic p-8 w-full md:w-1/3 text-center example-card flex flex-col">
							<div class="w-16 h-16 mx-auto rounded-full bg-[#0a84ff] text-white flex items-center justify-center text-xl font-medium mb-6">2</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Add Your Products</h3>
							<p class="text-[#86868b] flex-grow">Upload product details, images, and customize your catalog structure</p>
							<img src="https://placehold.co/400x250/e8e8ed/86868b.png?text=Product+Management" alt="Product management interface showing a clean dashboard for adding and organizing products" class="mt-6 rounded-xl w-full" />
						</div>
						
						<div class="neumorphic p-8 w-full md:w-1/3 text-center example-card flex flex-col">
							<div class="w-16 h-16 mx-auto rounded-full bg-[#0a84ff] text-white flex items-center justify-center text-xl font-medium mb-6">3</div>
							<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Share With Customers</h3>
							<p class="text-[#86868b] flex-grow">Share your catalog link and let customers explore your offerings</p>
							<img src="https://placehold.co/400x250/e8e8ed/86868b.png?text=Customer+View" alt="Customer view showing an elegant product catalog with search and filter options" class="mt-6 rounded-xl w-full" />
						</div>
					</div>
				</div>
			</section>
			
			<!-- Testimonial Section -->
			<section class="py-24 bg-[#f5f5f7]">
				<div class="container mx-auto px-4">
					<h2 class="section-title text-center text-[#1d1d1f] mb-16">
						What Our Users Say
					</h2>
					
					<div class="neumorphic p-10 testimonial max-w-4xl mx-auto">
						<div class="flex flex-col md:flex-row items-center md:items-start gap-8">
							<div class="w-24 h-24 rounded-full bg-gradient-to-br from-[#0a84ff] to-[#54c7fc] flex-shrink-0 flex items-center justify-center text-white text-4xl font-medium">
								S
							</div>
							<div>
								<svg class="h-8 w-8 text-[#0a84ff] mb-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
									<path d="M11.605 4.5C11.605 3.867 11.105 3.5 10.605 3.5C10.105 3.5 9.605 3.867 9.605 4.5H11.605ZM9.605 4.5V17.5H11.605V4.5H9.605Z" fill="currentColor"/>
									<path d="M3.60498 17.5C3.60498 16.867 3.10498 16.5 2.60498 16.5C2.10498 16.5 1.60498 16.867 1.60498 17.5H3.60498ZM1.60498 17.5V20.5H3.60498V17.5H1.60498Z" fill="currentColor"/>
									<path d="M16.605 15.5C16.605 16.133 17.105 16.5 17.605 16.5C18.105 16.5 18.605 16.133 18.605 15.5L16.605 15.5ZM18.605 15.5L18.605 7.5L16.605 7.5L16.605 15.5L18.605 15.5Z" fill="currentColor"/>
									<path d="M22.605 7.5C22.605 8.133 23.105 8.5 23.605 8.5C24.105 8.5 24.605 8.133 24.605 7.5L22.605 7.5ZM24.605 7.5L24.605 4.5L22.605 4.5L22.605 7.5L24.605 7.5Z" fill="currentColor"/>
								</svg>
								<p class="text-xl leading-relaxed mb-6 text-[#1d1d1f]">"ApexLinks transformed how we showcase our handcrafted jewelry. The AI-powered search and beautiful interface have increased our online engagement by 70%. Customers love how easy it is to find exactly what they're looking for."</p>
								<p class="font-semibold text-[#1d1d1f]">Sarah Johnson</p>
								<p class="text-[#86868b]">Founder, Artisan Gems</p>
							</div>
						</div>
					</div>
				</div>
			</section>
			
			<!-- CTA Section -->
			<section class="py-20 bg-gradient-to-br from-[#0a84ff] to-[#54c7fc] text-white">
				<div class="container mx-auto px-4 text-center">
					<h2 class="text-4xl md:text-5xl font-bold mb-6 tracking-tight">
						Ready to Elevate Your Product Catalog?
					</h2>
					<p class="text-xl md:text-2xl max-w-3xl mx-auto mb-10 opacity-90">
						Join thousands of businesses that use ApexLinks to create stunning product showcases
					</p>
					
					{#if !$isAuthenticated}
						<div class="flex flex-col sm:flex-row justify-center gap-4">
							<button 
								on:click={handleLogin}
								class="bg-white text-[#0a84ff] px-8 py-4 rounded-xl text-lg font-medium hover:bg-opacity-90 transition-all"
							>
								Sign Up Free
							</button>
							<a 
								href="/about" 
								class="bg-transparent border-2 border-white px-8 py-4 rounded-xl text-lg font-medium hover:bg-white hover:bg-opacity-10 transition-all"
							>
								Learn More
							</a>
						</div>
					{:else}
						<div class="flex flex-col sm:flex-row justify-center gap-4">
							<a 
								href="/i/add" 
								class="bg-white text-[#0a84ff] px-8 py-4 rounded-xl text-lg font-medium hover:bg-opacity-90 transition-all"
							>
								Create New Catalog
							</a>
							<a 
								href="/profile" 
								class="bg-transparent border-2 border-white px-8 py-4 rounded-xl text-lg font-medium hover:bg-white hover:bg-opacity-10 transition-all"
							>
								View Your Profile
							</a>
						</div>
					{/if}
				</div>
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