<script lang="ts">
  import { userStore, isAuthenticated, currentUser } from '../stores/userStore';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  
  // Handle navigation bar animation
  let scrolled = $state(false);
  
  onMount(() => {
    userStore.initialize();
    
    const handleScroll = () => {
      scrolled = window.scrollY > 20;
    };
    
    window.addEventListener('scroll', handleScroll);
    
    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  });
  
  // Handle login
  function handleLogin() {
    userStore.startGoogleAuth();
  }
  
  // Handle logout
  function handleLogout() {
    userStore.logout();
  }
  
  // Navigate to profile page
  function goToProfile() {
    goto('/profile');
  }
</script>

<nav class={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 p-4 ${scrolled ? 'bg-white/30 backdrop-blur-md shadow-md' : 'bg-transparent'}`}>
  <div class="container mx-auto flex justify-between items-center">
    <!-- Logo -->
    <div>
      <a href="/" class="flex items-center">
        <span class="self-center text-2xl font-semibold whitespace-nowrap dreamy-text">ApexLinks</span>
      </a>
    </div>
    
    <!-- Navigation Links -->
    <div class="hidden md:flex space-x-8">
      <a href="/users" class={`dreamy-text font-medium ${$page.url.pathname === '/users' ? 'border-b-2 border-primary' : ''}`}>Find Users</a>
    </div>
    
    <!-- Auth Buttons -->
    <div>
      {#if $isAuthenticated && $currentUser}
        <div class="flex items-center space-x-4">
          <!-- Chat Notification Badge (add real-time notification indicator here later) -->
          <a href="/chat" class="relative glass-button px-3 py-2 text-gray-700">
            <span class="text-lg">💬</span>
            <span class="absolute -top-1 -right-1 bg-red-500 text-white text-xs h-5 w-5 flex items-center justify-center rounded-full">
              <!-- Add notification count here -->
            </span>
          </a>
          
          <div class="glass-sm flex items-center px-3 py-2 cursor-pointer" on:click={goToProfile}>
            <img src={$currentUser.picture} alt="Profile" class="w-8 h-8 rounded-full mr-2" />
            <span class="text-sm font-medium text-gray-700">{$currentUser.name}</span>
          </div>
          <button 
            on:click={handleLogout}
            class="glass-button px-4 py-2 text-sm text-gray-700"
          >
            Logout
          </button>
        </div>
      {:else}
        <button 
          on:click={handleLogin}
          class="btn-primary"
        >
          Login with Google
        </button>
      {/if}
    </div>
  </div>
</nav>

<!-- Spacer to prevent content from being hidden under the navbar -->
<div class="h-20"></div> 