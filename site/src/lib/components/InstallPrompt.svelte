<script lang="ts">
  import { onMount } from 'svelte';
  
  let deferredPrompt: any = null;
  let showInstallButton = $state(false);
  let showInstallPrompt = $state(false);
  let isIOS = $state(false);
  
  onMount(() => {
    // Check if it's iOS
    isIOS = /iPad|iPhone|iPod/.test(navigator.userAgent) && !(window as any).MSStream;
    
    // Listen for the beforeinstallprompt event
    window.addEventListener('beforeinstallprompt', (e) => {
      // Prevent Chrome 67 and earlier from automatically showing the prompt
      e.preventDefault();
      
      // Stash the event so it can be triggered later
      deferredPrompt = e;
      
      // Show the install button
      showInstallButton = true;
    });
    
    // Listen for app installed event
    window.addEventListener('appinstalled', () => {
      // Log app installed event to analytics
      console.log('PWA was installed');
      
      // Hide install button and prompt
      showInstallButton = false;
      showInstallPrompt = false;
      
      // Clear the deferredPrompt
      deferredPrompt = null;
    });
  });
  
  // Handle install button click
  function showInstallInstructions() {
    showInstallPrompt = true;
  }
  
  // Hide install prompt
  function hideInstallPrompt() {
    showInstallPrompt = false;
  }
  
  // Install the PWA
  async function installPWA() {
    if (!deferredPrompt) return;
    
    // Show the install prompt
    deferredPrompt.prompt();
    
    // Wait for the user to respond to the prompt
    const choiceResult = await deferredPrompt.userChoice;
    
    // User responded to the prompt
    if (choiceResult.outcome === 'accepted') {
      console.log('User accepted the install prompt');
    } else {
      console.log('User dismissed the install prompt');
    }
    
    // Clear the deferredPrompt
    deferredPrompt = null;
    
    // Hide the install button
    showInstallButton = false;
    showInstallPrompt = false;
  }
</script>

{#if showInstallButton}
  <button 
    class="fixed bottom-4 left-4 z-40 glass-button px-4 py-2 rounded-full shadow-lg flex items-center space-x-2"
    on:click={showInstallInstructions}
    aria-label="Install app"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
    </svg>
    <span>Install App</span>
  </button>
{/if}

{#if showInstallPrompt}
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="card-glass w-full max-w-md relative">
      <button 
        class="absolute top-3 right-3 text-gray-400 hover:text-gray-600"
        on:click={hideInstallPrompt}
        aria-label="Close"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
      
      <h2 class="text-xl font-semibold dreamy-text mb-4">Install ApexLinks</h2>
      
      {#if isIOS}
        <div class="space-y-4">
          <p class="text-gray-600">To install this app on iOS:</p>
          <ol class="list-decimal list-inside space-y-2 text-gray-600">
            <li>Tap the share button <span class="inline-block"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 inline" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
            </svg></span></li>
            <li>Scroll down and tap "Add to Home Screen"</li>
            <li>Tap "Add" in the top-right corner</li>
          </ol>
          <button 
            class="btn-primary w-full mt-4"
            on:click={hideInstallPrompt}
          >
            Got it
          </button>
        </div>
      {:else if deferredPrompt}
        <div class="space-y-4">
          <p class="text-gray-600">Install this app on your device for a better experience. It won't take up much space and you can access it from your home screen.</p>
          <div class="flex space-x-3 mt-4">
            <button 
              class="glass-button flex-1 px-4 py-2"
              on:click={hideInstallPrompt}
            >
              Not now
            </button>
            <button 
              class="btn-primary flex-1"
              on:click={installPWA}
            >
              Install
            </button>
          </div>
        </div>
      {:else}
        <div class="space-y-4">
          <p class="text-gray-600">To install this app on your device, use your browser's install feature.</p>
          <button 
            class="btn-primary w-full mt-4"
            on:click={hideInstallPrompt}
          >
            Got it
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if} 