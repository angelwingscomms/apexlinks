<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	
	const benefits = [
		"Strategic thinking development",
		"Pattern recognition skills",
		"Problem-solving abilities", 
		"Concentration improvement",
		"Memory enhancement",
		"Patience and discipline"
	];
	
	const tournaments = [
		{
			title: "Monthly School Championship",
			description: "Compete against fellow students in our monthly chess tournament",
			frequency: "Monthly",
			level: "All Levels"
		},
		{
			title: "Inter-School Chess League", 
			description: "Represent AngelWings in regional chess competitions",
			frequency: "Quarterly", 
			level: "Advanced"
		},
		{
			title: "Chess Puzzle Challenge",
			description: "Solve tactical puzzles and improve your chess vision",
			frequency: "Weekly",
			level: "Beginner to Advanced"
		}
	];
</script>

<div class="min-h-screen">
	<!-- Hero Section -->
	<section class="py-20 px-4 bg-gradient-to-br from-secondary/20 to-slate-600/10">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<div class="text-8xl mb-6 floating-element">♟️</div>
				<h1 class="text-5xl font-bold mb-6 text-glow">
					<span class="text-secondary">Chess</span> Club
				</h1>
				<p class="text-xl opacity-90 max-w-3xl mx-auto">
					Master the ancient game of strategy and develop critical thinking skills that will benefit you 
					in academics and life. Join our chess community where every move matters and every game teaches valuable lessons.
				</p>
			</div>
			
			<!-- Image Placeholder -->
			<div class="max-w-4xl mx-auto motion-preset-slide-up">
				<div class="relative neumorphic-accent p-4 overflow-hidden">
					<div class="w-full h-80 bg-gradient-to-br from-secondary/30 to-slate-600/20 rounded-xl flex items-center justify-center relative overflow-hidden">
						<div class="absolute inset-0 bg-gradient-to-t from-base-100/40 to-transparent"></div>
						<div class="text-center z-10">
							<div class="text-9xl mb-4 floating-element text-secondary">♟️</div>
							<div class="text-2xl font-bold text-secondary mb-2 text-glow">Chess Learning Environment</div>
							<div class="opacity-80">
								Students focused on strategic gameplay
							</div>
						</div>
						<!-- Chess piece floating elements -->
						<div class="absolute top-8 left-8 w-20 h-20 neumorphic-accent rounded-full opacity-40 floating-element flex items-center justify-center text-2xl">♔</div>
						<div class="absolute bottom-12 right-12 w-16 h-16 neumorphic rounded-full opacity-50 drifting-element flex items-center justify-center text-xl" style="animation-delay: 2s;">♛</div>
						<div class="absolute top-1/2 left-1/4 w-12 h-12 neumorphic-accent rounded-full opacity-35 floating-element flex items-center justify-center text-lg" style="animation-delay: 4s;">♖</div>
					</div>
					<img 
						src="/api/placeholder/800/400" 
						alt="Young students focused on chess games in a quiet library setting with wooden chess boards and pieces"
						class="w-full h-80 object-cover rounded-xl opacity-0"
					/>
				</div>
			</div>
		</div>
	</section>

	<!-- Benefits Section -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					Why Play <span class="text-secondary">Chess</span>?
				</h2>
				<p class="text-xl opacity-90">
					Chess is more than a game - it's a powerful tool for developing crucial life skills
				</p>
			</div>
			
			<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
				{#each benefits as benefit, index}
					<Card class="neumorphic p-6 motion-preset-slide-up hover:neumorphic-accent transition-all duration-300 border-0"
						 style="animation-delay: {index * 200}ms;">
						<div class="flex items-center">
							<div class="text-3xl text-secondary mr-4 floating-element">🧠</div>
							<p class="font-medium text-secondary">{benefit}</p>
						</div>
					</Card>
				{/each}
			</div>
		</div>
	</section>

	<!-- Learning Progression -->
	<section class="py-20 px-4 bg-base-200/50">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					Your <span class="text-secondary">Chess Journey</span>
				</h2>
			</div>
			
			<div class="grid lg:grid-cols-4 gap-8">
				<Card class="neumorphic p-6 motion-preset-slide-up border-0">
					<div class="text-center">
						<div class="text-4xl mb-4 text-secondary floating-element">♙</div>
						<h3 class="text-lg font-bold text-secondary mb-3">Beginner</h3>
						<p class="text-sm opacity-90">Learn piece movements, basic rules, and simple tactics</p>
					</div>
				</Card>
				
				<Card class="neumorphic p-6 motion-preset-slide-up border-0" style="animation-delay: 200ms;">
					<div class="text-center">
						<div class="text-4xl mb-4 text-secondary floating-element" style="animation-delay: 0.5s;">♘</div>
						<h3 class="text-lg font-bold text-secondary mb-3">Intermediate</h3>
						<p class="text-sm opacity-90">Master opening principles, tactical combinations, and endgames</p>
					</div>
				</Card>
				
				<Card class="neumorphic p-6 motion-preset-slide-up border-0" style="animation-delay: 400ms;">
					<div class="text-center">
						<div class="text-4xl mb-4 text-secondary floating-element" style="animation-delay: 1s;">♗</div>
						<h3 class="text-lg font-bold text-secondary mb-3">Advanced</h3>
						<p class="text-sm opacity-90">Develop positional understanding and complex strategies</p>
					</div>
				</Card>
				
				<Card class="neumorphic-accent p-6 motion-preset-slide-up border-0" style="animation-delay: 600ms;">
					<div class="text-center">
						<div class="text-4xl mb-4 text-secondary floating-element" style="animation-delay: 1.5s;">♕</div>
						<h3 class="text-lg font-bold text-secondary mb-3">Expert</h3>
						<p class="text-sm opacity-90">Compete in tournaments and mentor newer players</p>
					</div>
				</Card>
			</div>
		</div>
	</section>

	<!-- Tournaments & Events -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h2 class="text-4xl font-bold mb-6 text-glow">
					Tournaments & <span class="text-secondary">Events</span>
				</h2>
				<p class="text-xl opacity-90">
					Test your skills and compete with fellow chess enthusiasts
				</p>
			</div>
			
			<div class="grid lg:grid-cols-3 gap-8">
				{#each tournaments as tournament, index}
					<Card class="neumorphic motion-preset-slide-up hover:neumorphic-accent transition-all duration-500 border-0"
						 style="animation-delay: {index * 300}ms;">
						<div class="p-6">
							<div class="flex items-center justify-between mb-4">
								<h3 class="text-xl font-bold text-secondary">{tournament.title}</h3>
								<Badge class="neumorphic-sm text-xs px-3 py-1 text-secondary border-0">
									{tournament.level}
								</Badge>
							</div>
							<p class="opacity-90 mb-4">{tournament.description}</p>
							<div class="flex items-center justify-between text-sm opacity-70">
								<span class="flex items-center">
									<span class="text-secondary mr-1">📅</span>
									{tournament.frequency}
								</span>
								<div class="btn-outline-custom border-glow text-xs px-4 py-2 hover:btn-primary-custom hover:glow-blue transition-all duration-300">
									Join Event
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
					<h3 class="text-2xl font-bold text-secondary mb-6 text-glow">Club Information</h3>
					<div class="space-y-4">
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element">📅</span>
							<div>
								<div class="font-medium text-secondary">Schedule</div>
								<div class="opacity-80">Mondays & Wednesdays, 3:30 PM - 4:30 PM</div>
							</div>
						</div>
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element" style="animation-delay: 0.5s;">🏫</span>
							<div>
								<div class="font-medium text-secondary">Location</div>
								<div class="opacity-80">Library Chess Corner - Building B</div>
							</div>
						</div>
						<div class="flex items-center neumorphic-sm p-4 hover:neumorphic-accent transition-all duration-300">
							<span class="text-2xl text-secondary mr-4 floating-element" style="animation-delay: 1s;">👥</span>
							<div>
								<div class="font-medium text-secondary">Age Group</div>
								<div class="opacity-80">All grades welcome (Foundation to Grade 5)</div>
							</div>
						</div>
					</div>
				</Card>
				
				<Card class="neumorphic-accent p-8 motion-preset-slide-left border-0">
					<h3 class="text-2xl font-bold text-secondary mb-6 text-glow">Join the Chess Family</h3>
					<p class="mb-6 opacity-90">
						Whether you're a complete beginner or an experienced player, our chess club welcomes all skill levels!
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