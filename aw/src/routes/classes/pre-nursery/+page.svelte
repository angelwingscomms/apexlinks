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
			subject: "Language Development",
			description: "Building vocabulary and communication skills",
			icon: "🗣️",
			activities: ["Story time", "Vocabulary games", "Conversation circles", "Picture descriptions"]
		},
		{
			subject: "Basic Numeracy",
			description: "Introduction to numbers and mathematical concepts",
			icon: "🔢",
			activities: ["Number songs", "Counting exercises", "Shape recognition", "Size comparisons"]
		},
		{
			subject: "Creative Arts",
			description: "Exploring artistic expression and creativity",
			icon: "🎨",
			activities: ["Drawing and coloring", "Simple crafts", "Music and rhythm", "Dramatic play"]
		},
		{
			subject: "Social Development",
			description: "Learning to interact and cooperate with others",
			icon: "👥",
			activities: ["Group games", "Sharing activities", "Circle time", "Friendship building"]
		},
		{
			subject: "Physical Development",
			description: "Developing gross and fine motor skills",
			icon: "🏃",
			activities: ["Playground time", "Dance and movement", "Building blocks", "Manipulative toys"]
		},
		{
			subject: "Nature & Science",
			description: "Discovering the natural world around us",
			icon: "🌿",
			activities: ["Nature walks", "Simple experiments", "Animal studies", "Weather observation"]
		}
	];

	const achievements = [
		"Improved communication and language skills",
		"Enhanced social interaction abilities",
		"Basic number and counting recognition",
		"Creative self-expression through arts",
		"Development of listening and following directions",
		"Increased independence in daily activities"
	];
</script>

<svelte:head>
	<title>Pre-Nursery Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Pre-Nursery class at AngelWings - Nurturing curiosity and social skills in our youngest learners aged 2-3 years." />
</svelte:head>

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">🐣</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">
				Pre-Nursery Class
			</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Nurturing Curiosity and Social Skills in Our Youngest Learners
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
				<h2 class="text-3xl font-bold mb-6 text-primary">About Pre-Nursery Class</h2>
				<p class="text-lg mb-6 opacity-90">
					Our Pre-Nursery class is designed for children aged 2-3 years, focusing on developing essential 
					social skills, communication abilities, and basic learning readiness. Through gentle guidance 
					and nurturing care, we help young children transition into a structured learning environment.
				</p>
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">2-3</div>
						<div class="text-sm opacity-80">Years Old</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">1:6</div>
						<div class="text-sm opacity-80">Teacher Ratio</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">6</div>
						<div class="text-sm opacity-80">Hours Daily</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">Gentle</div>
						<div class="text-sm opacity-80">Learning</div>
					</div>
				</div>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/light.jpg" 
					 alt="Pre-nursery classroom with soft play areas, age-appropriate toys, comfortable reading corner, and caring teacher interacting with toddlers" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- Curriculum Section -->
<section id="curriculum" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Pre-Nursery Curriculum</h2>
			<p class="text-xl opacity-90">
				Age-appropriate activities for early childhood development
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
				What your child will accomplish in Pre-Nursery class
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div class="neumorphic-accent p-2 motion-preset-slide-right">
				<img src="/images/beads.jpg" 
					 alt="Happy toddlers engaged in group activities, showing improved social skills and communication with peers and teachers" 
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
			<h2 class="text-3xl font-bold mb-6 text-glow">Give Your Toddler the Best Start</h2>
			<p class="text-xl mb-8 opacity-90">
				Our Pre-Nursery program provides a gentle introduction to learning and socialization. 
				Schedule a visit to see how we nurture your child's natural curiosity and growth.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<Button href="/" class="btn-primary-custom glow-blue motion-preset-bounce" size="lg">
					Schedule a Visit
				</Button>
				<Button href="/" class="btn-outline-custom border-glow motion-preset-bounce" size="lg">
					Apply Now
				</Button>
			</div>
		</Card>
	</div>
</section> 