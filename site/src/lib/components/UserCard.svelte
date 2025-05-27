<script lang="ts">
  import type { UserSearchResult } from '../services/userService';
  import { currentUser } from '../stores/userStore';
  
  export let user: UserSearchResult;
  export let showSimilarity = false;
  export let similarityScore: number | null = null;
  
  $: isCurrentUser = $currentUser && $currentUser.id === user.id;
</script>

<div class="card-neumorphic mb-6 w-full max-w-md hover:scale-102 transition-transform" style="animation: var(--animate-float);">
  <div class="flex items-start gap-4">
    <!-- Profile Image -->
    <div class="neumorphic-inset h-24 w-24 rounded-full overflow-hidden flex items-center justify-center">
      {#if user.email}
        <img 
          src={`https://ui-avatars.com/api/?name=${user.name || 'User'}&background=0ea5e9&color=fff&size=128`} 
          alt={user.name || 'User'} 
          class="h-full w-full object-cover"
        />
      {:else}
        <div class="h-full w-full flex items-center justify-center bg-gradient-to-br from-blue-400 to-blue-600 text-white text-xl font-bold">
          {(user.name || 'U').charAt(0).toUpperCase()}
        </div>
      {/if}
    </div>
    
    <!-- User Info -->
    <div class="flex-1">
      <h3 class="text-xl font-semibold dreamy-text">{user.name || 'Anonymous User'}</h3>
      
      {#if user.username}
        <p class="text-gray-600 text-sm">@{user.username}</p>
      {/if}
      
      {#if user.description}
        <p class="mt-2 text-gray-700">{user.description}</p>
      {:else}
        <p class="mt-2 text-gray-500 italic">No description available</p>
      {/if}
      
      {#if showSimilarity && similarityScore !== null}
        <div class="mt-3 neumorphic-inset p-2 rounded-lg">
          <div class="flex items-center">
            <span class="text-sm font-medium text-gray-700 mr-2">Similarity:</span>
            <div class="bg-gray-200 h-2 flex-1 rounded-full overflow-hidden">
              <div 
                class="bg-gradient-to-r from-blue-300 to-blue-600 h-full rounded-full" 
                style={`width: ${similarityScore * 100}%`}
              ></div>
            </div>
            <span class="ml-2 text-sm font-semibold">{Math.round(similarityScore * 100)}%</span>
          </div>
        </div>
      {/if}
    </div>
  </div>
  
  <!-- Action Buttons -->
  <div class="mt-4 flex justify-end gap-2">
    <a 
      href={`/users/${user.id}`}
      class="neumorphic-button px-4 py-2 text-sm text-primary font-medium"
    >
      View Profile
    </a>
    
    {#if isCurrentUser}
      <a 
        href="/profile/edit"
        class="neumorphic-button px-4 py-2 text-sm text-gray-600"
      >
        Edit Profile
      </a>
    {/if}
  </div>
</div> 