<script lang="ts">
  import { userStore, isAuthenticated, currentUser } from '../stores/userStore';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  
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
</script>

<nav class={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 p-4 ${scrolled ? 'bg-white/90 backdrop-blur-md shadow-md' : 'bg-transparent'}`}>
  <div class="container mx-auto flex justify-between items-center">
    <!-- Logo -->
    <div>
      <a href="/" class="dreamy-text text-2xl font-bold tracking-wider transition-all duration-300 hover:scale-105">
        ApexLinks
      </a>
    </div>
    
    <!-- Navigation Links -->
    <div class="hidden md:flex space-x-8">
      <a href="/" class={`dreamy-text font-medium ${$page.url.pathname === '/' ? 'border-b-2 border-primary' : ''}`}>Home</a>
      <a href="/users" class={`dreamy-text font-medium ${$page.url.pathname === '/users' ? 'border-b-2 border-primary' : ''}`}>Find Users</a>
      {#if $isAuthenticated}
        <a href="/profile" class={`dreamy-text font-medium ${$page.url.pathname === '/profile' ? 'border-b-2 border-primary' : ''}`}>My Profile</a>
      {/if}
    </div>
    
    <!-- Auth Buttons -->
    <div>
      {#if $isAuthenticated && $currentUser}
        <div class="flex items-center space-x-4">
          <div class="neumorphic flex items-center px-3 py-2">
            <img src={$currentUser.picture} alt="Profile" class="w-8 h-8 rounded-full mr-2" />
            <span class="text-sm font-medium text-gray-700">{$currentUser.name}</span>
          </div>
          <button 
            on:click={handleLogout}
            class="neumorphic-button px-4 py-2 text-sm text-gray-700"
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