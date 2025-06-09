<script lang="ts">
  import { onMount } from 'svelte';
  import { qdrantService, type SearchResult, type SearchOptions } from '$lib/services/qdrantService';
  import { pulsateEffect, cardEntryAnimation } from '$lib/animations.js';
  import PageTransition from '$lib/components/PageTransition.svelte';
  import AnimatedCard from '$lib/components/AnimatedCard.svelte';
  
  // Search state
  let searchQuery = '';
  let loading = false;
  let results: SearchResult[] = [];
  let error = '';
  
  // Filter state
  let priceRange = { min: 0, max: 1000 };
  let itemType = 'all'; // 'all', 'product', 'service'
  let useLocation = false;
  
  /** @type {HTMLElement} */
  let searchButton: HTMLElement;
  /** @type {HTMLElement} */
  let resultsContainer: HTMLElement;
  
  // Mock embedding generation function (in a real app, this would call your backend API)
  async function generateEmbedding(text: string): Promise<number[]> {
    // This is a mock function - in a real app this would call an API to generate embeddings
    // For demo purposes, just return a random 384-dimensional vector
    return Array.from({ length: 384 }, () => Math.random() * 2 - 1);
  }
  
  // Get user's location
  async function getUserLocation(): Promise<{lat: number, lon: number}> {
    return new Promise((resolve, reject) => {
      if (!navigator.geolocation) {
        reject(new Error('Geolocation is not supported by your browser'));
        return;
      }
      
      navigator.geolocation.getCurrentPosition(
        position => {
          resolve({
            lat: position.coords.latitude,
            lon: position.coords.longitude
          });
        },
        error => {
          reject(error);
        }
      );
    });
  }
  
  // Handle search
  async function handleSearch() {
    if (!searchQuery.trim()) {
      error = 'Please enter a search query';
      return;
    }
    
    try {
      loading = true;
      error = '';
      
      // Generate embedding for search query
      const embedding = await generateEmbedding(searchQuery);
      
      // Build search options
      const searchOptions: SearchOptions = {
        limit: 20,
        threshold: 0.65,
        filter: {}
      };
      
      // Add price range filter
      if (priceRange.min > 0 || priceRange.max < 1000) {
        if (!searchOptions.filter) searchOptions.filter = {};
        searchOptions.filter.minPrice = priceRange.min;
        searchOptions.filter.maxPrice = priceRange.max;
      }
      
      // Add item type filter
      if (itemType !== 'all') {
        if (!searchOptions.filter) searchOptions.filter = {};
        searchOptions.filter.type = itemType;
      }
      
      // Add location if requested
      if (useLocation) {
        try {
          searchOptions.location = await getUserLocation();
        } catch (locationError) {
          console.error('Error getting location:', locationError);
          // Continue search without location
        }
      }
      
      // Perform search
      results = await qdrantService.searchSimilarItems(embedding, searchOptions);
      
      // Animate results container after search completes
      if (resultsContainer) {
        cardEntryAnimation(resultsContainer);
      }
    } catch (searchError) {
      console.error('Search error:', searchError);
      error = 'An error occurred during search. Please try again.';
      results = [];
    } finally {
      loading = false;
    }
  }
  
  onMount(() => {
    if (searchButton) {
      pulsateEffect(searchButton);
    }
  });
</script>

