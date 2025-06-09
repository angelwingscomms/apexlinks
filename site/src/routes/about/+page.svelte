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
	let teamSection: HTMLElement;
	
	onMount(() => {
		// Initialize animations
		if (headerSection) slideInFromBottom(headerSection);
		if (missionSection) slideInFromLeft(missionSection);
		if (teamSection) slideInFromRight(teamSection);
		
		// Setup scroll-based animations
		createScrollObserver('.feature-item', slideInFromBottom, { once: true, staggerDelay: 150 });
		createScrollObserver('.team-member', slideInFromBottom, { once: true, staggerDelay: 100 });
	});
	
	// Team members data
	const teamMembers = [
		{
			name: 'Sarah Johnson',
			role: 'Founder & CEO',
			bio: 'Sarah founded ApexLinks with a vision to transform how businesses showcase their products online.',
			image: '/images/team/sarah.jpg'
		},
		{
			name: 'Michael Chen',
			role: 'Chief Technology Officer',
			bio: 'Michael leads our engineering team, focusing on building cutting-edge AI-powered features.',
			image: '/images/team/michael.jpg'
		},
		{
			name: 'Aisha Patel',
			role: 'Head of Design',
			bio: 'Aisha ensures our platform delivers beautiful, intuitive experiences for all users.',
			image: '/images/team/aisha.jpg'
		},
		{
			name: 'Carlos Rodriguez',
			role: 'Customer Success Lead',
			bio: 'Carlos helps businesses make the most of their ApexLinks catalog and grow their customer base.',
			image: '/images/team/carlos.jpg'
		}
	];
</script>

