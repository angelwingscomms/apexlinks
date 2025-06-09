<script>
  import { pageTransition } from '$lib/animations.js';
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  
  /** @type {HTMLElement} */
  let container;
  let visible = false;
  
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
    // Add small delay for smoother entry
    setTimeout(() => {
      visible = true;
      if (container) {
        pageTransition(container);
      }
    }, 50);
  });
  
  onDestroy(() => {
    visible = false;
  });
</script>

{#if visible}
  <div 
    class="page-container" 
    bind:this={container} 
    use:animatePageTransition
    transition:fade={{ duration: 400, easing: cubicOut }}
  >
    <slot></slot>
  </div>
{:else}
  <div class="page-loading">
    <!-- Minimal loading indicator -->
    <div class="loading-spinner"></div>
  </div>
{/if}

<style>
  .page-container {
    width: 100%;
    min-height: 100%;
    display: flex;
    flex-direction: column;
    animation: slide-up 600ms cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }
  
  .page-loading {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    width: 100%;
  }
  
  .loading-spinner {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 2px solid rgba(10, 132, 255, 0.1);
    border-top-color: rgba(10, 132, 255, 0.8);
    animation: spin 800ms linear infinite;
  }
  
  @keyframes slide-up {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style> 