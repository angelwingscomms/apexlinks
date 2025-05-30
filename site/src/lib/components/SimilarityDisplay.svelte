<script lang="ts">
  import type { SimilarityResult } from '../services/userService';
  
  export let similarity: SimilarityResult;
  export let loading = false;
  
  // Determine the color based on similarity score
  $: scoreColor = getScoreColor(similarity.similarity_score);
  
  function getScoreColor(score: number): string {
    if (score >= 0.8) return 'from-green-300 to-green-600';
    if (score >= 0.6) return 'from-teal-300 to-teal-600';
    if (score >= 0.4) return 'from-blue-300 to-blue-600';
    if (score >= 0.2) return 'from-purple-300 to-purple-600';
    return 'from-gray-300 to-gray-500';
  }
</script>

<div class="card-glass">
  <h3 class="text-xl font-semibold dreamy-text mb-4">Compatibility Analysis</h3>
  
  {#if loading}
    <div class="p-8 flex justify-center">
      <div class="animate-pulse text-center">
        <div class="glass h-16 w-16 rounded-full mx-auto mb-4"></div>
        <p class="text-gray-600">Analyzing similarity...</p>
      </div>
    </div>
  {:else}
    <!-- Overall Similarity Score -->
    <div class="mb-6">
      <div class="flex justify-between items-center mb-2">
        <span class="text-sm font-medium text-gray-700">Overall Similarity</span>
        <span class="text-lg font-bold">{Math.round(similarity.similarity_score * 100)}%</span>
      </div>
      <div class="bg-gray-200 h-3 rounded-full overflow-hidden">
        <div 
          class={`bg-gradient-to-r ${scoreColor} h-full rounded-full`} 
          style={`width: ${similarity.similarity_score * 100}%`}
        ></div>
      </div>
    </div>
    
    <!-- Common Interests -->
    {#if similarity.common_interests && similarity.common_interests.length > 0}
      <div class="mb-6">
        <h4 class="text-md font-semibold text-gray-800 mb-2">Common Interests</h4>
        <div class="flex flex-wrap gap-2">
          {#each similarity.common_interests as interest}
            <span class="glass-sm px-3 py-1 text-sm text-gray-700">{interest}</span>
          {/each}
        </div>
      </div>
    {:else}
      <div class="mb-6">
        <h4 class="text-md font-semibold text-gray-800 mb-2">Common Interests</h4>
        <p class="text-gray-500 italic">No common interests found</p>
      </div>
    {/if}
    
    <!-- Matching Attributes -->
    {#if similarity.matching_attributes && Object.keys(similarity.matching_attributes).length > 0}
      <div>
        <h4 class="text-md font-semibold text-gray-800 mb-2">Matching Attributes</h4>
        <ul class="space-y-2">
          {#each Object.entries(similarity.matching_attributes) as [key, value]}
            <li class="glass-sm p-2 rounded-lg flex justify-between">
              <span class="text-sm font-medium text-gray-700">{key}</span>
              <span class="text-sm">{value}</span>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  {/if}
</div> 