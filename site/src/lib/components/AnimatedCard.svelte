<script>
  import { cardEntryAnimation, imageHoverEffect } from '$lib/animations.js';
  import { onMount } from 'svelte';
  
  // Define props using regular Svelte syntax until Svelte 5 is fully supported
  export let title = '';
  export let description = '';
  export let imageUrl = '';
  export let altText = '';
  export let href = '';
  
  /** @type {HTMLElement} */
  let card;
  /** @type {HTMLImageElement} */
  let image;
  
  // Define state for animation control
  let isHovering = false;
  
  onMount(() => {
    // Run entry animation when component mounts
    if (card) cardEntryAnimation(card);
  });
  
  // Image hover handlers
  function handleMouseEnter() {
    isHovering = true;
    if (image) imageHoverEffect(image);
  }
  
  function handleMouseLeave() {
    isHovering = false;
    // Use revised function with reverse parameter
    if (image) imageHoverEffect(image, true);
  }
</script>

<a 
  {href}
  class="block glass transition-all" 
  bind:this={card}
>
  <div class="relative overflow-hidden rounded-2xl">
    <img 
      src={imageUrl} 
      alt={altText} 
      class="w-full h-48 object-cover transition-transform"
      bind:this={image}
      on:mouseenter={handleMouseEnter}
      on:mouseleave={handleMouseLeave}
    />
    
    <div class="p-4">
      <h3 class="text-lg font-semibold text-sky-900 mb-2">{title}</h3>
      <p class="text-sm text-sky-800">{description}</p>
    </div>
    
    <div 
      class="absolute inset-0 bg-gradient-to-t from-sky-100/50 to-transparent opacity-0 transition-opacity"
      class:opacity-70={isHovering}
    ></div>
  </div>
</a>

<style>
  @theme {
    --glass-opacity: 0.2;
    --glass-blur: 15px;
    --glass-border: 1px solid rgba(255, 255, 255, 0.2);
    --glass-shadow: 0 8px 32px rgba(14, 165, 233, 0.2);
  }
  
  @layer utilities {
    .glass {
      background: rgba(255, 255, 255, var(--glass-opacity));
      backdrop-filter: blur(var(--glass-blur));
      border: var(--glass-border);
      box-shadow: var(--glass-shadow);
      border-radius: 1.5rem;
      transition: all 0.3s ease;
    }
  }
</style> 