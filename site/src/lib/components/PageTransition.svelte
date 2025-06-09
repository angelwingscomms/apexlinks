<script>
  import { pageTransition } from '$lib/animations.js';
  import { onMount } from 'svelte';
  
  /** @type {HTMLElement} */
  let container;
  
  // Create a custom action for the animation
  /**
   * @param {HTMLElement} node
   */
  function animatePageTransition(node) {
    // Run the animation when the component mounts
    const animation = pageTransition(node);
    
    return {
      destroy() {
        // Clean up if needed
        if (animation && typeof animation.pause === 'function') {
          animation.pause();
        }
      }
    };
  }
  
  onMount(() => {
    if (container) {
      pageTransition(container);
    }
  });
</script>

<!-- 
  Using the new @attach directive from Svelte 5
  This will run the animation when the slot content is mounted
-->
<div class="page-container" bind:this={container} use:animatePageTransition>
  <slot></slot>
</div>

<style>
  .page-container {
    width: 100%;
    min-height: 100%;
    display: flex;
    flex-direction: column;
  }
</style> 