<script>
  export let gradientBorder = false;
  export let gradientType = 'primary'; // primary, cosmic, sunset, ethereal
  export let hoverable = true;
  export let size = 'default'; // sm, default, lg
  export let className = '';
</script>

<div
  class="glass-card {hoverable ? 'hoverable' : ''} {size === 'sm' ? 'glass-card-sm' : size === 'lg' ? 'glass-card-lg' : ''} {gradientBorder ? `gradient-border gradient-border-${gradientType}` : ''} {className}"
  data-animate="fade"
>
  <slot />
</div>

<style>
  .glass-card {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    border-radius: 1.5rem;
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
    padding: 1.5rem;
    transition: all 0.4s var(--animation-spring);
    position: relative;
    z-index: 1;
  }

  .glass-card-sm {
    padding: 1rem;
    border-radius: 1.25rem;
  }

  .glass-card-lg {
    padding: 2rem;
    border-radius: 2rem;
  }
  
  .hoverable {
    cursor: pointer;
  }
  
  .hoverable:hover {
    transform: translateY(-5px);
    box-shadow: var(--glass-shadow-lg);
  }
</style> 