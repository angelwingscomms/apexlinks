<script lang="ts">
	import { onMount } from 'svelte';
	
	let activeStep = 0;
	let mounted = false;
	
	const journeySteps = [
		{
			grade: "Grades 6-8",
			title: "Foundation Phase",
			description: "Students discover their interests through diverse academic explorations, creative projects, and character-building activities.",
			activities: ["Academic Exploration", "Creative Arts Projects", "Leadership Development", "Community Service"],
			icon: "🌱",
			color: "from-blue-500 to-cyan-500"
		},
		{
			grade: "Grades 9-10",
			title: "Growth Phase",
			description: "Deep dive into chosen academic areas with specialized courses, mentorship programs, and real-world applications.",
			activities: ["Advanced Coursework", "Peer Mentorship", "Independent Projects", "Career Exploration"],
			icon: "🌿",
			color: "from-green-500 to-emerald-500"
		},
		{
			grade: "Grades 11-12",
			title: "Excellence Phase",
			description: "Students lead their own initiatives, participate in competitions, and prepare for university or career paths.",
			activities: ["Student Leadership", "Academic Competitions", "College Preparation", "Portfolio Development"],
			icon: "🌟",
			color: "from-purple-500 to-violet-500"
		},
		{
			grade: "Beyond",
			title: "Impact Phase",
			description: "Alumni continue to excel in their chosen fields, often returning as mentors and community leaders.",
			activities: ["University Success", "Career Achievement", "Community Leadership", "Alumni Mentoring"],
			icon: "🎓",
			color: "from-orange-500 to-red-500"
		}
	];
	
	onMount(() => {
		mounted = true;
		
		// Auto-advance timeline
		const interval = setInterval(() => {
			activeStep = (activeStep + 1) % journeySteps.length;
		}, 4000);
		
		return () => clearInterval(interval);
	});
	
	function selectStep(index: number) {
		activeStep = index;
	}
</script>

<section class="py-20 px-4" id="showcase">
	<div class="container mx-auto max-w-6xl">
		<div class="text-center mb-16 motion-preset-fade-in">
			<h2 class="text-4xl font-bold mb-6 text-glow">Student Journey</h2>
			<p class="text-xl opacity-90">
				Follow the path from discovery to achievement as our students grow into tomorrow's leaders
			</p>
		</div>
		
		<!-- Timeline Navigation -->
		<div class="relative mb-16">
			<!-- Timeline Line -->
			<div class="absolute top-1/2 left-0 right-0 h-1 bg-base-300 transform -translate-y-1/2"></div>
			<div class="absolute top-1/2 left-0 h-1 bg-gradient-to-r from-primary to-accent transform -translate-y-1/2 transition-all duration-1000"
				 style="width: {((activeStep + 1) / journeySteps.length) * 100}%"></div>
			
			<!-- Timeline Steps -->
			<div class="relative flex justify-between">
				{#each journeySteps as step, index}
					<button
						class="flex flex-col items-center group cursor-pointer motion-preset-slide-up"
						style="animation-delay: {index * 200}ms;"
						onclick={() => selectStep(index)}
					>
						<!-- Step Circle -->
						<div class="w-16 h-16 rounded-full flex items-center justify-center text-2xl transition-all duration-300 mb-4
									{index <= activeStep ? 'neumorphic-accent glow-blue' : 'neumorphic'} 
									group-hover:neumorphic-accent group-hover:glow-blue">
							<span class="motion-preset-bounce motion-delay-500">{step.icon}</span>
						</div>
						
						<!-- Step Label -->
						<div class="text-center">
							<div class="text-sm font-bold text-primary mb-1">{step.grade}</div>
							<div class="text-xs opacity-70 group-hover:text-glow transition-all">{step.title}</div>
						</div>
					</button>
				{/each}
			</div>
		</div>
		
		<!-- Active Step Details -->
		<div class="neumorphic-accent p-8 motion-preset-fade-in">
			<div class="grid lg:grid-cols-2 gap-8 items-center">
				<!-- Step Info -->
				<div class="motion-preset-slide-right">
					<div class="flex items-center mb-6">
						<span class="text-6xl mr-4 motion-preset-bounce">{journeySteps[activeStep].icon}</span>
						<div>
							<h3 class="text-3xl font-bold text-glow mb-2">{journeySteps[activeStep].title}</h3>
							<p class="text-primary text-lg">{journeySteps[activeStep].grade}</p>
							<div class="w-32 h-1 bg-gradient-to-r {journeySteps[activeStep].color} rounded-full mt-2 motion-preset-expand"></div>
						</div>
					</div>
					
					<p class="text-lg mb-6 opacity-90 motion-preset-slide-up motion-delay-300">
						{journeySteps[activeStep].description}
					</p>
					
					<!-- Progress Indicator -->
					<div class="mb-6">
						<div class="flex justify-between text-sm mb-2">
							<span>Journey Progress</span>
							<span>{Math.round(((activeStep + 1) / journeySteps.length) * 100)}%</span>
						</div>
						<div class="w-full h-3 bg-base-300 rounded-full overflow-hidden">
							<div class="h-full bg-gradient-to-r {journeySteps[activeStep].color} rounded-full transition-all duration-1000 motion-preset-expand"
								 style="width: {((activeStep + 1) / journeySteps.length) * 100}%"></div>
						</div>
					</div>
				</div>
				
				<!-- Activities -->
				<div class="motion-preset-slide-left">
					<h4 class="text-2xl font-bold mb-6 text-primary motion-preset-typewriter">Key Activities</h4>
					<div class="grid gap-4">
						{#each journeySteps[activeStep].activities as activity, index}
							<div class="neumorphic p-4 motion-preset-slide-up group hover:neumorphic-accent transition-all duration-300"
								 style="animation-delay: {index * 150}ms;">
								<div class="flex items-center">
									<span class="text-primary mr-3 group-hover:text-glow transition-all">✓</span>
									<span class="group-hover:text-glow transition-all">{activity}</span>
								</div>
							</div>
						{/each}
					</div>
					
					<!-- Call to Action -->
					<div class="mt-8 text-center">
						<button class="btn btn-primary glow-blue motion-preset-bounce">
							Learn More About This Phase
						</button>
					</div>
				</div>
			</div>
		</div>
		
		<!-- Navigation Dots -->
		<div class="flex justify-center mt-8 space-x-4">
			{#each journeySteps as step, index}
				<button
					class="w-3 h-3 rounded-full transition-all duration-300 {index === activeStep ? 'bg-primary glow-blue' : 'bg-base-300 hover:bg-primary/50'}"
					onclick={() => selectStep(index)}
					aria-label="View {step.title} phase for {step.grade}"
				></button>
			{/each}
		</div>
	</div>
</section> 