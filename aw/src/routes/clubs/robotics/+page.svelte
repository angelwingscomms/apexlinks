<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	
	const activities = [
		"Building and programming autonomous robots",
		"Learning Arduino and Raspberry Pi programming", 
		"Participating in robotics competitions",
		"3D printing robot components",
		"Working with sensors and actuators",
		"Team-based engineering challenges"
	];
	
	const skills = [
		{ name: "Programming", level: 90, icon: "💻" },
		{ name: "Engineering", level: 85, icon: "⚙️" },
		{ name: "Problem Solving", level: 95, icon: "🧩" },
		{ name: "Teamwork", level: 88, icon: "🤝" },
		{ name: "Creativity", level: 92, icon: "💡" }
	];
	
	const projects = [
		{
			title: "Line Following Robot",
			description: "Build a robot that can follow a black line autonomously using sensors",
			difficulty: "Beginner",
			duration: "4 weeks"
		},
		{
			title: "Obstacle Avoidance Bot", 
			description: "Create a robot that navigates around obstacles using ultrasonic sensors",
			difficulty: "Intermediate", 
			duration: "6 weeks"
		},
		{
			title: "Robotic Arm Controller",
			description: "Design and build a programmable robotic arm for precise movements",
			difficulty: "Advanced",
			duration: "10 weeks"
		}
	];
</script>

