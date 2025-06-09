<script lang="ts">
  import { onMount } from 'svelte';
  import { initializeAnimations } from '$lib/animations.js';
  import '$lib/styles/ripple.css';

  onMount(() => {
    // Initialize all animations
    initializeAnimations();
    
    // Add ripple effect to all buttons
    const buttons = document.querySelectorAll('button:not(.no-ripple), .btn-primary, .btn-secondary, .btn-gradient');
    buttons.forEach(button => {
      button.classList.add('ripple-container');
      
      button.addEventListener('click', (e: Event) => {
        const mouseEvent = e as MouseEvent;
        const ripple = document.createElement('span');
        ripple.classList.add('ripple-effect');
        
        const rect = button.getBoundingClientRect();
        const size = Math.max(rect.width, rect.height) * 2;
        
        ripple.style.width = `${size}px`;
        ripple.style.height = `${size}px`;
        ripple.style.left = `${mouseEvent.clientX - rect.left - size / 2}px`;
        ripple.style.top = `${mouseEvent.clientY - rect.top - size / 2}px`;
        
        button.appendChild(ripple);
        
        setTimeout(() => {
          ripple.remove();
        }, 800);
      });
    });
  });
</script>

<!-- This component doesn't render anything, it just initializes animations --> 