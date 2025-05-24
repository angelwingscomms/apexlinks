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
			subject: "Pre-Reading Skills",
			description: "Letter recognition, phonemic awareness, and storytelling",
			icon: "📚",
			activities: ["Alphabet games", "Picture books", "Letter tracing", "Rhyming activities"]
		},
		{
			subject: "Early Mathematics",
			description: "Number recognition, counting, shapes, and patterns",
			icon: "🔢",
			activities: ["Counting songs", "Shape exploration", "Pattern making", "Number games"]
		},
		{
			subject: "Motor Development",
			description: "Fine and gross motor skills through play",
			icon: "🤸",
			activities: ["Drawing exercises", "Building blocks", "Physical games", "Playground activities"]
		},
		{
			subject: "Social Skills",
			description: "Sharing, cooperation, and emotional expression",
			icon: "🤝",
			activities: ["Group activities", "Sharing circles", "Role playing", "Friendship games"]
		},
		{
			subject: "Creative Expression",
			description: "Art, music, and imaginative play",
			icon: "🎨",
			activities: ["Finger painting", "Music time", "Story creation", "Dance movement"]
		},
		{
			subject: "Nature Exploration",
			description: "Discovering the world around us",
			icon: "🌱",
			activities: ["Garden visits", "Weather observation", "Animal learning", "Outdoor exploration"]
		}
	];

	const dailySchedule = [
		{ time: "8:00 - 8:30", activity: "Welcome Circle & Free Play", color: "from-yellow-400 to-amber-500" },
		{ time: "8:30 - 9:15", activity: "Learning Centers Exploration", color: "from-green-400 to-emerald-500" },
		{ time: "9:15 - 9:45", activity: "Snack Time & Story", color: "from-blue-400 to-cyan-500" },
		{ time: "9:45 - 10:30", activity: "Creative Arts & Crafts", color: "from-purple-400 to-violet-500" },
		{ time: "10:30 - 11:15", activity: "Outdoor Play & Movement", color: "from-orange-400 to-red-500" },
		{ time: "11:15 - 12:00", activity: "Music & Movement Time", color: "from-pink-400 to-rose-500" },
		{ time: "12:00 - 1:00", activity: "Lunch & Rest Time", color: "from-indigo-400 to-blue-500" }
	];

	const achievements = [
		"Building confidence in social interactions",
		"Developing early literacy foundations", 
		"Expressing creativity through multiple mediums",
		"Learning through hands-on exploration",
		"Establishing healthy routines and habits",
		"Fostering curiosity about the world"
	];
</script>

<svelte:head>
	<title>Foundation Class - AngelWings Comprehensive College</title>
	<meta name="description" content="Foundation class at AngelWings - Building essential early learning skills through play-based discovery for our youngest learners." />
</svelte:head>

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="container mx-auto px-4 text-center relative z-10">
		<div class="max-w-4xl mx-auto motion-preset-fade-in motion-delay-300">
			<div class="text-8xl mb-6 motion-preset-bounce motion-delay-500">🌱</div>
			<h1 class="text-6xl font-bold mb-6 text-glow animate-float">
				Foundation Class
			</h1>
			<p class="text-2xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Building Essential Early Learning Skills Through Play-Based Discovery
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center motion-preset-slide-up motion-delay-1000">
				<Button href="#programs" class="btn-primary-custom glow-blue motion-preset-bounce motion-delay-1200" size="lg">
					Explore Programs
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
<section id="about" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="grid lg:grid-cols-2 gap-12 items-center">
			<Card class="neumorphic p-8 motion-preset-slide-right border-0">
				<h2 class="text-3xl font-bold mb-6 text-primary">About Foundation Class</h2>
				<p class="text-lg mb-6 opacity-90">
					Our Foundation class provides the perfect introduction to formal learning for children aged 3-4 years. 
					Through carefully designed play-based activities, we nurture essential skills that form the foundation 
					for future academic success while fostering a love for learning.
				</p>
				<div class="grid grid-cols-2 gap-4 mb-6">
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">3-4</div>
						<div class="text-sm opacity-80">Years Old</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">1:8</div>
						<div class="text-sm opacity-80">Teacher Ratio</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">7</div>
						<div class="text-sm opacity-80">Hours Daily</div>
					</div>
					<div class="text-center">
						<div class="text-2xl font-bold text-primary">Play</div>
						<div class="text-sm opacity-80">Based Learning</div>
					</div>
				</div>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/light.jpg" 
					 alt="Foundation classroom with colorful learning materials, small tables and chairs, educational toys, and bright natural lighting creating a warm learning environment" 
					 class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<section id="classes"></section>

