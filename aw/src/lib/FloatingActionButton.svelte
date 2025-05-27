<script lang="ts">
	import { onMount } from 'svelte';
	
	let isOpen = false;
	let mounted = false;
	
	const actions = [
		{
			label: "Call School",
			icon: "📞",
			action: () => window.location.href = "tel:+2348036513400",
			color: "btn-primary",
			description: "Speak with our admissions team"
		},
		{
			label: "Send Email",
			icon: "✉️",
			action: () => window.location.href = "mailto:angelwingseducation@gmail.com",
			color: "btn-secondary",
			description: "Email us your questions"
		},
		{
			label: "View Location",
			icon: "📍",
			action: () => window.location.href = "#location",
			color: "btn-accent",
			description: "Find our campus"
		},
		{
			label: "Explore Clubs",
			icon: "🎯",
			action: () => window.location.href = "/clubs",
			color: "btn-outline",
			description: "Discover activities"
		}
	];
	
	onMount(() => {
		mounted = true;
	});
	
	function toggleMenu() {
		isOpen = !isOpen;
	}
	
	function handleAction(actionFn: () => void) {
		actionFn();
		isOpen = false;
	}
</script>

<!-- Floating Action Button -->
<div class="fixed bottom-8 right-8 z-50 motion-preset-slide-up motion-delay-1000">
	<!-- Action Menu -->
	{#if isOpen}
		<div class="flex flex-col space-y-3 mb-6">
			{#each actions as action, index}
				<div 
					class="group cursor-pointer motion-preset-slide-right hover:motion-preset-bounce"
					style="animation-delay: {index * 120}ms;"
					onclick={() => handleAction(action.action)}
				>
					<div class="flex items-center space-x-3 neumorphic-accent p-4 hover:neumorphic transition-all duration-300 rounded-xl min-w-[200px]">
						<div class="flex-shrink-0">
							<div class="w-12 h-12 neumorphic-sm rounded-full flex items-center justify-center text-xl group-hover:glow-blue transition-all duration-300">
								{action.icon}
							</div>
						</div>
						<div class="flex-1 text-left">
							<div class="font-bold text-secondary group-hover:text-glow transition-all">
								{action.label}
							</div>
							<div class="text-xs opacity-70">
								{action.description}
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
	
	<!-- Main FAB -->
	<button
		class="w-16 h-16 neumorphic-accent rounded-full flex items-center justify-center text-3xl text-secondary hover:text-primary transition-all duration-300 motion-preset-bounce hover:motion-preset-pulse hover:glow-blue shadow-2xl relative overflow-hidden group"
		onclick={toggleMenu}
		aria-label={isOpen ? 'Close contact menu' : 'Open contact options'}
	>
		<!-- Background glow effect -->
		<div class="absolute inset-0 bg-gradient-to-r from-secondary/20 to-primary/20 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
		
		<!-- Icon with rotation effect -->
		<span class="relative z-10 transition-transform duration-300 {isOpen ? 'rotate-45' : ''} floating-element">
			{isOpen ? '✕' : '🚀'}
		</span>
		
		<!-- Pulse rings -->
		<div class="absolute inset-0 rounded-full border-2 border-secondary/30 animate-ping opacity-75"></div>
		<div class="absolute inset-2 rounded-full border border-secondary/20 animate-ping opacity-50" style="animation-delay: 0.5s;"></div>
	</button>
</div>

<!-- Backdrop -->
{#if isOpen}
	<button 
		class="fixed inset-0 bg-secondary/10 backdrop-blur-sm z-40 motion-preset-fade-in cursor-default"
		onclick={() => isOpen = false}
		onkeydown={(e) => e.key === 'Escape' && (isOpen = false)}
		aria-label="Close contact menu"
	></button>
{/if} 