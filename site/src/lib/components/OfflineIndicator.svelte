<script lang="ts">
  import { onMount } from 'svelte';
  
  let isOffline = $state(false);
  
  onMount(() => {
    // Set initial state
    isOffline = !navigator.onLine;
    
    // Add event listeners for online/offline events
    window.addEventListener('online', () => {
      isOffline = false;
    });
    
    window.addEventListener('offline', () => {
      isOffline = true;
    });
  });
</script>

{#if isOffline}
  <div 
    class="fixed top-20 left-0 right-0 mx-auto w-fit glass bg-red-50 py-2 px-4 rounded-full z-50 shadow-lg"
    style="animation: fadeIn 0.3s ease-out forwards;"
  >
    <div class="flex items-center space-x-2">
      <span class="w-2.5 h-2.5 bg-red-500 rounded-full animate-pulse"></span>
      <span class="text-sm font-medium text-red-600">You're offline</span>
    </div>
  </div>
{/if}

<style>
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style> 