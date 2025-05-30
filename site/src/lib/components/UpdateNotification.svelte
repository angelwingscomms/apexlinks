<script lang="ts">
  import { onMount } from 'svelte';
  
  let updateAvailable = $state(false);
  let dismissible = $state(true);
  let version = $state('');
  
  onMount(() => {
    // Listen for messages from service worker
    navigator.serviceWorker.addEventListener('message', (event) => {
      if (event.data && event.data.type === 'APP_UPDATED') {
        updateAvailable = true;
        version = event.data.version || '';
      }
    });
  });
  
  function dismissNotification() {
    updateAvailable = false;
  }
  
  function refreshApp() {
    // Tell service worker to skip waiting
    if (navigator.serviceWorker.controller) {
      navigator.serviceWorker.controller.postMessage({ type: 'SKIP_WAITING' });
    }
    
    // Reload the page to activate the new service worker
    window.location.reload();
  }
</script>

{#if updateAvailable}
  <div 
    class="fixed bottom-4 right-4 left-4 md:left-auto md:w-96 z-50 glass p-4 rounded-lg shadow-lg"
    style="animation: slideUp 0.3s ease-out forwards;"
  >
    <div class="flex items-start">
      <div class="flex-1">
        <h3 class="text-md font-semibold dreamy-text mb-1">App Update Available</h3>
        <p class="text-sm text-gray-600 mb-3">
          A new version of ApexLinks is available. Refresh to update.
        </p>
        <div class="flex gap-2">
          <button 
            class="glass-button px-3 py-1.5 text-sm text-gray-600"
            on:click={refreshApp}
          >
            Update Now
          </button>
          
          {#if dismissible}
            <button 
              class="glass-sm px-3 py-1.5 text-sm text-gray-500"
              on:click={dismissNotification}
            >
              Later
            </button>
          {/if}
        </div>
      </div>
      
      <button 
        class="ml-2 text-gray-400 hover:text-gray-600"
        on:click={dismissNotification}
        aria-label="Dismiss"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>
  </div>
{/if}

<style>
  @keyframes slideUp {
    from {
      transform: translateY(100%);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
</style> 