<PageTransition>
	<div class="min-h-screen bg-[#151A33] text-[#FDFDFF] pb-16">
		<!-- Background elements -->
		<div class="fixed -z-10 top-1/4 left-1/4 w-64 h-64 rounded-full bg-[#B2B5E0]/10 blur-3xl"></div>
		<div class="fixed -z-10 bottom-1/4 right-1/4 w-72 h-72 rounded-full bg-[#A3F1FF]/10 blur-3xl"></div>
		<div class="fixed -z-10 top-1/2 right-1/3 w-48 h-48 rounded-full bg-[#E5D9FF]/10 blur-3xl"></div>
		
		<!-- Hero Section -->
		<header class="relative py-20" bind:this={headerSection}>
			<div class="container mx-auto px-4 text-center">
				<h1 class="text-5xl md:text-6xl font-bold mb-6 text-[#FDFDFF]">
					About <span class="text-[#A3F1FF]">ApexLinks</span>
				</h1>
				<p class="text-xl text-[#C9CCF5] max-w-3xl mx-auto mb-12">
					We're on a mission to revolutionize how businesses showcase their products and services online.
				</p>
				<div class="max-w-4xl mx-auto">
					<div class="glass p-8 rounded-2xl">
						<p class="text-[#FDFDFF] text-lg leading-relaxed mb-6">
							Founded in 2023, ApexLinks is a platform that empowers businesses of all sizes to create stunning product and service catalogs that engage customers and drive sales.
						</p>
						<p class="text-[#FDFDFF] text-lg leading-relaxed">
							Our AI-powered platform combines beautiful design with intelligent features to create an unparalleled catalog experience for both businesses and their customers.
						</p>
					</div>
				</div>
			</div>
		</header>
		
		<!-- Mission Section -->
		<section class="py-16" bind:this={missionSection}>
			<div class="container mx-auto px-4">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-12 text-center">Our Mission</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-12 max-w-5xl mx-auto">
					<div class="glass p-8 rounded-2xl">
						<h3 class="text-2xl font-semibold text-[#F4E6FF] mb-4">For Businesses</h3>
						<p class="text-[#C9CCF5] mb-6">
							We provide powerful tools that make it easy to showcase products and services with beautiful, customizable catalogs that reflect your brand identity.
						</p>
						<p class="text-[#C9CCF5]">
							Our platform helps you connect with customers more effectively, understand their needs, and convert interest into sales.
						</p>
					</div>
					
					<div class="glass p-8 rounded-2xl">
						<h3 class="text-2xl font-semibold text-[#F4E6FF] mb-4">For Customers</h3>
						<p class="text-[#C9CCF5] mb-6">
							We create intuitive, engaging experiences that make it easy to find exactly what you're looking for, with AI-powered search and recommendation features.
						</p>
						<p class="text-[#C9CCF5]">
							Our platform connects you with the right products and services while providing all the information you need to make confident decisions.
						</p>
					</div>
				</div>
			</div>
		</section>
		
		<!-- Features Section -->
		<section class="py-16 bg-[#1C1F4A]/30">
			<div class="container mx-auto px-4">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-12 text-center">What Sets Us Apart</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
					<div class="glass p-6 rounded-2xl feature-item">
						<div class="text-4xl mb-4 text-[#A3F1FF]">🤖</div>
						<h3 class="text-xl font-semibold text-[#F4E6FF] mb-3">AI-Powered Features</h3>
						<p class="text-[#C9CCF5]">
							Our advanced AI understands what customers are looking for and helps them find the perfect match in your catalog.
						</p>
					</div>
					
					<div class="glass p-6 rounded-2xl feature-item">
						<div class="text-4xl mb-4 text-[#A3F1FF]">🎨</div>
						<h3 class="text-xl font-semibold text-[#F4E6FF] mb-3">Beautiful Design</h3>
						<p class="text-[#C9CCF5]">
							We believe in the power of visual appeal. Our platform helps you create stunning catalogs that capture attention and elevate your brand.
						</p>
					</div>
					
					<div class="glass p-6 rounded-2xl feature-item">
						<div class="text-4xl mb-4 text-[#A3F1FF]">📱</div>
						<h3 class="text-xl font-semibold text-[#F4E6FF] mb-3">Seamless Experience</h3>
						<p class="text-[#C9CCF5]">
							Whether on desktop or mobile, your catalog provides a consistent, engaging experience that keeps customers coming back.
						</p>
					</div>
				</div>
			</div>
		</section>
		
		<!-- Team Section -->
		<section class="py-16" bind:this={teamSection}>
			<div class="container mx-auto px-4">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-12 text-center">Meet Our Team</h2>
				
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 max-w-6xl mx-auto">
					{#each teamMembers as member, index}
						<div class="glass p-6 rounded-2xl team-member flex flex-col items-center text-center">
							<div class="w-32 h-32 rounded-full overflow-hidden mb-4 border-2 border-[#A3F1FF]/20">
								<img 
									src={member.image} 
									alt={member.name}
									class="w-full h-full object-cover"
									on:error={(e) => {
										const target = e.target as HTMLImageElement;
										target.src = `https://ui-avatars.com/api/?name=${encodeURIComponent(member.name)}&background=2A2E6E&color=FDFDFF&size=128`;
									}}
								/>
							</div>
							<h3 class="text-xl font-semibold text-[#F4E6FF] mb-1">{member.name}</h3>
							<p class="text-[#A3F1FF] text-sm mb-3">{member.role}</p>
							<p class="text-[#C9CCF5] text-sm">{member.bio}</p>
						</div>
					{/each}
				</div>
			</div>
		</section>
		
		<!-- CTA Section -->
		<section class="py-16">
			<div class="container mx-auto px-4 text-center">
				<h2 class="text-3xl font-bold text-[#FDFDFF] mb-6">Join the ApexLinks Community</h2>
				<p class="text-xl text-[#C9CCF5] max-w-3xl mx-auto mb-8">
					Ready to transform how you showcase your products and services?
				</p>
				<div class="flex flex-col md:flex-row justify-center gap-4">
					<a 
						href="/auth/register" 
						class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium"
					>
						Get Started Today
					</a>
					<a 
						href="/i" 
						class="glass-button px-8 py-4 text-[#FDFDFF] text-lg font-medium"
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