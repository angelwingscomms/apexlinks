<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { ServiceSearchParams } from '$lib/services/serviceService';
  
  const dispatch = createEventDispatcher<{
    search: ServiceSearchParams;
  }>();
  
  const props = $props<{ loading?: boolean }>();
  
  // Search parameters
  let query = $state('');
  let type = $state('');
  let minRating = $state<number | null>(null);
  let maxPrice = $state<number | null>(null);
  let location = $state('');
  
  // Service type options
  const typeOptions = [
    { value: '', label: 'All Types' },
    { value: 'mechanics', label: 'Mechanics' },
    { value: 'towing', label: 'Towing' },
    { value: 'cleaning', label: 'Cleaning' },
    { value: 'delivery', label: 'Delivery' }
  ];
  
  // Rating options
  const ratingOptions = [
    { value: null, label: 'Any Rating' },
    { value: 3, label: '3+ Stars' },
    { value: 4, label: '4+ Stars' },
    { value: 5, label: '5 Stars' }
  ];
  
  // Price range options
  const priceOptions = [
    { value: null, label: 'Any Price' },
    { value: 50, label: 'Up to $50' },
    { value: 100, label: 'Up to $100' },
    { value: 200, label: 'Up to $200' }
  ];
  
  // Initialize form from URL parameters if available
  function initFromUrlParams() {
    const url = new URL(window.location.href);
    
    query = url.searchParams.get('q') || '';
    type = url.searchParams.get('type') || '';
    
    const urlMinRating = url.searchParams.get('minRating');
    minRating = urlMinRating ? Number(urlMinRating) : null;
    
    const urlMaxPrice = url.searchParams.get('maxPrice');
    maxPrice = urlMaxPrice ? Number(urlMaxPrice) : null;
    
    location = url.searchParams.get('location') || '';
    
    // If we have any parameters, perform search
    if (query || type || minRating || maxPrice || location) {
      handleSubmit();
    }
  }
  
  // Handle form submission
  function handleSubmit() {
    const params: ServiceSearchParams = {
      limit: 50
    };
    
    // Add search parameters if they're set
    if (query) {
      params.query = query;
    }
    
    if (type) {
      params.type = type;
    }
    
    if (minRating !== null) {
      params.minRating = minRating;
    }
    
    if (maxPrice !== null) {
      params.maxPrice = maxPrice;
    }
    
    if (location) {
      params.location = location;
    }
    
    dispatch('search', params);
  }
  
  // Reset the form
  function resetForm() {
    query = '';
    type = '';
    minRating = null;
    maxPrice = null;
    location = '';
  }
  
  // Initialize on mount
  import { onMount } from 'svelte';
  onMount(initFromUrlParams);
</script>

<div class="card-glass p-6 mb-8">
  <h2 class="text-xl font-semibold dreamy-text mb-4">Search Services</h2>
  
  <div class="mb-4">
    <label for="query" class="block text-gray-700 mb-2">Service Name or Keywords</label>
    <input
      type="text"
      id="query"
      placeholder="What service are you looking for?"
      bind:value={query}
      class="w-full neumorphic p-3 rounded-lg"
    />
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
    <div>
      <label for="type" class="block text-gray-700 mb-2">Service Type</label>
      <select 
        id="type"
        bind:value={type}
        class="w-full neumorphic p-3 rounded-lg"
      >
        {#each typeOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>
    
    <div>
      <label for="minRating" class="block text-gray-700 mb-2">Minimum Rating</label>
      <select 
        id="minRating"
        bind:value={minRating}
        class="w-full neumorphic p-3 rounded-lg"
      >
        {#each ratingOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>
    
    <div>
      <label for="maxPrice" class="block text-gray-700 mb-2">Maximum Price</label>
      <select 
        id="maxPrice"
        bind:value={maxPrice}
        class="w-full neumorphic p-3 rounded-lg"
      >
        {#each priceOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>
  </div>
  
  <div class="mb-6">
    <label for="location" class="block text-gray-700 mb-2">Location</label>
    <input
      type="text"
      id="location"
      placeholder="Enter location"
      bind:value={location}
      class="w-full neumorphic p-3 rounded-lg"
    />
  </div>
  
  <div class="flex gap-3">
    <button 
      on:click={handleSubmit}
      disabled={props.loading}
      class="btn-primary flex-1"
    >
      {#if props.loading}
        <span class="animate-pulse">Searching...</span>
      {:else}
        Search
      {/if}
    </button>
    <button 
      on:click={resetForm}
      disabled={props.loading}
      class="btn"
    >
      Reset
    </button>
  </div>
</div> 