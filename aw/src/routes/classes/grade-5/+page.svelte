<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Card, Badge, List, Li, Tabs, TabItem } from 'flowbite-svelte';
	import ParticleBackground from '$lib/ParticleBackground.svelte';
	
	let mounted = false;
	
	onMount(() => {
		mounted = true;
	});

	const curriculum = [
		{
			subject: "Advanced Language Arts",
			description: "Comprehensive reading, writing, and communication skills",
			icon: "📚",
			activities: ["Essay writing", "Literature analysis", "Public speaking", "Creative writing"]
		},
		{
			subject: "Mathematics",
			description: "Advanced arithmetic, pre-algebra, and problem-solving",
			icon: "🔢",
			activities: ["Fractions & decimals", "Basic algebra", "Geometry", "Statistical analysis"]
		},
		{
			subject: "Science & Technology",
			description: "Scientific method, research, and technology integration",
			icon: "🔬",
			activities: ["Lab experiments", "Research projects", "Coding basics", "Scientific reports"]
		},
		{
			subject: "Social Sciences",
			description: "History, geography, civics, and cultural studies",
			icon: "🌍",
			activities: ["Historical research", "Map skills", "Government studies", "Cultural projects"]
		},
		{
			subject: "Creative Arts",
			description: "Advanced artistic expression and multimedia projects",
			icon: "🎨",
			activities: ["Digital art", "Music composition", "Drama production", "Portfolio development"]
		},
		{
			subject: "Life Skills & Leadership",
			description: "Preparing for secondary education and leadership roles",
			icon: "🎯",
			activities: ["Study skills", "Time management", "Leadership projects", "Career exploration"]
		}
	];

	const extracurriculars = [
		{ name: "Robotics Club", icon: "🤖", description: "Building and programming robots" },
		{ name: "Coding Academy", icon: "💻", description: "Learning programming languages" },
		{ name: "Chess Masters", icon: "♟️", description: "Strategic thinking and competition" },
		{ name: "AI Innovation Lab", icon: "🧠", description: "Exploring artificial intelligence" },
		{ name: "Student Government", icon: "🏛️", description: "Leadership and civic engagement" },
		{ name: "Science Olympiad", icon: "🏆", description: "Competitive science challenges" }
	];

	const achievements = [
		"Advanced reading comprehension and analytical thinking",
		"Proficiency in mathematical problem-solving",
		"Scientific research and experimental skills",
		"Effective communication and presentation abilities",
		"Technology literacy and basic programming",
		"Leadership skills and collaborative teamwork",
		"Readiness for secondary education transition"
	];

	const preparationAreas = [
		{
			title: "Academic Excellence",
			description: "Strong foundation in core subjects",
			skills: ["Critical thinking", "Research skills", "Academic writing", "Mathematical reasoning"]
		},
		{
			title: "Technology Proficiency",
			description: "Digital literacy for the modern world",
			skills: ["Basic programming", "Digital tools", "AI applications", "Online collaboration"]
		},
		{
			title: "Leadership Development",
			description: "Building future leaders",
			skills: ["Team management", "Public speaking", "Problem solving", "Project leadership"]
		}
	];
</script>

<svelte:head>
	<title>Grade 5 Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Grade 5 class at AngelWings - Preparing for secondary education with comprehensive learning and leadership development." />
</svelte:head>

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">🎓</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">
				Grade 5 Class
			</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Preparing for Secondary Education with Comprehensive Learning
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
				<h2 class="text-3xl font-bold mb-6 text-primary">About Grade 5 Class</h2>
				<p class="text-lg mb-6 opacity-90">
					Grade 5 represents the culmination of primary education at AngelWings. Students aged 10-11 years 
					engage in advanced learning that prepares them for secondary education while developing leadership 
					skills, critical thinking, and technological proficiency for the modern world.
				</p>
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">10-11</div>
						<div class="text-sm opacity-80">Years Old</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">1:15</div>
						<div class="text-sm opacity-80">Teacher Ratio</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">8</div>
						<div class="text-sm opacity-80">Hours Daily</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">Advanced</div>
						<div class="text-sm opacity-80">Learning</div>
					</div>
				</div>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/light.jpg" 
					 alt="Grade 5 classroom featuring advanced learning environments with technology integration, collaborative spaces, and leadership development areas" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- Curriculum Section -->
<section id="curriculum" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Grade 5 Curriculum</h2>
			<p class="text-xl opacity-90">
				Advanced academic programs preparing students for secondary education
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each curriculum as subject, index}
				<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
					 style="animation-delay: {index * 150}ms;">
					<div class="text-4xl mb-4 motion-preset-bounce motion-delay-500">{subject.icon}</div>
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{subject.subject}</h3>
					<p class="opacity-90 mb-4">{subject.description}</p>
					<div class="space-y-2">
						{#each subject.activities as activity}
							<Badge class="neumorphic-sm px-3 py-1 text-xs">{activity}</Badge>
						{/each}
					</div>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Extracurriculars Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Advanced Extracurriculars</h2>
			<p class="text-xl opacity-90">
				Specialized programs for developing 21st-century skills
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each extracurriculars as activity, index}
				<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
					 style="animation-delay: {index * 150}ms;">
					<div class="text-4xl mb-4 motion-preset-bounce motion-delay-500">{activity.icon}</div>
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{activity.name}</h3>
					<p class="opacity-90">{activity.description}</p>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Preparation Areas Section -->
<section class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Secondary School Preparation</h2>
			<p class="text-xl opacity-90">
				Building essential skills for future academic success
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each preparationAreas as area, index}
				<Card class="neumorphic p-6 hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
					 style="animation-delay: {index * 150}ms;">
					<div class="text-3xl mb-4 motion-preset-bounce motion-delay-500">{area.icon}</div>
					<h3 class="text-lg font-bold mb-3 text-primary group-hover:text-glow transition-all">{area.title}</h3>
					<p class="text-sm opacity-90 mb-4">{area.description}</p>
					<div class="space-y-2">
						{#each area.skills as skill}
							<Badge class="neumorphic-sm px-3 py-1 text-xs">{skill}</Badge>
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
			<h2 class="text-4xl font-bold mb-6 text-glow">Grade 5 Achievements</h2>
			<p class="text-xl opacity-90">
				Comprehensive skills and knowledge gained by graduation
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 gap-12 items-center">
			<Card class="neumorphic p-8 motion-preset-slide-right border-0">
				<h3 class="text-2xl font-bold mb-6 text-primary">Elementary Graduation</h3>
				<p class="text-lg mb-6 opacity-90">
					Our Grade 5 graduation ceremony celebrates the accomplishments of our students as they 
					complete their elementary education. This milestone event recognizes their academic 
					achievements and personal growth, preparing them for the next chapter in their 
					educational journey.
				</p>
				<p class="text-lg opacity-90">
					Students participate in special projects, performances, and reflective activities leading up 
					to graduation, allowing them to showcase their development throughout their years at 
					AngelWings Comprehensive College.
				</p>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/logo.jpg" 
					 alt="Grade 5 graduation ceremony with students in caps and gowns, proud parents, celebration decorations, and ceremonial diplomas" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-4xl text-center">
		<Card class="neumorphic-accent p-12 motion-preset-slide-up border-0">
			<h2 class="text-3xl font-bold mb-6 text-glow">Launch Your Child's Future</h2>
			<p class="text-xl mb-8 opacity-90">
				Grade 5 at AngelWings provides the comprehensive preparation your child needs for secondary 
				education and beyond. Schedule a visit to see our advanced programs in action.
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