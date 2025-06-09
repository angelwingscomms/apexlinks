<script>
  import { cardEntryAnimation, imageHoverEffect } from '$lib/animations.js';
  import { onMount } from 'svelte';
  
  // Define props using regular Svelte syntax until Svelte 5 is fully supported
  export let title = '';
  export let description = '';
  export let imageUrl = '';
  export let altText = '';
  export let href = '';
  export let badge = '';
  
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
  class="block neumorphic overflow-hidden transition-all duration-300 hover:translate-y-[-5px]" 
  bind:this={card}
  on:mouseenter={handleMouseEnter}
  on:mouseleave={handleMouseLeave}
>
  <div class="relative overflow-hidden rounded-2xl">
    <img 
      src={imageUrl} 
      alt={altText} 
      class="w-full h-52 object-cover transition-transform duration-500"
      bind:this={image}
      style="transform: scale({isHovering ? 1.05 : 1});"
    />
    
    {#if badge}
      <div class="absolute top-3 right-3 bg-[#0a84ff] text-white text-xs px-2 py-1 rounded-full">
        {badge}
      </div>
    {/if}
    
    <div class="p-6">
      <h3 class="text-lg font-semibold text-[#1d1d1f] mb-3">{title}</h3>
      <p class="text-[#86868b] text-sm leading-relaxed">{description}</p>
      
      <div class="mt-4 flex items-center text-[#0a84ff] text-sm font-medium">
        <span>Learn more</span>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-1"><path d="m9 6 6 6-6 6"></path></svg>
      </div>
    </div>
  </div>
</a>

<style>
  .neumorphic {
    background: var(--color-lighter, #e8e8ed);
    border-radius: 1.25rem;
    box-shadow: 
      10px 10px 20px rgba(0, 0, 0, 0.05),
      -10px -10px 20px rgba(255, 255, 255, 0.8);
    transition: all 0.3s cubic-bezier(0.42, 0, 0.58, 1);
  }
  
  .neumorphic:hover {
    box-shadow: 
      15px 15px 30px rgba(0, 0, 0, 0.07),
      -15px -15px 30px rgba(255, 255, 255, 0.9);
  }
</style> 