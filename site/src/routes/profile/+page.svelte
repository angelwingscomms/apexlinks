<script lang="ts">
  import { currentUser, isAuthenticated } from '$lib/stores/userStore';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  // Redirect to login if not authenticated
  onMount(() => {
    if (!$isAuthenticated) {
      goto('/');
    }
  });
</script>

<div>
  {#if !$isAuthenticated}
    <div class="card-neumorphic text-center py-12">
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Login Required</h2>
      <p class="text-gray-600 mb-6">You need to be logged in to view your profile.</p>
      <a href="/" class="btn-primary">Back to Home</a>
    </div>
  {:else if $currentUser}
    <div class="card-neumorphic">
      <div class="flex flex-col md:flex-row items-center md:items-start gap-6">
        <!-- Profile Image -->
        <div class="neumorphic-inset h-40 w-40 rounded-full overflow-hidden flex items-center justify-center">
          {#if $currentUser.picture}
            <img src={$currentUser.picture} alt={$currentUser.name} class="h-full w-full object-cover" />
          {:else}
            <div class="h-full w-full flex items-center justify-center bg-gradient-to-br from-blue-400 to-blue-600 text-white text-3xl font-bold">
              {$currentUser.name.charAt(0).toUpperCase()}
            </div>
          {/if}
        </div>
        
        <!-- User Info -->
        <div class="flex-1 text-center md:text-left">
          <h1 class="text-3xl font-bold dreamy-text mb-2">{$currentUser.name}</h1>
          
          {#if $currentUser.username}
            <p class="text-gray-600 text-lg">@{$currentUser.username}</p>
          {/if}
          
          {#if $currentUser.email}
            <p class="text-gray-500 text-sm mt-1">{$currentUser.email}</p>
          {/if}
          
          {#if $currentUser.description}
            <div class="mt-4 neumorphic-inset p-4 rounded-lg">
              <p class="text-gray-700">{$currentUser.description}</p>
            </div>
          {/if}
          
          <div class="mt-6 grid grid-cols-2 gap-4">
            {#if $currentUser.age}
              <div class="neumorphic-inset p-3 rounded-lg">
                <span class="text-sm font-medium text-gray-500">Age</span>
                <p class="text-gray-800 font-medium">{$currentUser.age} years</p>
              </div>
            {/if}
            
            {#if $currentUser.gender}
              <div class="neumorphic-inset p-3 rounded-lg">
                <span class="text-sm font-medium text-gray-500">Gender</span>
                <p class="text-gray-800 font-medium">{$currentUser.gender}</p>
              </div>
            {/if}
          </div>
        </div>
      </div>
      
      <!-- Action Buttons -->
      <div class="mt-8 flex justify-end gap-3">
        <a href="/users" class="neumorphic-button px-4 py-2 text-sm text-gray-600">
          Find Users
        </a>
        
        <a href="/profile/edit" class="btn-primary">
          Edit Profile
        </a>
      </div>
    </div>
    
    <!-- User's Zone Information -->
    {#if $currentUser.zone_id}
      <div class="card-neumorphic mt-8">
        <h2 class="text-xl font-semibold dreamy-text mb-4">Your Zone</h2>
        <p class="text-gray-700 mb-4">You are currently in zone: <span class="font-semibold">{$currentUser.zone_id}</span></p>
        <div class="flex justify-end">
          <a href="/zones" class="neumorphic-button px-4 py-2 text-sm text-gray-600">
            Explore Zones
          </a>
        </div>
      </div>
    {:else}
      <div class="card-neumorphic mt-8">
        <h2 class="text-xl font-semibold dreamy-text mb-4">Join a Zone</h2>
        <p class="text-gray-700 mb-4">You haven't joined any zone yet. Zones help you connect with like-minded people in specific communities.</p>
        <div class="flex justify-end">
          <a href="/zones" class="btn-primary">
            Find Zones
          </a>
        </div>
      </div>
    {/if}
  {/if}
</div> 