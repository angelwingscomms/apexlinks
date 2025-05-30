<script lang="ts">
  import { currentUser, isAuthenticated, userStore } from '$lib/stores/userStore';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import ProfileEditForm from '$lib/components/ProfileEditForm.svelte';
  import type { User } from '$lib/stores/userStore';
  
  // State variables
  let loading = $state(false);
  let error = $state<string | null>(null);
  let success = $state(false);
  
  // Redirect to login if not authenticated
  onMount(() => {
    if (!$isAuthenticated) {
      goto('/');
    }
  });
  
  // Handle profile update
  async function handleSave(event: CustomEvent<User>) {
    const updatedUser = event.detail;
    
    loading = true;
    error = null;
    success = false;
    
    try {
      const result = await userStore.updateProfile(updatedUser);
      
      if (result) {
        success = true;
        setTimeout(() => {
          goto('/profile');
        }, 1500);
      } else {
        throw new Error('Failed to update profile');
      }
    } catch (err) {
      console.error('Error updating profile:', err);
      error = 'Failed to update profile. Please try again.';
    } finally {
      loading = false;
    }
  }
  
  // Handle cancel
  function handleCancel() {
    goto('/profile');
  }
</script>

<div>
  <h1 class="text-3xl font-bold dreamy-text mb-8">Edit Your Profile</h1>
  
  {#if !$isAuthenticated}
    <div class="card-glass text-center py-12">
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Login Required</h2>
      <p class="text-gray-600 mb-6">You need to be logged in to edit your profile.</p>
      <a href="/" class="btn-primary">Back to Home</a>
    </div>
  {:else if error}
    <div class="card-glass bg-red-50 text-center py-6 mb-6">
      <p class="text-red-600">{error}</p>
    </div>
    <ProfileEditForm 
      {loading}
      on:save={handleSave}
      on:cancel={handleCancel}
    />
  {:else if success}
    <div class="card-glass bg-green-50 text-center py-12">
      <div class="glass h-16 w-16 mx-auto mb-4 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 class="text-2xl font-semibold dreamy-text mb-2">Profile Updated!</h2>
      <p class="text-gray-600 mb-6">Your profile has been successfully updated.</p>
      <p class="text-sm text-gray-500">Redirecting to your profile...</p>
    </div>
  {:else}
    <ProfileEditForm 
      {loading}
      on:save={handleSave}
      on:cancel={handleCancel}
    />
  {/if}
</div> 