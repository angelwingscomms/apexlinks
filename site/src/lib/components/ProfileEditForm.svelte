<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { currentUser, userStore } from '../stores/userStore';
  import type { User } from '../stores/userStore';
  
  const dispatch = createEventDispatcher<{
    save: User;
    cancel: void;
  }>();
  
  export let loading = false;
  
  // Profile data
  let username = $currentUser?.username || '';
  let description = $currentUser?.description || '';
  let age = $currentUser?.age || null;
  let gender = $currentUser?.gender || '';
  
  // Available gender options
  const genderOptions = [
    { value: '', label: 'Select Gender' },
    { value: 'male', label: 'Male' },
    { value: 'female', label: 'Female' },
    { value: 'non-binary', label: 'Non-binary' },
    { value: 'other', label: 'Other' },
    { value: 'prefer-not-to-say', label: 'Prefer not to say' }
  ];
  
  // Handle form submission
  async function handleSubmit() {
    if (!$currentUser) return;
    
    // Create updated user object
    const updatedUser: Partial<User> = {
      username,
      description,
      age: age !== null ? Number(age) : undefined,
      gender
    };
    
    // Dispatch save event
    dispatch('save', { ...$currentUser, ...updatedUser });
  }
  
  // Handle cancel
  function handleCancel() {
    dispatch('cancel');
  }
</script>

<div class="card-glass">
  <h2 class="text-xl font-semibold dreamy-text mb-6">Edit Your Profile</h2>
  
  <form on:submit|preventDefault={handleSubmit}>
    <!-- Username -->
    <div class="mb-4">
      <label for="username" class="block text-sm font-medium text-gray-700 mb-1">Username</label>
      <input
        id="username"
        bind:value={username}
        type="text"
        class="input-glass w-full"
        placeholder="Your username"
      />
    </div>
    
    <!-- Description -->
    <div class="mb-4">
      <label for="description" class="block text-sm font-medium text-gray-700 mb-1">About You</label>
      <textarea
        id="description"
        bind:value={description}
        class="input-glass w-full h-32"
        placeholder="Tell us about yourself"
      ></textarea>
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <!-- Age -->
      <div>
        <label for="age" class="block text-sm font-medium text-gray-700 mb-1">Age</label>
        <input
          id="age"
          bind:value={age}
          type="number"
          min="13"
          max="120"
          class="input-glass w-full"
          placeholder="Your age"
        />
      </div>
      
      <!-- Gender -->
      <div>
        <label for="gender" class="block text-sm font-medium text-gray-700 mb-1">Gender</label>
        <select
          id="gender"
          bind:value={gender}
          class="input-glass w-full"
        >
          {#each genderOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
    </div>
    
    <!-- Action buttons -->
    <div class="flex justify-end space-x-3">
      <button
        type="button"
        on:click={handleCancel}
        class="glass-button px-4 py-2 text-sm text-gray-600"
        disabled={loading}
      >
        Cancel
      </button>
      
      <button
        type="submit"
        class="btn-primary"
        disabled={loading}
      >
        {#if loading}
          <span class="animate-pulse">Saving...</span>
        {:else}
          Save Changes
        {/if}
      </button>
    </div>
  </form>
</div> 