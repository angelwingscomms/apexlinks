<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Card, Badge } from 'flowbite-svelte';
	import ParticleBackground from '$lib/ParticleBackground.svelte';
	import CreativeShowcase from '$lib/CreativeShowcase.svelte';
	import StudentJourney from '$lib/StudentJourney.svelte';
	import FloatingActionButton from '$lib/FloatingActionButton.svelte';
	
	let mounted = false;
	
	onMount(() => {
		mounted = true;
	});
	
	const schoolClasses = [
		{
			title: "Foundation",
			description: "Building essential early learning skills through play-based discovery",
			icon: "🌱",
			color: "from-emerald-400 to-green-500",
			link: "/classes/foundation"
		},
		{
			title: "Pre-Nursery",
			description: "Nurturing curiosity and social skills in our youngest learners",
			icon: "🐣",
			color: "from-yellow-400 to-amber-500",
			link: "/classes/pre-nursery"
		},
		{
			title: "Nursery 1",
			description: "Introduction to structured learning through creative activities",
			icon: "🎈",
			color: "from-pink-400 to-rose-500",
			link: "/classes/nursery-1"
		},
		{
			title: "Nursery 2",
			description: "Developing fundamental skills in preparation for formal education",
			icon: "🎯",
			color: "from-purple-400 to-violet-500",
			link: "/classes/nursery-2"
		},
		{
			title: "Grade 1",
			description: "Beginning formal education with reading, writing, and numeracy",
			icon: "📖",
			color: "from-blue-400 to-indigo-500",
			link: "/classes/grade-1"
		},
		{
			title: "Grade 2",
			description: "Building stronger foundations in core academic subjects",
			icon: "✏️",
			color: "from-teal-400 to-cyan-500",
			link: "/classes/grade-2"
		},
		{
			title: "Grade 3",
			description: "Expanding knowledge and critical thinking abilities",
			icon: "🔍",
			color: "from-orange-400 to-red-500",
			link: "/classes/grade-3"
		},
		{
			title: "Grade 4",
			description: "Developing independence and advanced problem-solving skills",
			icon: "🧠",
			color: "from-indigo-400 to-purple-500",
			link: "/classes/grade-4"
		},
		{
			title: "Grade 5",
			description: "Preparing for secondary education with comprehensive learning",
			icon: "🎓",
			color: "from-sky-400 to-blue-500",
			link: "/classes/grade-5"
		}
	];

	const academicPrograms = [
		{
			title: "Liberal Arts",
			description: "Discover your creative voice through literature and history",
			icon: "📚",
			color: "from-amber-400 to-yellow-500"
		},
		{
			title: "Fine Arts",
			description: "Express yourself through visual arts, music, and performance",
			icon: "🎨",
			color: "from-rose-400 to-pink-500"
		},
		{
			title: "Languages",
			description: "Connect with the world through multilingual communication",
			icon: "🌍",
			color: "from-emerald-400 to-green-500"
		},
		{
			title: "Social Sciences",
			description: "Understand society, culture, and human behavior",
			icon: "🤝",
			color: "from-sky-400 to-blue-500"
		},
		{
			title: "Life Skills",
			description: "Develop practical skills for personal and professional success",
			icon: "🌱",
			color: "from-violet-400 to-purple-500"
		},
		{
			title: "Athletics",
			description: "Build character and teamwork through sports and physical education",
			icon: "⚽",
			color: "from-orange-400 to-red-500"
		}
	];

	const extracurriculars = [
		"Robotics",
		"Chess",
		"Coding",
		"Making Software with AI",
		"Drama Club",
		"Student Government",
		"Environmental Club",
		"Debate Team",
		"Art Society",
		"Music Ensemble",
		"Community Service",
		"Photography Club",
		"Literary Magazine"
	];

	const facilities = [
		{
			title: "Creative Studios",
			description: "Modern spaces equipped for artistic expression and innovation",
			image: "/images/light.jpg"
		},
		{
			title: "Learning Commons",
			description: "Collaborative environments for knowledge sharing and discovery",
			image: "/images/fire.jpg"
		},
		{
			title: "Wellness Center",
			description: "Supporting holistic student development and well-being",
			image: "/images/beads.jpg"
		}
	];
</script>

<!-- Main Content Container -->
<div class="transition-all duration-300">

