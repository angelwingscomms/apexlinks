<script>
  export let type = 'primary'; // primary, cosmic, sunset, ethereal
  export let pill = false;
  export let size = 'default'; // sm, default, lg
  export let className = '';
  export let ripple = true;
  
  // Determine classes based on props
  const buttonClass = `btn-gradient ${type !== 'primary' ? `btn-gradient-${type}` : ''} ${pill ? 'btn-pill' : ''} ${size === 'sm' ? 'btn-sm' : size === 'lg' ? 'btn-lg' : ''} ${ripple ? 'ripple-container' : ''} ${className}`;
</script>

<button class={buttonClass} on:click>
  <slot />
</button>

<style>
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-weight: 500;
    transition: all 0.3s var(--animation-ease);
    position: relative;
    overflow: hidden;
    padding: 0.75rem 1.5rem;
    border-radius: 1.25rem;
    border: none;
    color: var(--color-lightest);
  }
  
  .btn-sm {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
  
  .btn-lg {
    padding: 1rem 2rem;
    font-size: 1.125rem;
  }
  
  button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 15px rgba(10, 132, 255, 0.4);
    filter: brightness(1.05);
  }
  
  button:active {
    transform: translateY(1px);
    box-shadow: 0 2px 5px rgba(10, 132, 255, 0.2);
  }
</style> 