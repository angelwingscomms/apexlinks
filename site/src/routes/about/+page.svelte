<script lang="ts">
	import { onMount } from 'svelte';
	import PageTransition from '$lib/components/PageTransition.svelte';
	import { 
		slideInFromBottom, 
		slideInFromLeft, 
		slideInFromRight, 
		createScrollObserver 
	} from '$lib/animations.js';
	
	// Animation references
	let headerSection: HTMLElement;
	let missionSection: HTMLElement;
	
	onMount(() => {
		// Initialize animations
		if (headerSection) slideInFromBottom(headerSection);
		if (missionSection) slideInFromLeft(missionSection);
		
		// Setup scroll-based animations
		createScrollObserver('.feature-item', slideInFromBottom, { once: true, staggerDelay: 150 });
		
		// Add parallax effect to floating elements
		const handleMouseMove = (e: MouseEvent) => {
			const floatingElements = document.querySelectorAll('.parallax-element');
			floatingElements.forEach((el: Element) => {
				const speed = parseFloat(el.getAttribute('data-speed') || '0.05');
				const x = (window.innerWidth - e.pageX * speed) / 100;
				const y = (window.innerHeight - e.pageY * speed) / 100;
				
				if (el instanceof HTMLElement) {
					el.style.transform = `translateX(${x}px) translateY(${y}px)`;
				}
			});
		};
		
		document.addEventListener('mousemove', handleMouseMove);
		
		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
		};
	});
</script>