<PageTransition>
  <div class="container mx-auto px-4 py-12">
    <h1 class="text-4xl font-bold text-sky-900 mb-8 text-center">Find Products and Services</h1>
    
    <div class="glass max-w-4xl mx-auto p-6 mb-12">
      <div class="flex flex-col md:flex-row gap-4 mb-6">
        <div class="flex-grow">
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search for products or services..."
            class="glass-sm w-full p-4 rounded-xl"
          />
        </div>
        <button
          bind:this={searchButton}
          on:click={handleSearch}
          class="glass-button px-8 py-4 text-sky-900 font-medium rounded-xl"
          disabled={loading}
        >
          {loading ? 'Searching...' : 'Search'}
        </button>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <!-- Price Range Filter -->
        <div>
          <label class="block mb-2 text-sky-800">Price Range</label>
          <div class="flex items-center gap-2">
            <input
              type="number"
              bind:value={priceRange.min}
              min="0"
              class="glass-sm w-full p-2 rounded-lg"
              placeholder="Min"
            />
            <span>to</span>
            <input
              type="number"
              bind:value={priceRange.max}
              min="0"
              class="glass-sm w-full p-2 rounded-lg"
              placeholder="Max"
            />
          </div>
        </div>
        
        <!-- Item Type Filter -->
        <div>
          <label class="block mb-2 text-sky-800">Item Type</label>
          <select bind:value={itemType} class="glass-sm w-full p-3 rounded-lg">
            <option value="all">All Types</option>
            <option value="product">Products Only</option>
            <option value="service">Services Only</option>
          </select>
        </div>
        
        <!-- Location Filter -->
        <div class="flex items-end">
          <label class="flex items-center gap-2 text-sky-800">
            <input type="checkbox" bind:checked={useLocation} />
            Use my location for nearest results
          </label>
        </div>
      </div>
    </div>
    
    {#if error}
      <div class="glass-accent p-4 text-red-600 mb-8 text-center">
        {error}
      </div>
    {/if}
    
    {#if loading}
      <div class="text-center py-12">
        <div class="inline-block glass-sm p-6 rounded-full animate-spin mb-4">
          <div class="w-8 h-8 rounded-full border-4 border-t-sky-600 border-r-transparent border-b-transparent border-l-transparent"></div>
        </div>
        <p>Searching for the perfect match...</p>
      </div>
    {:else if results.length > 0}
      <div bind:this={resultsContainer} class="mb-8">
        <h2 class="text-2xl font-semibold text-sky-900 mb-6">Search Results</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          {#each results as result}
            <AnimatedCard
              title={result.payload.name || 'Unnamed Item'}
              description={result.payload.description || ''}
              imageUrl={result.payload.imageUrl || '/images/placeholder.jpg'}
              altText={result.payload.name || 'Item image'}
              href={`/i?i=${result.id}`}
            />
          {/each}
        </div>
      </div>
    {:else if searchQuery && !loading}
      <div class="text-center py-12 glass-sm max-w-md mx-auto">
        <p class="text-lg mb-4">No items match your search criteria.</p>
        <p>Try adjusting your filters or search with different keywords.</p>
      </div>
    {/if}
  </div>
</PageTransition>

<style>
  @theme {
    --glass-opacity: 0.15;
    --glass-blur: 15px;
    --glass-border: 1px solid rgba(255, 255, 255, 0.2);
    --glass-shadow: 0 8px 32px rgba(14, 165, 233, 0.2);
  }
  
  @layer components {
    .glass-sm {
      background: rgba(255, 255, 255, 0.1);
      backdrop-filter: blur(10px);
      border: 1px solid rgba(255, 255, 255, 0.2);
      box-shadow: 0 4px 16px rgba(14, 165, 233, 0.1);
      border-radius: 1rem;
    }
    
    .glass-accent {
      background: rgba(255, 255, 255, 0.2);
      backdrop-filter: blur(10px);
      border: 1px solid rgba(255, 55, 55, 0.3);
      box-shadow: 0 4px 16px rgba(220, 38, 38, 0.1);
      border-radius: 1rem;
    }
    
    .glass-button {
      background: rgba(255, 255, 255, 0.25);
      backdrop-filter: blur(15px);
      border: 1px solid rgba(255, 255, 255, 0.3);
      box-shadow: 0 8px 16px rgba(14, 165, 233, 0.15);
      transition: all 0.3s ease;
    }
    
    .glass-button:hover:not(:disabled) {
      background: rgba(255, 255, 255, 0.35);
      box-shadow: 0 8px 25px rgba(14, 165, 233, 0.25);
      transform: translateY(-2px);
    }
    
    .glass-button:disabled {
      opacity: 0.7;
      cursor: not-allowed;
    }
  }
</style> 