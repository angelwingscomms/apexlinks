<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { UserSearchParams } from '../services/userService';
  
  const dispatch = createEventDispatcher<{
    search: UserSearchParams;
  }>();
  
  export let loading = false;
  
  // Search parameters
  let query = '';
  let minAge: number | null = null;
  let maxAge: number | null = null;
  let gender: string = '';
  
  // Available gender options
  const genderOptions = [
    { value: '', label: 'Any' },
    { value: 'male', label: 'Male' },
    { value: 'female', label: 'Female' },
    { value: 'non-binary', label: 'Non-binary' },
    { value: 'other', label: 'Other' }
  ];
  
  // Handle form submission
  function handleSubmit() {
    const params: UserSearchParams = {
      query,
      s: 'u', // tenant identifier for users
      limit: 50
    };
    
    // Add filters if they're set
    if (gender) {
      params.gender = gender;
    }
    
    // Use minAge if it's set
    if (minAge !== null) {
      params.minAge = minAge;
    }
    
    // Use maxAge if it's set
    if (maxAge !== null) {
      params.maxAge = maxAge;
    }
    
    dispatch('search', params);
  }
  
  // Reset the form
  function resetForm() {
    query = '';
    minAge = null;
    maxAge = null;
    gender = '';
  }
</script>

<div class="glass p-6 mb-8">
  <h2 class="text-xl font-semibold dreamy-text mb-4">Find Users</h2>
  
  <form on:submit|preventDefault={handleSubmit}>
    <!-- Search query input -->
    <div class="mb-4">
      <label for="search-query" class="block text-sm font-medium text-gray-700 mb-1">Search</label>
      <input
        id="search-query"
        bind:value={query}
        type="text"
        placeholder="Search by name, username, or description"
        class="input-glass w-full"
      />
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
      <!-- Age range -->
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Age Range</label>
        <div class="flex items-center space-x-2">
          <input
            bind:value={minAge}
            type="number"
            min="13"
            max="120"
            placeholder="Min"
            class="input-glass w-full"
          />
          <span class="text-gray-500">to</span>
          <input
            bind:value={maxAge}
            type="number"
            min="13"
            max="120"
            placeholder="Max"
            class="input-glass w-full"
          />
        </div>
      </div>
      
      <!-- Gender filter -->
      <div>
        <label for="gender-filter" class="block text-sm font-medium text-gray-700 mb-1">Gender</label>
        <select
          id="gender-filter"
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
        on:click={resetForm}
        class="glass-button px-4 py-2 text-sm text-gray-600"
        disabled={loading}
      >
        Reset
      </button>
      
      <button
        type="submit"
        class="btn-primary"
        disabled={loading}
      >
        {#if loading}
          <span class="animate-pulse">Searching...</span>
        {:else}
          Search
        {/if}
      </button>
    </div>
  </form>
</div> 