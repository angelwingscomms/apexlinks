<script lang="ts">
	import { onMount } from 'svelte';
	
	export let selectedArea = '';
	
	const stemDetails = {
		'Computer Science': {
			description: 'Dive into the world of algorithms, artificial intelligence, and software development. Learn programming languages, build apps, and create the digital future.',
			skills: ['Programming', 'AI & Machine Learning', 'Web Development', 'Mobile Apps', 'Game Development', 'Cybersecurity'],
			projects: ['Build your own AI chatbot', 'Create a mobile game', 'Design a website', 'Develop a security system'],
			careers: ['Software Engineer', 'Data Scientist', 'AI Researcher', 'Game Developer', 'Cybersecurity Analyst'],
			color: 'from-blue-500 to-cyan-500',
			icon: '💻'
		},
		'Engineering': {
			description: 'Design, build, and innovate solutions to real-world problems. From bridges to robots, engineers shape our physical world.',
			skills: ['CAD Design', 'Robotics', 'Circuit Design', 'Mechanical Systems', 'Materials Science', '3D Printing'],
			projects: ['Build a robot', 'Design a bridge', 'Create a renewable energy system', 'Develop a prosthetic device'],
			careers: ['Mechanical Engineer', 'Electrical Engineer', 'Robotics Engineer', 'Aerospace Engineer', 'Civil Engineer'],
			color: 'from-green-500 to-emerald-500',
			icon: '⚙️'
		},
		'Mathematics': {
			description: 'Explore the language of the universe through numbers, patterns, and logical reasoning. Mathematics is the foundation of all sciences.',
			skills: ['Calculus', 'Statistics', 'Geometry', 'Algebra', 'Mathematical Modeling', 'Data Analysis'],
			projects: ['Model population growth', 'Analyze sports statistics', 'Create geometric art', 'Solve optimization problems'],
			careers: ['Data Analyst', 'Actuary', 'Research Mathematician', 'Financial Analyst', 'Statistician'],
			color: 'from-purple-500 to-violet-500',
			icon: '📐'
		},
		'Physics': {
			description: 'Uncover the fundamental laws that govern our universe, from quantum mechanics to astrophysics.',
			skills: ['Quantum Mechanics', 'Thermodynamics', 'Electromagnetism', 'Optics', 'Particle Physics', 'Astrophysics'],
			projects: ['Build a particle detector', 'Study black holes', 'Create laser experiments', 'Design solar panels'],
			careers: ['Research Physicist', 'Astrophysicist', 'Medical Physicist', 'Optical Engineer', 'Quantum Computing Researcher'],
			color: 'from-orange-500 to-red-500',
			icon: '🔬'
		},
		'Chemistry': {
			description: 'Transform matter at the molecular level and create new materials that will change the world.',
			skills: ['Organic Chemistry', 'Biochemistry', 'Materials Science', 'Analytical Chemistry', 'Green Chemistry', 'Nanotechnology'],
			projects: ['Synthesize new compounds', 'Create biodegradable plastics', 'Develop new medicines', 'Design smart materials'],
			careers: ['Research Chemist', 'Pharmaceutical Scientist', 'Materials Engineer', 'Environmental Chemist', 'Forensic Scientist'],
			color: 'from-pink-500 to-rose-500',
			icon: '🧪'
		},
		'Biology': {
			description: 'Explore the mysteries of life, from DNA to ecosystems, and contribute to medical breakthroughs.',
			skills: ['Genetics', 'Biotechnology', 'Ecology', 'Microbiology', 'Bioinformatics', 'Molecular Biology'],
			projects: ['Study genetic mutations', 'Create biological sensors', 'Analyze ecosystems', 'Develop gene therapies'],
			careers: ['Biomedical Researcher', 'Geneticist', 'Biotechnologist', 'Environmental Scientist', 'Bioinformatics Specialist'],
			color: 'from-teal-500 to-green-500',
			icon: '🧬'
		}
	};
	
	let mounted = false;
	let activeTab = 'skills';
	
	onMount(() => {
		mounted = true;
	});
	
	$: currentArea = stemDetails[selectedArea as keyof typeof stemDetails];
</script>

{#if selectedArea && currentArea}
	<div class="neumorphic-red p-8 motion-preset-fade-in">
		<div class="flex items-center mb-6">
			<span class="text-6xl mr-4 motion-preset-bounce">{currentArea.icon}</span>
			<div>
				<h3 class="text-3xl font-bold text-glow mb-2">{selectedArea}</h3>
				<div class="w-32 h-1 bg-gradient-to-r {currentArea.color} rounded-full motion-preset-expand"></div>
			</div>
		</div>
		
		<p class="text-lg mb-8 opacity-90 motion-preset-slide-up motion-delay-300">
			{currentArea.description}
		</p>
		
		<!-- Tab Navigation -->
		<div class="tabs tabs-boxed neumorphic mb-6 motion-preset-slide-up motion-delay-500">
			<button 
				class="tab {activeTab === 'skills' ? 'tab-active bg-primary text-primary-content' : ''}"
				onclick={() => activeTab = 'skills'}
			>
				Skills
			</button>
			<button 
				class="tab {activeTab === 'projects' ? 'tab-active bg-primary text-primary-content' : ''}"
				onclick={() => activeTab = 'projects'}
			>
				Projects
			</button>
			<button 
				class="tab {activeTab === 'careers' ? 'tab-active bg-primary text-primary-content' : ''}"
				onclick={() => activeTab = 'careers'}
			>
				Careers
			</button>
		</div>
		
		<!-- Tab Content -->
		<div class="motion-preset-fade-in">
			{#if activeTab === 'skills'}
				<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
					{#each currentArea.skills as skill, index}
						<div class="neumorphic-sm p-4 motion-preset-slide-up group hover:neumorphic-red transition-all duration-300"
							 style="animation-delay: {index * 100}ms;">
							<span class="text-primary group-hover:text-glow transition-all">✓</span>
							{skill}
						</div>
					{/each}
				</div>
			{:else if activeTab === 'projects'}
				<div class="grid md:grid-cols-2 gap-6">
					{#each currentArea.projects as project, index}
						<div class="neumorphic p-6 motion-preset-slide-up group hover:neumorphic-red transition-all duration-300 cursor-pointer"
							 style="animation-delay: {index * 150}ms;">
							<div class="flex items-center">
								<span class="text-2xl mr-3 group-hover:motion-preset-bounce">🚀</span>
								<span class="group-hover:text-glow transition-all">{project}</span>
							</div>
						</div>
					{/each}
				</div>
			{:else if activeTab === 'careers'}
				<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
					{#each currentArea.careers as career, index}
						<div class="neumorphic p-4 motion-preset-slide-up group hover:neumorphic-red transition-all duration-300 cursor-pointer"
							 style="animation-delay: {index * 100}ms;">
							<div class="flex items-center">
								<span class="text-primary mr-2 group-hover:text-glow transition-all">💼</span>
								<span class="group-hover:text-glow transition-all">{career}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
		
		<div class="mt-8 text-center">
			<button 
				class="btn btn-primary glow-red motion-preset-bounce"
				onclick={() => selectedArea = ''}
			>
				Explore Other Areas
			</button>
		</div>
	</div>
{/if} 