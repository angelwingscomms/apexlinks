<script lang="ts">
	import { onMount } from 'svelte';
	
	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D;
	let particles: Particle[] = [];
	let animationId: number;
	
	class Particle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		size: number;
		opacity: number;
		color: string;
		
		constructor(width: number, height: number) {
			this.x = Math.random() * width;
			this.y = Math.random() * height;
			this.vx = (Math.random() - 0.5) * 0.5;
			this.vy = (Math.random() - 0.5) * 0.5;
			this.size = Math.random() * 3 + 1;
			this.opacity = Math.random() * 0.5 + 0.1;
			this.color = Math.random() > 0.7 ? '#ef4444' : '#ffffff';
		}
		
		update(width: number, height: number) {
			this.x += this.vx;
			this.y += this.vy;
			
			if (this.x < 0 || this.x > width) this.vx *= -1;
			if (this.y < 0 || this.y > height) this.vy *= -1;
			
			// Keep particles within bounds
			this.x = Math.max(0, Math.min(width, this.x));
			this.y = Math.max(0, Math.min(height, this.y));
		}
		
		draw(ctx: CanvasRenderingContext2D) {
			ctx.save();
			ctx.globalAlpha = this.opacity;
			ctx.fillStyle = this.color;
			ctx.beginPath();
			ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
			ctx.fill();
			
			// Add glow effect for red particles
			if (this.color === '#ef4444') {
				ctx.shadowColor = '#ef4444';
				ctx.shadowBlur = 10;
				ctx.fill();
			}
			
			ctx.restore();
		}
	}
	
	function initParticles() {
		particles = [];
		const particleCount = Math.min(50, Math.floor((canvas.width * canvas.height) / 15000));
		
		for (let i = 0; i < particleCount; i++) {
			particles.push(new Particle(canvas.width, canvas.height));
		}
	}
	
	function drawConnections() {
		for (let i = 0; i < particles.length; i++) {
			for (let j = i + 1; j < particles.length; j++) {
				const dx = particles[i].x - particles[j].x;
				const dy = particles[i].y - particles[j].y;
				const distance = Math.sqrt(dx * dx + dy * dy);
				
				if (distance < 100) {
					ctx.save();
					ctx.globalAlpha = (100 - distance) / 100 * 0.1;
					ctx.strokeStyle = '#ef4444';
					ctx.lineWidth = 1;
					ctx.beginPath();
					ctx.moveTo(particles[i].x, particles[i].y);
					ctx.lineTo(particles[j].x, particles[j].y);
					ctx.stroke();
					ctx.restore();
				}
			}
		}
	}
	
	function animate() {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		
		// Update and draw particles
		particles.forEach(particle => {
			particle.update(canvas.width, canvas.height);
			particle.draw(ctx);
		});
		
		// Draw connections
		drawConnections();
		
		animationId = requestAnimationFrame(animate);
	}
	
	function resizeCanvas() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		initParticles();
	}
	
	onMount(() => {
		ctx = canvas.getContext('2d')!;
		resizeCanvas();
		animate();
		
		window.addEventListener('resize', resizeCanvas);
		
		return () => {
			cancelAnimationFrame(animationId);
			window.removeEventListener('resize', resizeCanvas);
		};
	});
</script>

<canvas 
	bind:this={canvas}
	class="fixed inset-0 pointer-events-none z-0"
	style="opacity: 0.3;"
></canvas> 