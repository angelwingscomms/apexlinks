<script lang="ts">
  import UserSearchForm from '$lib/components/UserSearchForm.svelte';
  import UserCard from '$lib/components/UserCard.svelte';
  import userService from '$lib/services/userService';
  import type { UserSearchResult, UserSearchParams } from '$lib/services/userService';
  import { isAuthenticated } from '$lib/stores/userStore';
  import { onMount } from 'svelte';
  
  // State variables
  let users: UserSearchResult[] = [];
  let loading = $state(false);
  let error = $state<string | null>(null);
  let searchPerformed = $state(false);
  
  // Perform search with the given parameters
  async function handleSearch(event: CustomEvent<UserSearchParams>) {
    const params = event.detail;
    
    loading = true;
    error = null;
    searchPerformed = true;
    
    try {
      users = await userService.searchUsers(params);
    } catch (err) {
      console.error('Error searching users:', err);
      error = 'Failed to search users. Please try again.';
      users = [];
    } finally {
      loading = false;
    }
  }
  
  // Perform an initial search on page load to show some users
  onMount(async () => {
    if ($isAuthenticated) {
      loading = true;
      
      try {
        // Initial search with empty query to get some users
        users = await userService.searchUsers({
          query: '',
          s: 'u',
          limit: 10
        });
      } catch (err) {
        console.error('Error loading initial users:', err);
        error = 'Failed to load users. Please try again.';
      } finally {
        loading = false;
      }
    }
  });
</script>

<div>
  <h1 class="text-4xl font-bold dreamy-text mb-8">Find Users</h1>
  
  {#if !$isAuthenticated}
    <div class="card-neumorphic text-center py-12">
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Login Required</h2>
      <p class="text-gray-600 mb-6">You need to be logged in to search for users.</p>
      <a href="/" class="btn-primary">Back to Home</a>
    </div>
  {:else}
    <!-- Search Form -->
    <UserSearchForm 
      {loading}
      on:search={handleSearch}
    />
    
    <!-- Results -->
    <div class="mt-8">
      {#if loading}
        <div class="flex justify-center py-12">
          <div class="animate-pulse text-center">
            <div class="neumorphic h-16 w-16 rounded-full mx-auto mb-4"></div>
            <p class="text-gray-600">Searching for users...</p>
          </div>
        </div>
      {:else if error}
        <div class="card-neumorphic bg-red-50 text-center py-6">
          <p class="text-red-600">{error}</p>
          <button 
            class="btn-primary mt-4" 
            on:click={() => error = null}
          >
            Try Again
          </button>
        </div>
      {:else if users.length === 0 && searchPerformed}
        <div class="card-neumorphic text-center py-8">
          <h3 class="text-xl font-semibold dreamy-text mb-2">No Users Found</h3>
          <p class="text-gray-600">Try adjusting your search criteria.</p>
        </div>
      {:else if users.length === 0}
        <div class="card-neumorphic text-center py-8">
          <h3 class="text-xl font-semibold dreamy-text mb-2">Start Searching</h3>
          <p class="text-gray-600">Use the search form above to find users.</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          {#each users as user (user.id)}
            <UserCard {user} />
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div> 