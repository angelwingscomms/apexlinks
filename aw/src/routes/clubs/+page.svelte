<script lang="ts">
	import { Card } from 'flowbite-svelte';
	
	const clubs = [
		{
			title: "Robotics Club",
			description: "Build and program robots while learning cutting-edge engineering and technology skills",
			icon: "🤖",
			color: "from-blue-400 to-indigo-500",
			link: "/clubs/robotics",
			image: "A modern robotics lab with students assembling and programming colorful robots, with screens showing code and robotic components scattered on tables"
		},
		{
			title: "Chess Club", 
			description: "Develop strategic thinking and problem-solving skills through the classic game of chess",
			icon: "♟️",
			color: "from-slate-400 to-gray-600",
			link: "/clubs/chess",
			image: "Young students focused on chess games in a quiet library setting with wooden chess boards and pieces"
		},
		{
			title: "Coding Club",
			description: "Learn programming languages and create amazing software applications",
			icon: "💻",
			color: "from-green-400 to-emerald-500", 
			link: "/clubs/coding",
			image: "Children coding on computers with colorful code displayed on screens in a modern computer lab"
		},
		{
			title: "AI Development Club",
			description: "Making software with AI and exploring the frontiers of artificial intelligence",
			icon: "🧠",
			color: "from-purple-400 to-violet-500",
			link: "/clubs/ai-development",
			image: "Students using AI tools on laptops with neural network diagrams and futuristic AI interfaces visible on screens"
		},
		{
			title: "Drama Club",
			description: "Express creativity through theater, acting, and dramatic performances",
			icon: "🎭",
			color: "from-pink-400 to-rose-500",
			link: "/clubs/drama",
			image: "Young actors in colorful costumes rehearsing on a stage with dramatic lighting and theater props"
		},
		{
			title: "Student Government",
			description: "Develop leadership skills and represent student voices in school decisions",
			icon: "🏛️",
			color: "from-amber-400 to-yellow-500",
			link: "/clubs/student-government",
			image: "Students in a meeting room discussing policies with a whiteboard showing leadership concepts and voting charts"
		},
		{
			title: "Environmental Club",
			description: "Protect our planet through sustainability projects and environmental awareness",
			icon: "🌱",
			color: "from-emerald-400 to-green-500",
			link: "/clubs/environmental",
			image: "Students planting trees and working in a school garden with recycling bins and solar panels visible"
		},
		{
			title: "Debate Team",
			description: "Sharpen communication skills and critical thinking through competitive debates",
			icon: "🗣️",
			color: "from-orange-400 to-red-500",
			link: "/clubs/debate",
			image: "Young debaters at podiums in a formal debate setting with judges and an engaged audience"
		},
		{
			title: "Art Society",
			description: "Explore visual arts, painting, drawing, and creative expression",
			icon: "🎨",
			color: "from-teal-400 to-cyan-500",
			link: "/clubs/art",
			image: "Children painting colorful artwork in an art studio with easels, brushes, and vibrant paintings on walls"
		},
		{
			title: "Music Ensemble",
			description: "Create beautiful music together through instrumental and vocal performances",
			icon: "🎵",
			color: "from-indigo-400 to-blue-500",
			link: "/clubs/music",
			image: "Young musicians playing various instruments in an orchestra setting with music stands and conductor"
		},
		{
			title: "Community Service",
			description: "Make a positive impact through volunteer work and community outreach",
			icon: "🤝",
			color: "from-red-400 to-pink-500",
			link: "/clubs/community-service",
			image: "Students volunteering at a community center, helping elderly people and organizing charity drives"
		},
		{
			title: "Photography Club",
			description: "Capture the world through creative photography and digital imaging",
			icon: "📸",
			color: "from-violet-400 to-purple-500",
			link: "/clubs/photography",
			image: "Young photographers with cameras taking pictures outdoors with beautiful landscapes and artistic compositions"
		},
		{
			title: "Literary Magazine",
			description: "Express yourself through creative writing, poetry, and literary publications",
			icon: "📖",
			color: "from-sky-400 to-blue-500",
			link: "/clubs/literary",
			image: "Students writing in journals and discussing literature in a cozy library corner with books and vintage writing desks"
		}
	];
