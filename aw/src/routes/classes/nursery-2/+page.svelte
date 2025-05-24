<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Card, Badge, List, Li } from 'flowbite-svelte';
	import ParticleBackground from '$lib/ParticleBackground.svelte';
	
	let mounted = false;
	
	onMount(() => {
		mounted = true;
	});

	const curriculum = [
		{
			subject: "Reading Readiness",
			description: "Developing foundational literacy skills",
			icon: "📚",
			activities: ["Letter sounds", "Simple words", "Reading practice", "Comprehension games"]
		},
		{
			subject: "Mathematical Foundations",
			description: "Building number sense and basic math concepts",
			icon: "🔢",
			activities: ["Number recognition to 50", "Basic addition/subtraction", "Measurement concepts", "Time and money"]
		},
		{
			subject: "Science Discovery",
			description: "Exploring natural phenomena through investigation",
			icon: "🔬",
			activities: ["Simple experiments", "Plant growth", "Animal life cycles", "Weather patterns"]
		},
		{
			subject: "Creative Expression",
			description: "Developing artistic and creative abilities",
			icon: "🎨",
			activities: ["Advanced crafts", "Storytelling", "Music appreciation", "Drama activities"]
		},
		{
			subject: "Social & Emotional Learning",
			description: "Building character and interpersonal skills",
			icon: "💝",
			activities: ["Conflict resolution", "Empathy building", "Team cooperation", "Responsibility tasks"]
		},
		{
			subject: "Physical Development",
			description: "Enhancing motor skills and health awareness",
			icon: "🏃‍♀️",
			activities: ["Fine motor practice", "Sports introduction", "Health habits", "Safety awareness"]
		}
	];

	const achievements = [
		"Pre-reading skills and phonetic awareness",
		"Basic mathematical operations and number sense",
		"Improved concentration and attention span",
		"Enhanced social skills and cooperation",
		"Creative problem-solving abilities",
		"Readiness for formal Grade 1 education"
	];
</script>

<svelte:head>
	<title>Nursery 2 Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Nursery 2 class at AngelWings - Developing fundamental skills in preparation for formal education for children aged 4-5 years." />
</svelte:head>

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">🎯</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">
				Nursery 2 Class
			</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Developing Fundamental Skills in Preparation for Formal Education
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center motion-preset-slide-up motion-delay-1000">
				<Button href="#curriculum" class="btn-primary-custom glow-blue motion-preset-bounce motion-delay-1200" size="lg">
					Explore Curriculum
				</Button>
				<Button href="#contact" class="btn-outline-custom border-glow motion-preset-bounce motion-delay-1400" size="lg">
					Schedule Visit
				</Button>
			</div>
		</div>
	</div>
	
	<!-- Floating Background Elements -->
	<div class="absolute inset-0 overflow-hidden pointer-events-none">
		<div class="absolute top-20 left-10 w-32 h-32 neumorphic-accent rounded-full floating-element opacity-20"></div>
		<div class="absolute top-40 right-20 w-24 h-24 neumorphic rounded-full drifting-element opacity-30" style="animation-delay: 2s;"></div>
		<div class="absolute bottom-40 left-1/4 w-40 h-40 neumorphic-accent rounded-full floating-element opacity-15" style="animation-delay: 4s;"></div>
		<div class="absolute bottom-20 right-1/3 w-28 h-28 neumorphic rounded-full drifting-element opacity-25" style="animation-delay: 6s;"></div>
	</div>
</section>

<!-- Overview Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="grid lg:grid-cols-2 gap-12 items-center">
			<Card class="neumorphic p-8 motion-preset-slide-right border-0">
				<h2 class="text-3xl font-bold mb-6 text-primary">About Nursery 2 Class</h2>
				<p class="text-lg mb-6 opacity-90">
					Nursery 2 serves as the crucial bridge between early childhood learning and formal education. 
					Children aged 4-5 years develop essential academic readiness skills while maintaining 
					the joy of discovery and creativity that makes learning meaningful and lasting.
				</p>
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">4-5</div>
						<div class="text-sm opacity-80">Years Old</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">1:12</div>
						<div class="text-sm opacity-80">Teacher Ratio</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">7</div>
						<div class="text-sm opacity-80">Hours Daily</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">Structured</div>
						<div class="text-sm opacity-80">Learning</div>
					</div>
				</div>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/placeholder-nursery2.jpg" 
					 alt="Nursery 2 classroom with structured learning areas, alphabet displays, number charts, and children engaged in focused educational activities" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- Curriculum Section -->
<section id="curriculum" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Nursery 2 Curriculum</h2>
			<p class="text-xl opacity-90">
				Comprehensive preparation for primary school success
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each curriculum as subject, index}
				<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
					 style="animation-delay: {index * 200}ms;">
					<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">{subject.icon}</div>
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{subject.subject}</h3>
					<p class="opacity-90 mb-4">{subject.description}</p>
					<div class="space-y-2">
						{#each subject.activities as activity}
							<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">{activity}</Badge>
						{/each}
					</div>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Learning Outcomes Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Learning Achievements</h2>
			<p class="text-xl opacity-90">
				What your child will accomplish in Nursery 2 class
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div class="neumorphic-accent p-2 motion-preset-slide-right">
				<img src="/images/placeholder-nursery2-achievements.jpg" 
					 alt="Nursery 2 students confidently participating in classroom activities, showing readiness for Grade 1 with improved focus and academic skills" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
			<Card class="neumorphic p-8 motion-preset-slide-left border-0">
				<h3 class="text-2xl font-bold mb-6 text-primary">Key Achievements</h3>
				<List class="space-y-3">
					{#each achievements as achievement, index}
						<Li class="flex items-center motion-preset-slide-up" style="animation-delay: {index * 100}ms;">
							<span class="text-primary mr-3 text-lg">✓</span>
							<span class="text-lg">{achievement}</span>
						</Li>
					{/each}
				</List>
			</Card>
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-4xl text-center">
		<Card class="neumorphic-accent p-12 motion-preset-slide-up border-0">
			<h2 class="text-3xl font-bold mb-6 text-glow">Prepare Your Child for Success</h2>
			<p class="text-xl mb-8 opacity-90">
				Nursery 2 provides the essential foundation your child needs for a smooth transition to formal education. 
				Schedule a visit to see how we prepare children for their academic journey ahead.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<Button href="/contact" class="btn-primary-custom glow-blue motion-preset-bounce" size="lg">
					Schedule a Visit
				</Button>
				<Button href="/apply" class="btn-outline-custom border-glow motion-preset-bounce" size="lg">
					Apply Now
				</Button>
			</div>
		</Card>
	</div>
</section> 