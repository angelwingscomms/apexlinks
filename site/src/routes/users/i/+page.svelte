<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import userService from '$lib/services/userService';
  import type { User } from '$lib/stores/userStore';
  import type { SimilarityResult } from '$lib/services/userService';
  import { currentUser, isAuthenticated } from '$lib/stores/userStore';
  import SimilarityDisplay from '$lib/components/SimilarityDisplay.svelte';
  
  // Get user ID from query params
  const userId = $page.url.searchParams.get('i');
  
  // State variables
  let user = $state<User | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  
  // Similarity data
  let similarity = $state<SimilarityResult | null>(null);
  let similarityLoading = $state(false);
  let similarityError = $state<string | null>(null);
  
  // Load user data
  onMount(async () => {
    loading = true;
    error = null;
    
    if (!userId) {
      error = 'No user ID provided.';
      loading = false;
      return;
    }
    
    try {
      user = await userService.getUserById(userId);
      
      // If we're authenticated and viewing another user's profile, calculate similarity
      if ($isAuthenticated && $currentUser && userId !== $currentUser.id) {
        await loadSimilarity();
      }
    } catch (err) {
      console.error('Error loading user:', err);
      error = 'Failed to load user profile. Please try again.';
    } finally {
      loading = false;
    }
  });
  
  // Load similarity data
  async function loadSimilarity() {
    if (!$currentUser || !userId) return;
    
    similarityLoading = true;
    similarityError = null;
    
    try {
      similarity = await userService.calculateSimilarity($currentUser.id, userId);
    } catch (err) {
      console.error('Error calculating similarity:', err);
      similarityError = 'Failed to calculate similarity. Please try again.';
    } finally {
      similarityLoading = false;
    }
  }
</script>

<div>
  {#if loading}
    <div class="flex justify-center py-16">
      <div class="animate-pulse text-center">
        <div class="glass h-24 w-24 rounded-full mx-auto mb-4"></div>
        <p class="text-gray-600">Loading profile...</p>
      </div>
    </div>
  {:else if error}
    <div class="card-glass bg-red-50 text-center py-10">
      <p class="text-red-600">{error}</p>
      <a href="/users" class="btn-primary mt-6">Back to Users</a>
    </div>
  {:else if user}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
      <!-- User Profile Card -->
      <div class="md:col-span-2">
        <div class="card-glass">
          <div class="flex flex-col md:flex-row items-center md:items-start gap-6">
            <!-- Profile Image -->
            <div class="glass-sm h-40 w-40 rounded-full overflow-hidden flex items-center justify-center">
              {#if user.picture}
                <img src={user.picture} alt={user.name} class="h-full w-full object-cover" />
              {:else}
                <div class="h-full w-full flex items-center justify-center bg-gradient-to-br from-blue-400 to-blue-600 text-white text-3xl font-bold">
                  {user.name.charAt(0).toUpperCase()}
                </div>
              {/if}
            </div>
            
            <!-- User Info -->
            <div class="flex-1 text-center md:text-left">
              <h1 class="text-3xl font-bold dreamy-text mb-2">{user.name}</h1>
              
              {#if user.username}
                <p class="text-gray-600 text-lg">@{user.username}</p>
              {/if}
              
              {#if user.description}
                <div class="mt-4 glass-sm p-4 rounded-lg">
                  <p class="text-gray-700">{user.description}</p>
                </div>
              {/if}
              
              <div class="mt-6 grid grid-cols-2 gap-4">
                {#if user.age}
                  <div class="glass-sm p-3 rounded-lg">
                    <span class="text-sm font-medium text-gray-500">Age</span>
                    <p class="text-gray-800 font-medium">{user.age} years</p>
                  </div>
                {/if}
                
                {#if user.gender}
                  <div class="glass-sm p-3 rounded-lg">
                    <span class="text-sm font-medium text-gray-500">Gender</span>
                    <p class="text-gray-800 font-medium">{user.gender}</p>
                  </div>
                {/if}
              </div>
            </div>
          </div>
          
          <!-- Action Buttons -->
          <div class="mt-8 flex justify-end gap-3">
            <a href="/users" class="glass-button px-4 py-2 text-sm text-gray-600">
              Back to Users
            </a>
            
            {#if $currentUser && userId === $currentUser.id}
              <a href="/profile/edit" class="btn-primary">
                Edit Profile
              </a>
            {/if}
          </div>
        </div>
      </div>
      
      <!-- Similarity Section (if applicable) -->
      <div class="md:col-span-1">
        {#if $isAuthenticated && $currentUser && userId !== $currentUser.id}
          {#if similarity}
            <SimilarityDisplay similarity={similarity} loading={similarityLoading} />
          {:else if similarityLoading}
            <div class="card-glass">
              <h3 class="text-xl font-semibold dreamy-text mb-4">Compatibility Analysis</h3>
              <div class="flex justify-center py-6">
                <div class="animate-pulse text-center">
                  <div class="glass h-16 w-16 rounded-full mx-auto mb-4"></div>
                  <p class="text-gray-600">Calculating compatibility...</p>
                </div>
              </div>
            </div>
          {:else if similarityError}
            <div class="card-glass bg-red-50">
              <h3 class="text-xl font-semibold dreamy-text mb-4">Compatibility Analysis</h3>
              <p class="text-red-600 mb-4">{similarityError}</p>
              <button 
                class="glass-button px-4 py-2 text-sm text-gray-600 w-full" 
                on:click={loadSimilarity}
              >
                Try Again
              </button>
            </div>
          {/if}
        {:else if $isAuthenticated && $currentUser && userId === $currentUser.id}
          <div class="card-glass">
            <h3 class="text-xl font-semibold dreamy-text mb-4">Your Profile</h3>
            <p class="text-gray-600">This is your own profile. Explore other users to see compatibility.</p>
          </div>
        {:else}
          <div class="card-glass">
            <h3 class="text-xl font-semibold dreamy-text mb-4">Login Required</h3>
            <p class="text-gray-600 mb-4">Login to see compatibility with this user.</p>
            <a href="/" class="btn-primary block text-center">Back to Home</a>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>