</script>

<div class="min-h-screen">
	<!-- Hero Section -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-6xl">
			<div class="text-center mb-16 motion-preset-fade-in">
				<h1 class="text-5xl font-bold mb-6 text-glow">
					Discover Your <span class="text-primary">Passion</span>
				</h1>
				<p class="text-xl opacity-90 max-w-3xl mx-auto">
					Explore our diverse range of clubs and extracurricular activities designed to nurture your talents, 
					develop new skills, and create lasting friendships. From cutting-edge technology to creative arts, 
					there's something for every young mind at AngelWings.
				</p>
			</div>
		</div>
	</section>

	<!-- Clubs Grid -->
	<section class="py-20 px-4 bg-base-200/50">
		<div class="container mx-auto max-w-7xl">
			<div class="grid desktop:grid-cols-2 lg:grid-cols-3 gap-8">
				{#each clubs as club, index}
					<Card 
						href={club.link} 
						class="card-custom hover:neumorphic-accent transition-all duration-500 motion-preset-slide-up group cursor-pointer border-0 overflow-hidden"
						style="animation-delay: {index * 150}ms;"
					>
						<!-- Image Placeholder -->
						<div class="relative h-48 bg-gradient-to-br {club.color} overflow-hidden">
							<div class="absolute inset-0 bg-gradient-to-t from-base-100/60 to-transparent"></div>
							<div class="absolute inset-0 flex items-center justify-center">
								<div class="text-8xl opacity-30 motion-preset-bounce motion-delay-500">{club.icon}</div>
							</div>
							<!-- Placeholder for AI-generated image -->
							<img 
								src="/api/placeholder/400/300" 
								alt={club.image}
								class="w-full h-full object-cover opacity-0 group-hover:opacity-20 transition-opacity duration-500"
							/>
							<div class="absolute bottom-4 left-4 right-4">
								<h3 class="text-xl font-bold text-white text-glow group-hover:motion-preset-bounce">{club.title}</h3>
							</div>
						</div>
						
						<!-- Content -->
						<div class="p-6">
							<div class="flex items-center mb-4">
								<div class="text-3xl mr-3 motion-preset-bounce motion-delay-700">{club.icon}</div>
								<div class="flex-1">
									<div class="w-full h-2 bg-base-300 rounded-full overflow-hidden">
										<div class="h-full bg-gradient-to-r {club.color} rounded-full motion-preset-expand motion-delay-1000"></div>
									</div>
								</div>
							</div>
							<p class="opacity-90 leading-relaxed">{club.description}</p>
							
							<!-- Action Button -->
							<div class="mt-6">
								<div class="btn-outline-custom border-glow group-hover:btn-primary-custom group-hover:glow-blue transition-all duration-300 text-center py-2">
									Learn More
								</div>
							</div>
						</div>
					</Card>
				{/each}
			</div>
		</div>
	</section>

	<!-- Call to Action -->
	<section class="py-20 px-4">
		<div class="container mx-auto max-w-4xl text-center">
			<div class="neumorphic-accent p-12 motion-preset-slide-up">
				<h2 class="text-3xl font-bold mb-6 text-glow">Ready to Join a Club?</h2>
				<p class="text-xl opacity-90 mb-8">
					Contact our student activities coordinator to learn more about joining any of these exciting clubs!
				</p>
				<div class="flex flex-col sm:flex-row gap-4 justify-center">
					<a href="tel:+2348036513400" class="btn-primary-custom glow-blue motion-preset-bounce">
						<span class="mr-2">📞</span>
						Call Us
					</a>
					<a href="mailto:angelwingseducation@gmail.com" class="btn-outline-custom border-glow motion-preset-bounce motion-delay-200">
						<span class="mr-2">✉️</span>
						Email Us
					</a>
				</div>
			</div>
		</div>
	</section>
</div> 