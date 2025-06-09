<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { onMount } from 'svelte';
  import { currentUser } from '$lib/stores/userStore';
  
  // API URL configuration - assuming localhost:8000 for now
  const API_URL = 'http://localhost:8000';
  
  let searchQuery = $state('');
  let searchResults = $state<any[]>([]);
  let isSearching = $state(false);
  let error = $state<string | null>(null);
  
  async function searchMessages() {
    if (!searchQuery.trim() || !$currentUser) return;
    
    isSearching = true;
    error = null;
    
    try {
      const response = await fetch(`${API_URL}/chat/search`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${$currentUser.id}`
        },
        body: JSON.stringify({
          query: searchQuery,
          limit: 20
        })
      });
      
      if (!response.ok) {
        throw new Error(`Search failed: ${response.statusText}`);
      }
      
      const data = await response.json();
      searchResults = data;
    } catch (err) {
      console.error('Error searching messages:', err);
      error = err instanceof Error ? err.message : 'Failed to search messages';
      searchResults = [];
    } finally {
      isSearching = false;
    }
  }
  
  function formatDate(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  }
  
  // Clear results when query is cleared
  $effect(() => {
    if (!searchQuery.trim()) {
      searchResults = [];
    }
  });
</script>

<div class="glass-sm p-4 rounded-xl">
  <h3 class="text-xl font-semibold mb-4 dreamy-text">Search Messages</h3>
  
  <form on:submit|preventDefault={searchMessages} class="mb-4">
    <div class="flex gap-2">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search for message content..."
        class="input-glass flex-1"
      />
      <button 
        type="submit" 
        class="btn-primary"
        disabled={isSearching || !searchQuery.trim()}
      >
        {isSearching ? 'Searching...' : 'Search'}
      </button>
    </div>
  </form>
  
  {#if error}
    <div class="bg-red-100 text-red-800 p-3 rounded-lg mb-4" transition:fade>
      {error}
    </div>
  {/if}
  
  {#if searchResults.length > 0}
    <div class="mt-4">
      <h4 class="text-lg font-semibold mb-2">Results</h4>
      <div class="space-y-3 max-h-96 overflow-y-auto p-2">
        {#each searchResults as result}
          <div 
            class="glass-sm p-3 rounded-lg"
            transition:fly={{ y: 10, duration: 200 }}
          >
            <p class="mb-2">{result.message}</p>
            <div class="flex justify-between text-xs text-gray-500">
              <span>From: {result.sender_id === $currentUser?.id ? 'You' : result.sender_id}</span>
              <span>{formatDate(result.timestamp)}</span>
            </div>
            <div class="text-xs text-gray-500 mt-1">
              Session: {result.session_id}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else if searchQuery && !isSearching}
    <div class="text-center py-6 text-gray-500">
      No messages found matching your search
    </div>
  {/if}
</div> 