<!-- Particle Background -->
<ParticleBackground />

<!-- Hero Section -->
<section class="min-h-screen relative overflow-hidden flex items-center justify-center">
	<div class="text-center relative z-10">
		<div class="max-w-4xl motion-preset-fade-in motion-delay-500">
			<h1 class="text-6xl font-bold mb-8 text-glow">
				Welcome to <span class="text-primary">AngelWings</span><br />
				Comprehensive College
			</h1>
			<p class="text-xl mb-8 opacity-90 motion-preset-slide-up motion-delay-700">
				Where young minds take flight and potential soars. Discover exceptional primary and secondary education 
				that nurtures creativity, character, and academic excellence from foundation through grade 5.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center motion-preset-slide-up motion-delay-1000">
				<Button href="#classes" class="btn-primary-custom glow-blue motion-preset-bounce motion-delay-1200" size="lg">
					Explore Classes
				</Button>
				<Button href="#about" class="btn-outline-custom border-glow motion-preset-bounce motion-delay-1400" size="lg">
					Learn More
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

<!-- About Section -->
<section id="about" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Nurturing Tomorrow's Leaders</h2>
			<p class="text-xl opacity-90 max-w-3xl mx-auto">
				At AngelWings Comprehensive College, we believe every student has unique gifts waiting to be discovered. 
				Our mission is to provide a supportive, inspiring environment where young learners can explore their interests, 
				develop their talents, and build strong foundations for their educational journey.
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 gap-12 items-center">
			<Card class="neumorphic p-8 motion-preset-slide-right border-0">
				<h3 class="text-2xl font-bold mb-4 text-primary">Our Philosophy</h3>
				<p class="mb-4">
					We embrace a holistic approach to primary and secondary education that values academic excellence, creative expression, 
					and character development. Our dedicated faculty creates personalized learning experiences that 
					help each young student thrive in their own unique way.
				</p>
				<ul class="space-y-2">
					<li class="flex items-center"><span class="text-primary mr-2">✓</span> Age-appropriate learning paths</li>
					<li class="flex items-center"><span class="text-primary mr-2">✓</span> Creative expression opportunities</li>
					<li class="flex items-center"><span class="text-primary mr-2">✓</span> Character and values development</li>
					<li class="flex items-center"><span class="text-primary mr-2">✓</span> Community engagement</li>
				</ul>
			</Card>
			<div class="neumorphic-accent p-2 motion-preset-slide-left">
				<img src="/images/balloon.jpg" alt="Creative Learning Environment - Young students engaged in hands-on learning activities with colorful educational materials" class="w-full h-80 object-cover rounded-xl" />
			</div>
		</div>
	</div>
</section>

<!-- School Classes Section -->
<section id="classes" class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Our School Classes</h2>
			<p class="text-xl opacity-90">
				From foundation to grade 5, we provide age-appropriate education for every stage of development
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each schoolClasses as schoolClass, index}
				<Card href={schoolClass.link} class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group cursor-pointer border-0"
					 style="animation-delay: {index * 200}ms;">
					<div class="text-4xl mb-4 motion-preset-bounce motion-delay-500">{schoolClass.icon}</div>
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{schoolClass.title}</h3>
					<p class="opacity-90">{schoolClass.description}</p>
					<div class="mt-4">
						<div class="w-full h-2 bg-base-300 rounded-full overflow-hidden">
							<div class="h-full bg-gradient-to-r {schoolClass.color} rounded-full motion-preset-expand motion-delay-1000"></div>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Academic Programs Section -->
<section id="programs" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Academic Programs</h2>
			<p class="text-xl opacity-90">
				Discover your passion through our comprehensive range of academic programs
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each academicPrograms as program, index}
				<Card class="card-custom hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group cursor-pointer border-0"
					 style="animation-delay: {index * 200}ms;">
					<div class="text-4xl mb-4 motion-preset-bounce motion-delay-500">{program.icon}</div>
					<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{program.title}</h3>
					<p class="opacity-90">{program.description}</p>
					<div class="mt-4">
						<div class="w-full h-2 bg-base-300 rounded-full overflow-hidden">
							<div class="h-full bg-gradient-to-r {program.color} rounded-full motion-preset-expand motion-delay-1000"></div>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Extracurriculars Section -->
