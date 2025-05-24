<script lang="ts">
	import { onMount } from 'svelte';
	
	let currentProject = 0;
	let autoSlide = true;
	
	const projects = [
		{
			title: "Community Garden Initiative",
			student: "Sarah Chen, Grade 11",
			description: "Led a school-wide project to create sustainable gardens, teaching students about environmental responsibility and healthy living.",
			image: "/images/light.jpg",
			category: "Environmental",
			achievements: ["1st Place - Community Impact Award", "Featured in Local News"],
			skills: ["Leadership", "Environmental Science", "Project Management", "Community Engagement"]
		},
		{
			title: "Digital Art Exhibition",
			student: "Marcus Rodriguez, Grade 12",
			description: "Curated and organized a virtual art gallery showcasing student artwork, reaching over 1,000 viewers worldwide.",
			image: "/images/beads.jpg",
			category: "Fine Arts",
			achievements: ["Best Creative Project", "Arts Council Recognition"],
			skills: ["Digital Arts", "Event Planning", "Graphic Design", "Social Media"]
		},
		{
			title: "Peer Tutoring Program",
			student: "Aisha Patel, Grade 10",
			description: "Developed a comprehensive tutoring system that improved academic performance across all grade levels.",
			image: "/images/fire.jpg",
			category: "Academic",
			achievements: ["Outstanding Service Award", "Student Leadership Prize"],
			skills: ["Teaching", "Communication", "Organization", "Mentorship"]
		},
		{
			title: "Cultural Heritage Festival",
			student: "David Kim, Grade 11",
			description: "Organized a multicultural celebration that brought together diverse communities and promoted cultural understanding.",
			image: "/images/balloon.jpg",
			category: "Social Sciences",
			achievements: ["Cultural Ambassador Award", "Unity in Diversity Prize"],
			skills: ["Event Management", "Cultural Research", "Public Speaking", "Collaboration"]
		}
	];
	
	onMount(() => {
		const interval = setInterval(() => {
			if (autoSlide) {
				currentProject = (currentProject + 1) % projects.length;
			}
		}, 5000);
		
		return () => clearInterval(interval);
	});
	
	function nextProject() {
		autoSlide = false;
		currentProject = (currentProject + 1) % projects.length;
		setTimeout(() => autoSlide = true, 10000);
	}
	
	function prevProject() {
		autoSlide = false;
		currentProject = (currentProject - 1 + projects.length) % projects.length;
		setTimeout(() => autoSlide = true, 10000);
	}
	
	function selectProject(index: number) {
		autoSlide = false;
		currentProject = index;
		setTimeout(() => autoSlide = true, 10000);
	}
</script>

<section class="py-20 px-4 bg-base-200/30">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Student Achievement Showcase</h2>
			<p class="text-xl opacity-90">
				Discover the incredible projects our students create when passion meets opportunity
			</p>
		</div>
		
		<div class="relative neumorphic-accent overflow-hidden motion-preset-slide-up">
			<!-- Main Project Display -->
			<div class="grid lg:grid-cols-2 gap-8 p-8">
				<!-- Project Image -->
				<div class="relative overflow-hidden rounded-xl motion-preset-slide-right">
					<img 
						src={projects[currentProject].image} 
						alt={projects[currentProject].title}
						class="w-full h-80 object-cover transition-transform duration-500 hover:scale-105"
					/>
					<div class="absolute top-4 left-4">
						<span class="badge badge-primary text-xs font-bold px-3 py-2 glow-blue">
							{projects[currentProject].category}
						</span>
					</div>
				</div>
				
				<!-- Project Details -->
				<div class="flex flex-col justify-center motion-preset-slide-left">
					<h3 class="text-3xl font-bold mb-4 text-glow motion-preset-typewriter">
						{projects[currentProject].title}
					</h3>
					
					<p class="text-primary mb-4 motion-preset-fade-in motion-delay-300">
						By {projects[currentProject].student}
					</p>
					
					<p class="text-lg mb-6 opacity-90 motion-preset-slide-up motion-delay-500">
						{projects[currentProject].description}
					</p>
					
					<!-- Achievements -->
					<div class="mb-6 motion-preset-slide-up motion-delay-700">
						<h4 class="text-lg font-semibold mb-3 text-primary">🏆 Achievements</h4>
						<div class="space-y-2">
							{#each projects[currentProject].achievements as achievement}
								<div class="flex items-center">
									<span class="text-primary mr-2">✓</span>
									<span class="text-sm">{achievement}</span>
								</div>
							{/each}
						</div>
					</div>
					
					<!-- Skills -->
					<div class="motion-preset-slide-up motion-delay-900">
						<h4 class="text-lg font-semibold mb-3 text-primary">🌟 Skills Developed</h4>
						<div class="flex flex-wrap gap-2">
							{#each projects[currentProject].skills as skill}
								<span class="badge badge-outline border-primary text-primary hover:bg-primary hover:text-primary-content transition-all">
									{skill}
								</span>
							{/each}
						</div>
					</div>
				</div>
			</div>
			
			<!-- Navigation Controls -->
			<div class="absolute top-1/2 left-4 transform -translate-y-1/2">
				<button 
					class="btn btn-circle btn-primary glow-blue motion-preset-bounce"
					onclick={prevProject}
				>
					❮
				</button>
			</div>
			
			<div class="absolute top-1/2 right-4 transform -translate-y-1/2">
				<button 
					class="btn btn-circle btn-primary glow-blue motion-preset-bounce"
					onclick={nextProject}
				>
					❯
				</button>
			</div>
		</div>
		
		<!-- Project Indicators -->
		<div class="flex justify-center mt-8 space-x-4">
			{#each projects as project, index}
				<button
					class="w-4 h-4 rounded-full transition-all duration-300 {index === currentProject ? 'bg-primary glow-blue' : 'bg-base-300 hover:bg-primary/50'}"
					onclick={() => selectProject(index)}
				></button>
			{/each}
		</div>
		
		<!-- Project Grid Preview -->
		<div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 mt-16">
			{#each projects as project, index}
				<div 
					class="neumorphic p-4 cursor-pointer transition-all duration-300 hover:neumorphic-accent motion-preset-slide-up group {index === currentProject ? 'neumorphic-accent' : ''}"
					style="animation-delay: {index * 200}ms;"
					onclick={() => selectProject(index)}
				>
					<img 
						src={project.image} 
						alt={project.title}
						class="w-full h-32 object-cover rounded-lg mb-3 group-hover:scale-105 transition-transform"
					/>
					<h4 class="font-bold text-sm mb-1 group-hover:text-glow transition-all">{project.title}</h4>
					<p class="text-xs opacity-70">{project.student}</p>
					<span class="text-xs text-primary">{project.category}</span>
				</div>
			{/each}
		</div>
	</div>
</section> 