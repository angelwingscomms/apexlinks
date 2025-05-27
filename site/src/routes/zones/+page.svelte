<script lang="ts">
  import { isAuthenticated } from '$lib/stores/userStore';
</script>

<div>
  <h1 class="text-4xl font-bold dreamy-text mb-8">Explore Zones</h1>
  
  {#if !$isAuthenticated}
    <div class="card-neumorphic text-center py-12">
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Login Required</h2>
      <p class="text-gray-600 mb-6">You need to be logged in to explore zones.</p>
      <a href="/" class="btn-primary">Back to Home</a>
    </div>
  {:else}
    <div class="card-neumorphic text-center py-16">
      <div class="neumorphic h-24 w-24 rounded-full mx-auto mb-6 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </div>
      <h2 class="text-2xl font-semibold dreamy-text mb-4">Zones Coming Soon!</h2>
      <p class="text-gray-600 mb-8 max-w-md mx-auto">
        We're working on bringing you the ability to join and create zones to connect with like-minded people in specific communities.
      </p>
      <a href="/users" class="btn-primary">Explore Users Instead</a>
    </div>
  {/if}
</div> 