<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { userStore } from '$lib/stores/userStore';
  
  // State variables
  let loading = $state(true);
  let error = $state<string | null>(null);
  let success = $state(false);
  
  onMount(async () => {
    // Get auth code and state from URL
    const code = $page.url.searchParams.get('code');
    const state = $page.url.searchParams.get('state');
    
    if (!code || !state) {
      error = 'Invalid authentication response. Missing required parameters.';
      loading = false;
      return;
    }
    
    try {
      const result = await userStore.handleAuthCallback(code, state);
      
      if (result) {
        success = true;
        setTimeout(() => {
          goto('/profile');
        }, 2000);
      } else {
        throw new Error('Authentication failed');
      }
    } catch (err) {
      console.error('Error handling auth callback:', err);
      error = 'Authentication failed. Please try again.';
    } finally {
      loading = false;
    }
  });
</script>

<div class="min-h-[60vh] flex items-center justify-center">
  <div class="card-neumorphic w-full max-w-md text-center py-12 px-6">
    {#if loading}
      <div class="animate-pulse">
        <div class="neumorphic h-20 w-20 rounded-full mx-auto mb-6 flex items-center justify-center">
          <div class="h-10 w-10 rounded-full bg-blue-200"></div>
        </div>
        <h2 class="text-2xl font-semibold dreamy-text mb-4">Completing Authentication</h2>
        <p class="text-gray-600">Please wait while we complete your authentication...</p>
      </div>
    {:else if error}
      <div class="neumorphic h-20 w-20 rounded-full mx-auto mb-6 flex items-center justify-center text-red-500">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      </div>
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Authentication Failed</h2>
      <p class="text-gray-600 mb-6">{error}</p>
      <a href="/" class="btn-primary">Return to Home</a>
    {:else if success}
      <div class="neumorphic h-20 w-20 rounded-full mx-auto mb-6 flex items-center justify-center text-green-500">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Authentication Successful!</h2>
      <p class="text-gray-600 mb-6">You are now logged in. Redirecting to your profile...</p>
      <div class="w-full bg-gray-200 h-2 rounded-full overflow-hidden">
        <div class="bg-blue-500 h-full rounded-full" style="width: 100%; transition: width 2s ease;"></div>
      </div>
    {/if}
  </div>
</div> 