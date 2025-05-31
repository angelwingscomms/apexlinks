<script lang="ts">
  import { onMount } from 'svelte';
  import { handleAuthCallback } from '$lib/api';
  import Header from '$lib/components/Header.svelte';
  
  let message = $state('Processing authentication...');
  let success = $state(false);
  
  onMount(() => {
    try {
      if (handleAuthCallback()) {
        success = true;
        message = 'Authentication successful! Redirecting...';
        setTimeout(() => {
          window.location.href = '/';
        }, 2000);
      } else {
        message = 'No authentication token found. Please try again.';
      }
    } catch (error) {
      console.error('Authentication error:', error);
      message = 'Authentication failed. Please try again.';
    }
  });
</script>

<Header activeTab="none" />

<div class="flex flex-col items-center justify-center p-8">
  <div class="neumorphic p-8 max-w-md text-center">
    <div class="w-16 h-16 mx-auto mb-6 rounded-full bg-gradient-dark flex items-center justify-center">
      {#if success}
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-primary">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
      {:else}
        <div class="w-6 h-6 rounded-full border-2 border-transparent border-t-primary animate-spin"></div>
      {/if}
    </div>
    
    <h1 class="text-2xl font-bold mb-4">Authentication</h1>
    <p class="text-secondary mb-6">{message}</p>
    
    {#if !success}
      <a href="/" class="btn w-full">Return to Home</a>
    {/if}
  </div>
</div> 