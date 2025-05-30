<script lang="ts">
  import { onMount } from 'svelte';
  import { isAuthenticated, currentUser } from '$lib/stores/userStore';
  import ChatInterface from '$lib/components/ChatInterface.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  
  let sessionId = $state<string | null>(null);
  let partnerId = $state<string | null>(null);
  
  onMount(() => {
    // Check for session ID in query params
    const urlSessionId = $page.url.searchParams.get('session');
    const urlPartnerId = $page.url.searchParams.get('partner');
    
    if (urlSessionId) {
      sessionId = urlSessionId;
    }
    
    if (urlPartnerId) {
      partnerId = urlPartnerId;
    }
    
    // Redirect to login if not authenticated
    if (!$isAuthenticated) {
      goto('/auth/login?redirect=/chat');
    }
  });
  
  function handleSessionCreated(event: CustomEvent<{ sessionId: string, partnerId: string }>) {
    sessionId = event.detail.sessionId;
    partnerId = event.detail.partnerId;
    
    // Update URL to include session ID without reloading
    const url = new URL(window.location.href);
    url.searchParams.set('session', sessionId);
    url.searchParams.set('partner', partnerId);
    window.history.pushState({}, '', url);
  }
</script>

<div class="container mx-auto p-4 md:p-8">
  <div class="card-glass mb-6">
    <h1 class="text-3xl font-bold dreamy-text mb-4">Chat</h1>
    <p class="text-gray-700">Connect with other users through real-time messaging</p>
  </div>
  
  {#if $isAuthenticated}
    <div class="glass h-[70vh] rounded-xl overflow-hidden">
      {#if partnerId !== null}
        <ChatInterface 
          partnerId={partnerId || ''} 
          sessionId={sessionId} 
          on:sessionCreated={handleSessionCreated}
        />
      {:else}
        <div class="flex flex-col h-full">
          <div class="p-6 border-b border-red-200">
            <h2 class="text-2xl font-semibold dreamy-text">Start a New Chat</h2>
            <p class="text-gray-600 mt-2">Find someone to chat with based on shared interests</p>
          </div>
          
          <div class="flex-1 flex items-center justify-center">
            <div class="text-center max-w-md p-6">
              <div class="glass-sm h-24 w-24 rounded-full mx-auto mb-6 flex items-center justify-center">
                <span class="text-red-500 text-4xl">💬</span>
              </div>
              <h3 class="text-xl font-semibold mb-4">Ready to connect?</h3>
              <p class="text-gray-600 mb-6">
                Our matching system will pair you with someone who shares similar interests and preferences.
              </p>
              <ChatInterface 
                partnerId="" 
                sessionId={null} 
                on:sessionCreated={handleSessionCreated}
              />
            </div>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="card-glass text-center py-12">
      <div class="glass-sm h-20 w-20 rounded-full mx-auto mb-6 flex items-center justify-center">
        <span class="text-red-500 text-3xl">🔒</span>
      </div>
      <h2 class="text-2xl font-semibold mb-4">Login Required</h2>
      <p class="text-gray-600 mb-6">You need to be logged in to use the chat feature</p>
      <a href="/auth/login?redirect=/chat" class="btn-primary">Login Now</a>
    </div>
  {/if}
</div> 