<!-- Programs Section -->
<section id="programs" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Foundation Programs</h2>
			<p class="text-xl opacity-90">
				Specialized programs designed to support your child's development journey
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
			<!-- Early Literacy Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 100ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">📖</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Early Literacy</h3>
				<p class="opacity-90 mb-4">Building strong reading foundations through phonics, storytelling, and interactive reading activities.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Letter Recognition</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Phonics Games</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Story Time</Badge>
				</div>
			</Card>

			<!-- Math Readiness Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 200ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">🧮</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Math Readiness</h3>
				<p class="opacity-90 mb-4">Developing number sense and mathematical thinking through hands-on exploration and play.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Number Games</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Shape Discovery</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Pattern Making</Badge>
				</div>
			</Card>

			<!-- Social Development Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 300ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">👥</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Social Skills</h3>
				<p class="opacity-90 mb-4">Fostering emotional intelligence, cooperation, and friendship skills in a supportive environment.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Circle Time</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Sharing Activities</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Emotion Games</Badge>
				</div>
			</Card>

			<!-- Creative Arts Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 400ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">🎨</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Creative Arts</h3>
				<p class="opacity-90 mb-4">Encouraging self-expression and creativity through various artistic mediums and music.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Art Projects</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Music & Movement</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Drama Play</Badge>
				</div>
			</Card>

			<!-- Nature Discovery Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 500ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">🌿</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Nature Discovery</h3>
				<p class="opacity-90 mb-4">Connecting children with nature through outdoor exploration and environmental awareness.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Garden Time</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Weather Watch</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Animal Friends</Badge>
				</div>
			</Card>

			<!-- Motor Skills Program -->
			<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
				 style="animation-delay: 600ms;">
				<div class="text-5xl mb-4 motion-preset-bounce motion-delay-500">🏃</div>
				<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Motor Development</h3>
				<p class="opacity-90 mb-4">Building strength, coordination, and fine motor skills through purposeful movement activities.</p>
				<div class="space-y-2">
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Playground Fun</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Fine Motor Tasks</Badge>
					<Badge class="mr-2 mb-2 bg-primary/10 text-primary border-primary/20">Balance Games</Badge>
				</div>
			</Card>
		</div>
	</div>
</section>

<!-- Curriculum Section -->
<section id="curriculum" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Foundation Curriculum</h2>
			<p class="text-xl opacity-90">
				Comprehensive learning areas designed for early childhood development
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

