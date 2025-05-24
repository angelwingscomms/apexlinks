<script lang="ts">
	import { onMount } from 'svelte';
	
	let isOpen = false;
	let mounted = false;
	
	const actions = [
		{
			label: "Apply Now",
			icon: "📝",
			action: () => window.location.href = "#contact",
			color: "btn-primary"
		},
		{
			label: "Schedule Tour",
			icon: "🏫",
			action: () => window.location.href = "#facilities",
			color: "btn-secondary"
		},
		{
			label: "Contact Us",
			icon: "📞",
			action: () => window.location.href = "#contact",
			color: "btn-accent"
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
		<div class="flex flex-col space-y-3 mb-4">
			{#each actions as action, index}
				<button
					class="btn {action.color} btn-sm glow-red motion-preset-slide-right group"
					style="animation-delay: {index * 100}ms;"
					onclick={() => handleAction(action.action)}
				>
					<span class="mr-2 group-hover:motion-preset-bounce">{action.icon}</span>
					{action.label}
				</button>
			{/each}
		</div>
	{/if}
	
	<!-- Main FAB -->
	<button
		class="btn btn-circle btn-primary btn-lg glow-red shadow-2xl motion-preset-bounce hover:motion-preset-pulse"
		onclick={toggleMenu}
		aria-label={isOpen ? 'Close action menu' : 'Open quick actions menu'}
	>
		<span class="text-2xl transition-transform duration-300 {isOpen ? 'rotate-45' : ''}">
			{isOpen ? '✕' : '🚀'}
		</span>
	</button>
</div>

<!-- Backdrop -->
{#if isOpen}
	<button 
		class="fixed inset-0 bg-black/20 z-40 motion-preset-fade-in cursor-default"
		onclick={() => isOpen = false}
		onkeydown={(e) => e.key === 'Escape' && (isOpen = false)}
		aria-label="Close action menu"
	></button>
{/if} 