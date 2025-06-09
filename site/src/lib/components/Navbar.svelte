<script lang="ts">
  import { userStore, isAuthenticated, currentUser } from '../stores/userStore';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  
  // Handle navigation bar animation
  let scrolled = $state(false);
  let prevScrollPos = 0;
  let visible = $state(true);
  
  onMount(() => {
    userStore.initialize();
    
    const handleScroll = () => {
      const currentScrollPos = window.scrollY;
      
      // Auto-hide navbar on scroll down and show on scroll up
      visible = prevScrollPos > currentScrollPos || currentScrollPos < 60;
      prevScrollPos = currentScrollPos;
      
      // Change navbar appearance on scroll
      scrolled = currentScrollPos > 20;
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

<nav class={`fixed top-0 left-0 right-0 z-50 transition-all duration-500 ${visible ? 'translate-y-0' : '-translate-y-full'} ${scrolled ? 'navbar-scrolled' : 'navbar-top'}`}>
  <div class="h-[1px] w-full bg-gradient-to-r from-transparent via-[#0a84ff]/10 to-transparent"></div>
  <div class="container mx-auto flex justify-between items-center px-6 py-4">
    <!-- Logo -->
    <div>
      <a href="/" class="flex items-center group">
        <span class="text-gradient-logo text-2xl font-medium tracking-tight">ApexLinks</span>
        <span class="transition-all duration-300 scale-x-0 group-hover:scale-x-100 origin-left h-[2px] w-full bg-gradient-to-r from-[#0a84ff] to-[#54c7fc] mt-0.5"></span>
      </a>
    </div>
    
    <!-- Navigation Links -->
    <div class="hidden md:flex space-x-10">
      <a href="/users" class={`nav-link ${$page.url.pathname === '/users' ? 'nav-link-active' : ''}`}>
        Find Users
      </a>
      <a href="/about" class={`nav-link ${$page.url.pathname === '/about' ? 'nav-link-active' : ''}`}>
        About
      </a>
    </div>
    
    <!-- Auth Buttons -->
    <div>
      {#if $isAuthenticated && $currentUser}
        <div class="flex items-center space-x-5">
          <!-- Chat Notification Badge -->
          <a href="/chat" class="relative nav-icon-button" aria-label="Chat messages">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 19c3.771 0 5.657 0 6.828-1.172C22 16.657 22 14.771 22 11c0-3.771 0-5.657-1.172-6.828C19.657 3 17.771 3 14 3h-4C6.229 3 4.343 3 3.172 4.172 2 5.343 2 7.229 2 11c0 3.771 0 5.657 1.172 6.828.653.654 1.528.943 2.828 1.07"></path><path d="M14 19c-1.236 0-2.598.5-3.841 1.145-1.998 1.037-2.997 1.556-3.489 1.225-.492-.33-.399-1.355-.212-3.404L6.5 17.5"></path></svg>
            <span class="absolute -top-1 -right-1 bg-red-500 text-white text-xs h-5 w-5 flex items-center justify-center rounded-full">
              <!-- Add notification count here -->
            </span>
          </a>
          
          <div class="neumorphic-sm flex items-center px-3 py-2 cursor-pointer transition-all hover:translate-y-[-2px]" on:click={goToProfile}>
            <img src={$currentUser.picture} alt="Profile" class="w-8 h-8 rounded-full mr-2" />
            <span class="text-sm font-medium text-[#1d1d1f]">{$currentUser.name}</span>
          </div>
          
          <button 
            on:click={handleLogout}
            class="nav-button-secondary"
            aria-label="Logout"
          >
            Logout
          </button>
        </div>
      {:else}
        <button 
          on:click={handleLogin}
          class="nav-button-primary"
        >
          Login with Google
        </button>
      {/if}
    </div>
  </div>
</nav>

<!-- Spacer to prevent content from being hidden under the navbar -->
<div class="h-20"></div>

<style>
  .navbar-top {
    background: rgba(245, 245, 247, 0.5);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid rgba(0, 0, 0, 0.02);
  }
  
  .navbar-scrolled {
    background: rgba(245, 245, 247, 0.8);
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba(0, 0, 0, 0.06);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.03);
  }
  
  .text-gradient-logo {
    background: linear-gradient(90deg, #0a84ff, #54c7fc);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
  
  .nav-link {
    position: relative;
    color: #1d1d1f;
    font-weight: 450;
    transition: all 0.3s ease;
    padding-bottom: 2px;
  }
  
  .nav-link::after {
    content: '';
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 0;
    height: 2px;
    background: linear-gradient(90deg, #0a84ff, #54c7fc);
    transition: width 0.3s ease;
  }
  
  .nav-link:hover::after {
    width: 100%;
  }
  
  .nav-link-active {
    font-weight: 500;
  }
  
  .nav-link-active::after {
    width: 100%;
  }
  
  .nav-button-primary {
    background-color: #0a84ff;
    color: white;
    padding: 0.6rem 1.25rem;
    border-radius: 1.25rem;
    font-weight: 500;
    transition: all 0.3s ease;
    border: none;
    box-shadow: 0 4px 10px rgba(10, 132, 255, 0.2);
  }
  
  .nav-button-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 15px rgba(10, 132, 255, 0.3);
    background-color: #0071e3;
  }
  
  .nav-button-secondary {
    background-color: rgba(0, 0, 0, 0.05);
    color: #1d1d1f;
    padding: 0.6rem 1.25rem;
    border-radius: 1.25rem;
    font-weight: 500;
    transition: all 0.3s ease;
    border: 1px solid rgba(0, 0, 0, 0.05);
  }
  
  .nav-button-secondary:hover {
    background-color: rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }
  
  .nav-icon-button {
    background-color: rgba(0, 0, 0, 0.05);
    color: #1d1d1f;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: all 0.3s ease;
  }
  
  .nav-icon-button:hover {
    background-color: rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }
  
  .neumorphic-sm {
    background: #e8e8ed;
    border-radius: 1rem;
    box-shadow: 
      4px 4px 8px rgba(0, 0, 0, 0.03),
      -4px -4px 8px rgba(255, 255, 255, 0.5);
  }
</style> 