<div class="min-h-screen">
	<!-- Hero Section -->
	<section class="py-20 px-4 bg-gradient-to-br from-secondary/20 to-primary/10">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<div class="text-8xl mb-6 floating-element">🤖</div>
				<h1 class="text-5xl font-bold mb-6 text-glow">
					<span class="text-secondary">Robotics</span> Club
				</h1>
				<p class="text-xl opacity-90 max-w-3xl mx-auto">
					Dive into the exciting world of robotics where engineering meets creativity. 
					Build, program, and compete with cutting-edge robots while developing critical STEM skills 
					that will prepare you for the future of technology.
				</p>
			</div>
			
			<!-- Image Placeholder -->
			<div class="max-w-4xl mx-auto motion-preset-slide-up">
				<div class="relative neumorphic-accent p-4 overflow-hidden">
					<div class="w-full h-80 bg-gradient-to-br from-secondary/30 to-primary/20 rounded-xl flex items-center justify-center relative overflow-hidden">
						<div class="absolute inset-0 bg-gradient-to-t from-base-100/40 to-transparent"></div>
						<div class="text-center z-10">
							<div class="text-9xl mb-4 floating-element text-secondary">🤖</div>
							<div class="text-2xl font-bold text-secondary mb-2 text-glow">Robotics Lab</div>
							<div class="opacity-80">
								Students working with advanced robotics equipment
							</div>
						</div>
						<!-- Floating elements -->
						<div class="absolute top-8 left-8 w-20 h-20 neumorphic-accent rounded-full opacity-40 floating-element"></div>
						<div class="absolute bottom-12 right-12 w-16 h-16 neumorphic rounded-full opacity-50 drifting-element" style="animation-delay: 2s;"></div>
					</div>
					<img 
						src="/api/placeholder/800/400" 
						alt="A modern robotics lab with students assembling and programming colorful robots, with screens showing code and robotic components scattered on tables"
						class="w-full h-80 object-cover rounded-xl opacity-0"
					/>
				</div>
			</div>
		</div>
	</section>

	<!-- What We Do Section -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					What We <span class="text-secondary">Build</span>
				</h2>
				<p class="text-xl opacity-90">
					Explore the amazing projects and activities that make our robotics club special
				</p>
			</div>
			
			<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
				{#each activities as activity, index}
					<Card class="neumorphic p-6 motion-preset-slide-up hover:neumorphic-accent transition-all duration-300 border-0"
						 style="animation-delay: {index * 200}ms;">
						<div class="flex items-center">
							<div class="text-3xl text-secondary mr-4 floating-element">⚙️</div>
							<p class="font-medium text-secondary">{activity}</p>
						</div>
					</Card>
				{/each}
			</div>
		</div>
	</section>

	<!-- Skills Development -->
	<section class="py-20 px-4 bg-base-200/50">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					<span class="text-secondary">Skills</span> You'll Develop
				</h2>
			</div>
			
			<div class="grid desktop:grid-cols-2 gap-12 items-center">
				<div class="motion-preset-slide-right">
					{#each skills as skill, index}
						<div class="mb-6 neumorphic p-6 hover:neumorphic-accent transition-all duration-300 motion-preset-slide-up"
							 style="animation-delay: {index * 200}ms;">
							<div class="flex items-center justify-between mb-3">
								<div class="flex items-center">
									<span class="text-2xl mr-3 floating-element">{skill.icon}</span>
									<span class="font-bold text-secondary">{skill.name}</span>
								</div>
								<span class="text-sm opacity-70">{skill.level}%</span>
							</div>
							<div class="w-full bg-base-300 rounded-full h-3 overflow-hidden">
								<div class="h-full bg-gradient-to-r from-secondary to-primary rounded-full motion-preset-expand motion-delay-1000"
									 style="width: {skill.level}%; animation-delay: {index * 300 + 1000}ms;"></div>
							</div>
						</div>
					{/each}
				</div>
				
				<div class="motion-preset-slide-left">
					<Card class="neumorphic-accent p-8 border-0">
						<h3 class="text-2xl font-bold text-secondary mb-6 text-glow">Why Join Robotics?</h3>
						<ul class="space-y-4">
							<li class="flex items-start">
								<span class="text-secondary mr-3 mt-1">🚀</span>
								<span>Hands-on experience with cutting-edge technology</span>
							</li>
							<li class="flex items-start">
								<span class="text-secondary mr-3 mt-1">🧠</span>
								<span>Develop critical thinking and problem-solving skills</span>
							</li>
							<li class="flex items-start">
								<span class="text-secondary mr-3 mt-1">🏆</span>
								<span>Participate in exciting robotics competitions</span>
							</li>
							<li class="flex items-start">
								<span class="text-secondary mr-3 mt-1">👥</span>
								<span>Work in teams to tackle complex engineering challenges</span>
							</li>
							<li class="flex items-start">
								<span class="text-secondary mr-3 mt-1">💼</span>
								<span>Prepare for future careers in STEM fields</span>
							</li>
						</ul>
					</Card>
				</div>
			</div>
		</div>
	</section>

	<!-- Featured Projects -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					Featured <span class="text-secondary">Projects</span>
				</h2>
				<p class="text-xl opacity-90">
					Start with beginner projects and progress to advanced robotics challenges
				</p>
			</div>
			
			<div class="grid lg:grid-cols-3 gap-8">
				{#each projects as project, index}
					<Card class="neumorphic motion-preset-slide-up hover:neumorphic-accent transition-all duration-500 border-0"
						 style="animation-delay: {index * 300}ms;">
						<div class="p-6">
							<div class="flex items-center justify-between mb-4">
								<h3 class="text-xl font-bold text-secondary">{project.title}</h3>
								<Badge class="neumorphic-sm text-xs px-3 py-1 text-secondary border-0">
									{project.difficulty}
								</Badge>
							</div>
							<p class="opacity-90 mb-4">{project.description}</p>
							<div class="flex items-center justify-between text-sm opacity-70">
								<span class="flex items-center">
									<span class="text-secondary mr-1">⏱️</span>
									{project.duration}
								</span>
								<div class="btn-outline-custom border-glow text-xs px-4 py-2 hover:btn-primary-custom hover:glow-blue transition-all duration-300">
									Learn More
								</div>
							</div>
						</div>
					</Card>
				{/each}
			</div>
		</div>
	</section>

	<!-- Meeting Info & Contact -->
	<section class="py-20 px-4 bg-base-200/50">
		<div class="container mx-auto max-w-4xl">
			<div class="grid desktop:grid-cols-2 gap-12">
				<Card class="neumorphic p-8 motion-preset-slide-right border-0">
					<h3 class="text-2xl font-bold text-secondary mb-6 text-glow">Meeting Information</h3>
					<div class="space-y-4">
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element">📅</span>
							<div>
								<div class="font-medium text-secondary">Schedule</div>
								<div class="opacity-80">Tuesdays & Thursdays, 3:30 PM - 5:00 PM</div>
							</div>
						</div>
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element" style="animation-delay: 0.5s;">🏫</span>
							<div>
								<div class="font-medium text-secondary">Location</div>
								<div class="opacity-80">STEM Lab - Building A, Room 205</div>
							</div>
						</div>
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element" style="animation-delay: 1s;">👥</span>
							<div>
								<div class="font-medium text-secondary">Age Group</div>
								<div class="opacity-80">Grades 3-5 (Advanced projects available)</div>
							</div>
						</div>
					</div>
				</Card>
				
				<Card class="neumorphic-accent p-8 motion-preset-slide-left border-0">
					<h3 class="text-2xl font-bold text-secondary mb-6 text-glow">Ready to Join?</h3>
					<p class="mb-6 opacity-90">
						Contact us to learn more about joining the Robotics Club and start your journey into the exciting world of robotics!
					</p>
					<div class="space-y-4">
						<a href="tel:+2348036513400" class="btn-primary-custom w-full glow-blue motion-preset-bounce flex items-center justify-center">
							<span class="mr-2">📞</span>
							Call School Office
						</a>
						<a href="mailto:angelwingseducation@gmail.com" class="btn-outline-custom border-glow w-full motion-preset-bounce motion-delay-200 flex items-center justify-center">
							<span class="mr-2">✉️</span>
							Send Email
						</a>
						<a href="/clubs" class="btn-outline-custom border-glow w-full motion-preset-bounce motion-delay-400 flex items-center justify-center">
							<span class="mr-2">🔙</span>
							Back to All Clubs
						</a>
					</div>
				</Card>
			</div>
		</div>
	</section>
</div> 