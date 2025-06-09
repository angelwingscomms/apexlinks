<script>
  export let gradient = 'primary'; // primary, cosmic, sunset, ethereal
  export let hoverable = true;
  export let className = '';
</script>

<div 
  class="gradient-card gradient-card-{gradient} {hoverable ? 'hoverable' : ''} {className}"
  data-animate="fade"
>
  <slot />
</div>

<style>
  .gradient-card {
    border-radius: 1.5rem;
    padding: 1.5rem;
    box-shadow: var(--glass-shadow);
    transition: all 0.4s var(--animation-spring);
    color: white;
  }
  
  .gradient-card-primary {
    background: var(--gradient-primary);
  }
  
  .gradient-card-cosmic {
    background: var(--gradient-cosmic);
  }
  
  .gradient-card-sunset {
    background: var(--gradient-sunset);
  }
  
  .gradient-card-ethereal {
    background: var(--gradient-ethereal);
  }
  
  .hoverable {
    cursor: pointer;
  }
  
  .hoverable:hover {
    transform: translateY(-5px);
    box-shadow: var(--glass-shadow-lg);
  }
</style> 