<section class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Beyond the Classroom</h2>
			<p class="text-xl opacity-90 mb-8">
				Explore your interests and develop new skills through our diverse extracurricular activities
			</p>
		</div>
		
		<Card class="neumorphic-accent p-8 motion-preset-slide-up border-0">
			<div class="grid desktop:grid-cols-2 lg:grid-cols-4 gap-4">
				{#each extracurriculars as activity, index}
					<Badge class="neumorphic-sm p-4 text-center hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up group border-0"
						 style="animation-delay: {index * 100}ms;">
						<span class="text-primary group-hover:text-glow transition-all font-medium">{activity}</span>
					</Badge>
				{/each}
			</div>
			<div class="text-center mt-8">
				<p class="text-sm opacity-70">
					From traditional activities to modern technology like robotics and AI-powered software development, 
					we're preparing students for the future while nurturing their current interests.
				</p>
			</div>
		</Card>
	</div>
</section>

<!-- Facilities Section -->
<section id="facilities" class="py-20 px-4">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Learning Environment</h2>
			<p class="text-xl opacity-90">
				Our beautiful campus provides the perfect environment for young minds to learn and grow
			</p>
		</div>
		
		<div class="grid lg:grid-cols-3 gap-8">
			{#each facilities as facility, index}
				<Card class="neumorphic overflow-hidden motion-preset-slide-up group hover:neumorphic-accent transition-all duration-500 border-0"
					 style="animation-delay: {index * 300}ms;">
					<div class="relative overflow-hidden">
						<img src={facility.image} alt={facility.title} 
							 class="w-full h-48 object-cover group-hover:scale-110 transition-transform duration-500" />
						<div class="absolute inset-0 bg-gradient-to-t from-base-100/80 to-transparent"></div>
					</div>
					<div class="p-6">
						<h3 class="text-xl font-bold mb-3 text-primary group-hover:text-glow transition-all">{facility.title}</h3>
						<p class="opacity-90">{facility.description}</p>
					</div>
				</Card>
			{/each}
		</div>
	</div>
</section>

<!-- Creative Showcase Section -->
<CreativeShowcase />

<!-- Student Journey Section -->
<StudentJourney />

<!-- Statistics Section -->
<section class="py-20 px-4 bg-base-200/50">
	<div class="container mx-auto max-w-4xl">
		<div class="grid desktop:grid-cols-4 gap-8 text-center">
			<Card class="neumorphic p-6 motion-preset-bounce border-0">
				<div class="text-3xl font-bold text-primary mb-2 motion-preset-typewriter">300+</div>
				<div class="opacity-90">Students</div>
			</Card>
			<Card class="neumorphic p-6 motion-preset-bounce motion-delay-200 border-0">
				<div class="text-3xl font-bold text-primary mb-2 motion-preset-typewriter motion-delay-500">30+</div>
				<div class="opacity-90">Expert Teachers</div>
			</Card>
			<Card class="neumorphic p-6 motion-preset-bounce motion-delay-400 border-0">
				<div class="text-3xl font-bold text-primary mb-2 motion-preset-typewriter motion-delay-1000">9</div>
				<div class="opacity-90">Grade Levels</div>
			</Card>
			<Card class="neumorphic p-6 motion-preset-bounce motion-delay-600 border-0">
				<div class="text-3xl font-bold text-primary mb-2 motion-preset-typewriter motion-delay-1500">100%</div>
				<div class="opacity-90">Success Rate</div>
			</Card>
		</div>
	</div>
</section>

<!-- Contact Section -->
<section id="contact" class="py-20 px-4">
	<div class="container mx-auto max-w-4xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Join Our School Community</h2>
			<p class="text-xl opacity-90">
				Ready to begin your child's educational journey with us? Get in touch today!
			</p>
		</div>
		
		<div class="grid desktop:grid-cols-2 gap-12">
			<div class="neumorphic p-8 motion-preset-slide-right">
				<h3 class="text-2xl font-bold mb-6 text-primary">Contact Information</h3>
				<div class="space-y-6">
					<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
						<div class="text-2xl text-primary mr-4 floating-element">📍</div>
						<div>
							<div class="font-medium text-primary">Location</div>
							<div class="opacity-80">Comprehensive Educational Center</div>
						</div>
					</div>
					<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
						<div class="text-2xl text-primary mr-4 floating-element" style="animation-delay: 0.5s;">📞</div>
						<div>
							<div class="font-medium text-primary">Phone</div>
							<div class="opacity-80">+2348036513400</div>
						</div>
					</div>
					<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
						<div class="text-2xl text-primary mr-4 floating-element" style="animation-delay: 1s;">✉️</div>
						<div>
							<div class="font-medium text-primary">Email</div>
							<div class="opacity-80">angelwingseducation@gmail.com</div>
						</div>
					</div>
					<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
						<div class="text-2xl text-primary mr-4 floating-element" style="animation-delay: 1.5s;">🕒</div>
						<div>
							<div class="font-medium text-primary">School Hours</div>
							<div class="opacity-80">Monday - Friday: 7:30 AM - 4:00 PM</div>
						</div>
					</div>
				</div>
			</div>
			
			<div class="neumorphic p-8 motion-preset-slide-left">
				<h3 class="text-2xl font-bold mb-6 text-primary">Get In Touch</h3>
				<form class="space-y-6">
					<div class="motion-preset-slide-up" style="animation-delay: 0.2s;">
						<label class="block text-sm font-medium text-primary mb-2">Parent/Guardian Name</label>
						<input 
							type="text" 
							placeholder="Enter your full name" 
							class="input-custom w-full focus:border-primary focus:ring-primary/30 placeholder:opacity-60" 
						/>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 0.4s;">
						<label class="block text-sm font-medium text-primary mb-2">Email Address</label>
						<input 
							type="email" 
							placeholder="your.email@example.com" 
							class="input-custom w-full focus:border-primary focus:ring-primary/30 placeholder:opacity-60" 
						/>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 0.6s;">
						<label class="block text-sm font-medium text-primary mb-2">Phone Number</label>
						<input 
							type="tel" 
							placeholder="+234 xxx xxx xxxx" 
							class="input-custom w-full focus:border-primary focus:ring-primary/30 placeholder:opacity-60" 
						/>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 0.8s;">
						<label class="block text-sm font-medium text-primary mb-2">Interested Class Level</label>
						<select class="input-custom w-full focus:border-primary focus:ring-primary/30">
							<option value="" disabled selected>Select a class level</option>
							<option value="foundation">Foundation</option>
							<option value="pre-nursery">Pre-Nursery</option>
							<option value="nursery-1">Nursery 1</option>
							<option value="nursery-2">Nursery 2</option>
							<option value="grade-1">Grade 1</option>
							<option value="grade-2">Grade 2</option>
							<option value="grade-3">Grade 3</option>
							<option value="grade-4">Grade 4</option>
							<option value="grade-5">Grade 5</option>
						</select>
					</div>
					<div class="motion-preset-slide-up" style="animation-delay: 1s;">
						<label class="block text-sm font-medium text-primary mb-2">Your Message</label>
						<textarea 
							placeholder="Tell us about your child's educational needs and any questions you have..." 
							class="input-custom w-full h-32 resize-none focus:border-primary focus:ring-primary/30 placeholder:opacity-60"
						></textarea>
					</div>
					<button 
						type="submit" 
						class="btn-primary-custom w-full glow-blue motion-preset-bounce motion-delay-1200 text-lg py-4"
					>
						<span class="mr-2">📨</span>
						Send Message
					</button>
					<p class="text-xs opacity-70 text-center">
						We'll get back to you within 24 hours during business days
					</p>
				</form>
			</div>
		</div>
	</div>
</section>

<!-- Footer -->
<footer class="footer footer-center p-10 neumorphic-accent text-base-content">
	<div class="grid grid-flow-col gap-4">
		<a href="#about" class="link link-hover">About</a>
		<a href="#classes" class="link link-hover">Classes</a>
		<a href="#programs" class="link link-hover">Programs</a>
		<a href="#facilities" class="link link-hover">Facilities</a>
		<a href="#contact" class="link link-hover">Contact</a>
	</div>
	<div>
		<p class="text-glow">Copyright © 2024 - AngelWings Comprehensive College</p>
		<p class="opacity-70">Where young minds take flight through exceptional education</p>
	</div>
</footer>

<!-- Floating Action Button -->
<FloatingActionButton />

</div>
