<script lang="ts">
	import { Button, Card } from 'flowbite-svelte';
	import ParticleBackground from '$lib/ParticleBackground.svelte';
</script>

<svelte:head>
	<title>Grade 1 Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Grade 1 class at AngelWings - Beginning formal education with reading, writing, and numeracy." />
</svelte:head>

<ParticleBackground />

<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">📖</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">Grade 1 Class</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Beginning Formal Education with Reading, Writing, and Numeracy
			</p>
			<Button href="/" class="btn-primary-custom glow-blue motion-preset-bounce" size="lg">
				Back to Home
			</Button>
		</div>
	</div>
</section>

<section class="py-20 px-4">
	<div class="container mx-auto max-w-4xl text-center">
		<Card class="neumorphic-accent p-12 border-0">
			<h2 class="text-3xl font-bold mb-6 text-glow">Grade 1 - Coming Soon</h2>
			<p class="text-xl mb-8 opacity-90">
				We're preparing detailed information about our Grade 1 program. 
				Contact us for more information about this class level.
			</p>
			<img src="/images/fire.jpg" 
				 alt="Grade 1 classroom with beginning readers, math manipulatives, writing centers, and students starting their formal education journey" 
				 class="w-full h-64 object-cover rounded-xl mb-6" />
		</Card>
	</div>
</section> 