<PageTransition>
	<div class="min-h-screen bg-gradient-soft">
		<!-- Subtle background pattern -->
		<div class="absolute inset-0 bg-subtle-dots"></div>
		
		<!-- Floating background elements -->
		<div class="fixed w-64 h-64 rounded-full bg-[#0a84ff]/5 blur-3xl top-20 -left-20 float-element parallax-element" data-speed="0.02"></div>
		<div class="fixed w-80 h-80 rounded-full bg-[#54c7fc]/5 blur-3xl bottom-40 -right-20 float-element-delayed parallax-element" data-speed="0.03"></div>
		<div class="fixed w-40 h-40 rounded-full bg-[#30c8ff]/5 blur-3xl top-1/3 right-1/4 float-element parallax-element" data-speed="0.05"></div>
		
		<!-- Hero Section -->
		<header class="relative py-28" bind:this={headerSection}>
			<div class="container mx-auto px-4 text-center">
				<h1 class="text-5xl md:text-6xl font-bold mb-6 text-[#1d1d1f] tracking-tight">
					About <span class="text-gradient">ApexLinks</span>
				</h1>
				<p class="text-xl md:text-2xl text-[#86868b] max-w-3xl mx-auto mb-12 leading-relaxed">
					We're on a mission to revolutionize how businesses showcase their products and services online.
				</p>
				<div class="max-w-4xl mx-auto">
					<div class="neumorphic p-8 rounded-2xl">
						<p class="text-[#1d1d1f] text-lg leading-relaxed mb-6">
							Founded in 2023, ApexLinks is a platform that empowers businesses of all sizes to create stunning product and service catalogs that engage customers and drive sales.
						</p>
						<p class="text-[#1d1d1f] text-lg leading-relaxed">
							Our AI-powered platform combines beautiful design with intelligent features to create an unparalleled catalog experience for both businesses and their customers.
						</p>
					</div>
				</div>
			</div>
		</header>
		
		<!-- Mission Section -->
		<section class="py-24 bg-gradient-cool relative" bind:this={missionSection}>
			<div class="absolute inset-0 bg-subtle-grid"></div>
			<!-- Decorative shapes -->
			<div class="absolute top-10 left-10 w-40 h-40 rounded-full border border-[#0a84ff]/10 opacity-60"></div>
			<div class="absolute bottom-10 right-10 w-60 h-60 rounded-full border border-[#0a84ff]/10 opacity-60"></div>
			
			<div class="container mx-auto px-4 relative z-10">
				<h2 class="section-title text-center text-[#1d1d1f] mb-16">Our Mission</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-12 max-w-5xl mx-auto">
					<div class="neumorphic p-10 benefit-item">
						<h3 class="text-2xl font-semibold text-[#1d1d1f] mb-6">For Businesses</h3>
						<p class="text-[#86868b] mb-6">
							We provide powerful tools that make it easy to showcase products and services with beautiful, customizable catalogs that reflect your brand identity.
						</p>
						<p class="text-[#86868b]">
							Our platform helps you connect with customers more effectively, understand their needs, and convert interest into sales.
						</p>
					</div>
					
					<div class="neumorphic p-10 benefit-item">
						<h3 class="text-2xl font-semibold text-[#1d1d1f] mb-6">For Customers</h3>
						<p class="text-[#86868b] mb-6">
							We create intuitive, engaging experiences that make it easy to find exactly what you're looking for, with AI-powered search and recommendation features.
						</p>
						<p class="text-[#86868b]">
							Our platform connects you with the right products and services while providing all the information you need to make confident decisions.
						</p>
					</div>
				</div>
			</div>
		</section>
		
		<div class="section-divider my-0"></div>
		
		<!-- Features Section -->
		<section class="py-24 bg-gradient-soft relative">
			<div class="absolute inset-0 bg-subtle-dots"></div>
			
			<div class="container mx-auto px-4 relative z-10">
				<h2 class="section-title text-center text-[#1d1d1f] mb-16">What Sets Us Apart</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-3 gap-10 max-w-5xl mx-auto">
					<div class="neumorphic p-8 text-center feature-item">
						<div class="feature-icon mx-auto">
							<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"></path><rect width="16" height="12" x="4" y="8" rx="2"></rect><path d="M2 14h2"></path><path d="M20 14h2"></path><path d="M15 13v2"></path><path d="M9 13v2"></path></svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">AI-Powered Features</h3>
						<p class="text-[#86868b]">Our advanced AI understands what customers are looking for and helps them find the perfect match in your catalog.</p>
					</div>
					
					<div class="neumorphic p-8 text-center feature-item">
						<div class="feature-icon mx-auto">
							<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"></path><path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"></path><path d="M12 2v2"></path><path d="M12 22v-2"></path><path d="m17 20.66-1-1.73"></path><path d="M11 10.27 7 3.34"></path><path d="m20.66 17-1.73-1"></path><path d="m3.34 7 1.73 1"></path><path d="M14 12h8"></path><path d="M2 12h2"></path><path d="m20.66 7-1.73 1"></path><path d="m3.34 17 1.73-1"></path><path d="m17 3.34-1 1.73"></path><path d="m7 20.66 1-1.73"></path></svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Beautiful Design</h3>
						<p class="text-[#86868b]">We believe in the power of visual appeal. Our platform helps you create stunning catalogs that capture attention and elevate your brand.</p>
					</div>
					
					<div class="neumorphic p-8 text-center feature-item">
						<div class="feature-icon mx-auto">
							<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22a8 8 0 0 0 8-8"></path><path d="M20 10V4h-6"></path><path d="m14 10 6-6"></path><path d="M12 2a8 8 0 0 0-8 8"></path><path d="M4 14v6h6"></path><path d="m10 14-6 6"></path></svg>
						</div>
						<h3 class="text-xl font-semibold mb-3 text-[#1d1d1f]">Seamless Experience</h3>
						<p class="text-[#86868b]">Whether on desktop or mobile, your catalog provides a consistent, engaging experience that keeps customers coming back.</p>
					</div>
				</div>
			</div>
		</section>
		
		<div class="section-divider my-0"></div>
		
		<!-- CTA Section -->
		<section class="py-20 relative overflow-hidden">
			<div class="absolute inset-0 bg-gradient-to-br from-[#0a84ff] to-[#54c7fc]"></div>
			
			<!-- Decorative shapes -->
			<div class="absolute top-0 left-0 w-full h-full overflow-hidden">
				<div class="absolute -top-10 -right-10 w-80 h-80 rounded-full bg-white opacity-5 parallax-element" data-speed="0.02"></div>
				<div class="absolute bottom-0 left-10 w-60 h-60 rounded-full bg-white opacity-5 parallax-element" data-speed="0.03"></div>
			</div>
			
			<div class="container mx-auto px-4 text-center relative z-10">
				<h2 class="text-4xl md:text-5xl font-bold mb-6 tracking-tight text-white">
					Join the ApexLinks Community
				</h2>
				<p class="text-xl md:text-2xl max-w-3xl mx-auto mb-10 text-white opacity-90">
					Ready to transform how you showcase your products and services?
				</p>
				<div class="flex flex-col sm:flex-row justify-center gap-4">
					<a 
						href="/auth/register" 
						class="bg-white text-[#0a84ff] px-8 py-4 rounded-xl text-lg font-medium hover:bg-opacity-90 transition-all"
					>
						Get Started Today
					</a>
					<a 
						href="/i" 
						class="bg-transparent border-2 border-white px-8 py-4 rounded-xl text-lg font-medium hover:bg-white hover:bg-opacity-10 transition-all"
					>
						Browse Catalogs
					</a>
				</div>
			</div>
		</section>
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
	
	:global(.glass-button:hover) {
		background: rgba(42, 46, 110, 0.35);
		box-shadow: 0 8px 25px rgba(163, 241, 255, 0.25);
		transform: translateY(-2px);
	}
</style> 