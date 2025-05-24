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
			subject: "Early Literacy",
			description: "Introduction to letters, sounds, and reading readiness",
			icon: "📖",
			activities: ["Letter recognition", "Phonics games", "Story comprehension", "Writing preparation"]
		},
		{
			subject: "Basic Math",
			description: "Numbers, counting, and simple mathematical concepts",
			icon: "🔢",
			activities: ["Number recognition", "Counting to 20", "Simple addition", "Pattern recognition"]
		},
		{
			subject: "Science Exploration",
			description: "Discovering the world through hands-on experiments",
			icon: "🔬",
			activities: ["Simple experiments", "Nature observation", "Weather tracking", "Animal habitats"]
		},
		{
			subject: "Creative Arts",
			description: "Developing artistic skills and creative expression",
			icon: "🎨",
			activities: ["Drawing and painting", "Craft projects", "Music and songs", "Creative writing"]
		},
		{
			subject: "Social Studies",
			description: "Understanding community, family, and relationships",
			icon: "🏘️",
			activities: ["Family trees", "Community helpers", "Cultural awareness", "Good citizenship"]
		},
		{
			subject: "Physical Education",
			description: "Building strength, coordination, and healthy habits",
			icon: "⚽",
			activities: ["Sports basics", "Coordination games", "Team activities", "Health education"]
		}
	];

	const achievements = [
		"Recognition of letters and basic sounds",
		"Counting and number recognition up to 20",
		"Improved fine motor skills for writing",
		"Enhanced creativity and artistic expression",
		"Better social interaction and teamwork skills",
		"Basic understanding of scientific concepts"
	];
</script>

<svelte:head>
	<title>Nursery 1 Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Nursery 1 class at AngelWings - Introduction to structured learning through creative activities for children aged 3-4 years." />
</svelte:head>

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">🎈</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">
				Nursery 1 Class
			</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Introduction to Structured Learning Through Creative Activities
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
				<h2 class="text-3xl font-bold mb-6 text-primary">About Nursery 1 Class</h2>
				<p class="text-lg mb-6 opacity-90">
					Nursery 1 introduces children aged 3-4 years to more structured learning while maintaining 
					a play-based approach. Students begin developing academic skills through engaging activities 
					that prepare them for formal education while fostering creativity and critical thinking.
				</p>
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">3-4</div>
						<div class="text-sm opacity-80">Years Old</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">1:10</div>
						<div class="text-sm opacity-80">Teacher Ratio</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">7</div>
						<div class="text-sm opacity-80">Hours Daily</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">Creative</div>
						<div class="text-sm opacity-80">Learning</div>
					</div>
				</div>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/placeholder-nursery1.jpg" 
					 alt="Nursery 1 classroom with learning centers, interactive displays, child-sized furniture, and students engaged in structured creative activities" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- Curriculum Section -->
<section id="curriculum" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Nursery 1 Curriculum</h2>
			<p class="text-xl opacity-90">
				Structured learning activities designed for early academic development
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
				What your child will accomplish in Nursery 1 class
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div class="neumorphic-accent p-2 motion-preset-slide-right">
				<img src="/images/placeholder-nursery1-achievements.jpg" 
					 alt="Children in Nursery 1 proudly showing their learning progress, reading simple books, and working on creative projects together" 
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
			<h2 class="text-3xl font-bold mb-6 text-glow">Start Your Child's Academic Journey</h2>
			<p class="text-xl mb-8 opacity-90">
				Nursery 1 provides the perfect balance of structured learning and creative play. 
				Visit us to see how we make learning fun and engaging for young minds.
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