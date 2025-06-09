<script lang="ts">
  import { onMount } from 'svelte';
  import { createTextAnimation } from '$lib/animations.js';
  
  export let gradient = 'primary'; // primary, cosmic, sunset, ethereal
  export let animated = false;
  export let tag = 'span'; // h1, h2, h3, p, span, etc.
  export let className = '';
  
  let textElement: HTMLElement;
  
  onMount(() => {
    if (animated && textElement) {
      createTextAnimation(textElement);
    }
  });
</script>

<svelte:element 
  this={tag}
  class="text-gradient-{gradient} {className}"
  bind:this={textElement}
  data-animate={animated ? 'text' : undefined}
>
  <slot />
</svelte:element>

<style>
  :global(.text-gradient-primary) {
    background: var(--gradient-primary);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
  
  :global(.text-gradient-cosmic) {
    background: var(--gradient-cosmic);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
  
  :global(.text-gradient-sunset) {
    background: var(--gradient-sunset);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
  
  :global(.text-gradient-ethereal) {
    background: var(--gradient-ethereal);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
  }
</style> 