<!-- Daily Schedule Section -->
<section class="py-20 px-4">
	<div class="container mx-auto max-w-4xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Daily Schedule</h2>
			<p class="text-xl opacity-90">
				A structured yet flexible day designed for optimal learning and development
			</p>
		</div>
		
		<Card class="neumorphic-accent p-8 motion-preset-slide-up border-0">
			<div class="space-y-6">
				{#each dailySchedule as item, index}
					<div class="flex items-center space-x-4 motion-preset-slide-up" style="animation-delay: {index * 200}ms;">
						<Badge class="bg-gradient-to-r {item.color} text-white font-semibold px-4 py-2 min-w-max">
							{item.time}
						</Badge>
						<div class="flex-1">
							<div class="text-lg font-medium">{item.activity}</div>
						</div>
						<div class="w-4 h-4 bg-primary rounded-full"></div>
					</div>
					{#if index < dailySchedule.length - 1}
						<div class="ml-16 border-l-2 border-primary/30 h-4"></div>
					{/if}
				{/each}
			</div>
		</Card>
	</div>
</section>

<!-- Learning Outcomes Section -->
<section class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Learning Achievements</h2>
			<p class="text-xl opacity-90">
				What your child will accomplish in Foundation class
			</p>
		</div>
		
		<div class="grid md:grid-cols-2 gap-12 items-center">
			<div class="neumorphic-accent p-2 motion-preset-slide-right">
				<img src="/images/balloon.jpg" 
					 alt="Young children celebrating learning achievements with teacher guidance, showing progress charts and creative artwork displayed on classroom walls" 
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

<!-- Foundation Facilities Section -->
<section id="facilities" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Foundation Learning Spaces</h2>
			<p class="text-xl opacity-90">
				Safe, engaging environments designed specifically for early learners
			</p>
		</div>
		
		<div class="grid lg:grid-cols-3 gap-8">
			<!-- Classroom -->
			<Card class="neumorphic overflow-hidden motion-preset-slide-up group hover:neumorphic-accent transition-all duration-500 border-0"
				 style="animation-delay: 100ms;">
				<div class="relative overflow-hidden">
					<img src="/images/light.jpg" 
						 alt="Bright, colorful Foundation classroom with age-appropriate furniture, learning centers, and abundant natural light creating a warm learning environment" 
						 class="w-full h-48 object-cover group-hover:scale-110 transition-transform duration-500" />
					<div class="absolute inset-0 bg-gradient-to-t from-base-100/80 to-transparent"></div>
				</div>
				<div class="p-6">
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Foundation Classroom</h3>
					<p class="opacity-90">Thoughtfully designed spaces with child-sized furniture, learning centers, and hands-on materials to encourage exploration and discovery.</p>
				</div>
			</Card>

			<!-- Indoor Play Area -->
			<Card class="neumorphic overflow-hidden motion-preset-slide-up group hover:neumorphic-accent transition-all duration-500 border-0"
				 style="animation-delay: 200ms;">
				<div class="relative overflow-hidden">
					<img src="/images/beads.jpg" 
						 alt="Safe indoor play area with soft play equipment, building blocks, puzzles, and creative play materials for motor skill development" 
						 class="w-full h-48 object-cover group-hover:scale-110 transition-transform duration-500" />
					<div class="absolute inset-0 bg-gradient-to-t from-base-100/80 to-transparent"></div>
				</div>
				<div class="p-6">
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Indoor Play Space</h3>
					<p class="opacity-90">Safe, spacious areas for active play featuring age-appropriate equipment to develop gross motor skills and encourage physical activity.</p>
				</div>
			</Card>

			<!-- Reading Corner -->
			<Card class="neumorphic overflow-hidden motion-preset-slide-up group hover:neumorphic-accent transition-all duration-500 border-0"
				 style="animation-delay: 300ms;">
				<div class="relative overflow-hidden">
					<img src="/images/fire.jpg" 
						 alt="Cozy reading corner with soft cushions, colorful picture books, and warm lighting designed to foster early literacy and love of stories" 
						 class="w-full h-48 object-cover group-hover:scale-110 transition-transform duration-500" />
					<div class="absolute inset-0 bg-gradient-to-t from-base-100/80 to-transparent"></div>
				</div>
				<div class="p-6">
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">Story & Reading Corner</h3>
					<p class="opacity-90">Cozy spaces filled with picture books and comfortable seating to nurture early literacy skills and imagination through storytelling.</p>
				</div>
			</Card>
		</div>

		<!-- Additional Facilities Features -->
		<div class="mt-16">
			<Card class="neumorphic-accent p-8 motion-preset-slide-up border-0">
				<h3 class="text-2xl font-bold mb-6 text-primary text-center">Additional Features</h3>
				<div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 text-center">
					<div class="motion-preset-slide-up" style="animation-delay: 100ms;">
						<div class="text-3xl mb-2">🚻</div>
						<h4 class="font-semibold text-primary">Child-Safe Restrooms</h4>
						<p class="text-sm opacity-80">Age-appropriate facilities with safety features</p>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 200ms;">
						<div class="text-3xl mb-2">🍎</div>
						<h4 class="font-semibold text-primary">Snack Area</h4>
						<p class="text-sm opacity-80">Clean dining space for healthy snacks and meals</p>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 300ms;">
						<div class="text-3xl mb-2">💤</div>
						<h4 class="font-semibold text-primary">Rest Area</h4>
						<p class="text-sm opacity-80">Quiet space for rest and quiet activities</p>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 400ms;">
						<div class="text-3xl mb-2">🌳</div>
						<h4 class="font-semibold text-primary">Outdoor Access</h4>
						<p class="text-sm opacity-80">Direct access to safe outdoor play areas</p>
					</div>
				</div>
			</Card>
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section id="contact" class="py-20 px-4">
	<div class="container mx-auto max-w-4xl text-center">
		<Card class="neumorphic-accent p-12 motion-preset-slide-up border-0">
			<h2 class="text-3xl font-bold mb-6 text-glow">Ready to Begin Your Child's Journey?</h2>
			<p class="text-xl mb-8 opacity-90">
				Give your child the best start in their educational journey with our Foundation class. 
				Contact us today to schedule a visit and see our nurturing environment in action.
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

<style>
	:global(.timeline-item) {
		margin-bottom: 1.5rem;
	}
</style> 