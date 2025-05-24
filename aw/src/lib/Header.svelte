<script lang="ts">
	import { onMount } from 'svelte';
	
	let mobileMenuOpen = false;
	let mounted = false;
	
	const navigationItems = [
		{ href: "/", label: "Home" },
		{ href: "#about", label: "About" },
		{ href: "#classes", label: "Classes" },
		{ href: "#programs", label: "Programs" },
		{ href: "#facilities", label: "Facilities" },
		{ href: "#location", label: "Location" },
		{ href: "#contact", label: "Contact" }
	];
	
	onMount(() => {
		mounted = true;
	});
	
	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}
	
	function closeMobileMenu() {
		mobileMenuOpen = false;
	}
	
	function handleCTA() {
		window.location.href = "#contact";
	}
</script>

<!-- Sticky Navigation Header -->
<nav class="fixed w-full z-50 top-0 start-0 backdrop-blur-15 motion-preset-slide-down motion-delay-300">
	<div class="neumorphic border-b border-white/30">
		<div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
			
			<!-- Logo and Brand -->
			<a href="/" class="flex items-center space-x-3 rtl:space-x-reverse group">
				<img 
					src="/images/logo.jpg" 
					class="h-10 w-10 rounded-xl neumorphic-sm group-hover:glow-blue transition-all duration-300" 
					alt="AngelWings Logo" 
				/>
				<span class="self-center text-2xl font-bold whitespace-nowrap text-primary group-hover:text-glow transition-all duration-300">
					AngelWings
				</span>
			</a>
			
			<!-- Mobile menu button and CTA container -->
			<div class="flex desktop:order-2 space-x-3 desktop:space-x-0 rtl:space-x-reverse">
				<!-- CTA Button -->
				<button 
					type="button" 
					class="btn-primary-custom glow-blue motion-preset-bounce hidden desktop:block"
					onclick={handleCTA}
				>
					Apply Now
				</button>
				
				<!-- Mobile Menu Toggle -->
				<button 
					type="button" 
					class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm rounded-lg desktop:hidden neumorphic-sm hover:neumorphic-accent transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-primary/30"
					aria-controls="navbar-sticky" 
					aria-expanded={mobileMenuOpen}
					onclick={toggleMobileMenu}
				>
					<span class="sr-only">Open main menu</span>
					<svg class="w-5 h-5 text-primary transition-transform duration-300 {mobileMenuOpen ? 'rotate-90' : ''}" 
						 aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
						<path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
					</svg>
				</button>
			</div>
			
			<!-- Navigation Menu -->
			<div class="items-center justify-between {mobileMenuOpen ? 'block' : 'hidden'} w-full desktop:flex desktop:w-auto desktop:order-1" id="navbar-sticky">
				<ul class="flex flex-col p-4 desktop:p-0 mt-4 font-medium rounded-lg neumorphic-inset desktop:space-x-8 rtl:space-x-reverse desktop:flex-row desktop:mt-0 desktop:border-0 desktop:bg-transparent">
					{#each navigationItems as item, index}
						<li class="motion-preset-slide-down" style="animation-delay: {index * 100}ms;">
							<a 
								href={item.href} 
								class="block py-3 px-4 rounded-xl transition-all duration-300 
									   {item.href === '/' ? 'text-primary bg-primary/10 neumorphic-sm border-glow' : 'text-slate-700 hover:bg-primary/10 hover:text-primary hover:neumorphic-sm'} 
									   desktop:bg-transparent desktop:p-0 desktop:hover:text-glow group desktop:py-2 desktop:px-3"
								onclick={closeMobileMenu}
							>
								<span class="group-hover:motion-preset-bounce">{item.label}</span>
							</a>
						</li>
					{/each}
					
					<!-- Mobile CTA Button -->
					<li class="desktop:hidden mt-4">
						<button 
							class="w-full btn-primary-custom glow-blue motion-preset-bounce"
							onclick={handleCTA}
						>
							Apply Now
						</button>
					</li>
				</ul>
			</div>
		</div>
	</div>
</nav>

<!-- Spacer to prevent content from hiding behind fixed header -->
<div class="h-20"></div>

<style>
	/* Custom hover effects for navigation */
	nav a:hover {
		transform: translateY(-1px);
	}
	
	/* Mobile menu animation */
	#navbar-sticky {
		transition: all 0.3s ease-in-out;
	}
	
	/* Enhanced logo glow effect */
	.group:hover img {
		box-shadow: 0 0 20px rgba(14, 165, 233, 0.4);
	}
</style> 