<script lang="ts">
  import { isAuthenticated, googleAuth, clearToken } from '$lib/api';
  let { activeTab = 'home' } = $props();
  
  let authenticated = $state(false);
  let searchQuery = $state('');
  
  $effect(() => {
    authenticated = isAuthenticated();
  });
  
  const handleAuth = () => {
    if (authenticated) {
      clearToken();
      authenticated = false;
    } else {
      googleAuth();
    }
  };

  const handleSearch = (e: Event) => {
    const isEnterKey = e instanceof KeyboardEvent && e.key === 'Enter';
    const isClick = e.type === 'click';
    
    if (isEnterKey || isClick) {
      if (searchQuery.trim()) {
        window.location.href = `/search?q=${encodeURIComponent(searchQuery)}`;
      }
    }
  };
</script>

<header class="neumorphic p-4 mb-8">
  <div class="flex justify-between items-center">
    <div class="text-2xl font-bold">SendMe</div>
    
    <div class="flex-1 mx-auto max-w-md px-4">
      <div class="relative">
        <input
          type="text"
          placeholder="Search services..."
          bind:value={searchQuery}
          onkeydown={handleSearch}
          class="w-full bg-gradient-dark text-primary p-2 rounded-full neumorphic"
          aria-label="Search services"
        />
        <button 
          onclick={handleSearch}
          class="absolute right-2 top-1/2 transform -translate-y-1/2"
          aria-label="Submit search"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
          </svg>
        </button>
      </div>
    </div>
    
    <nav class="flex gap-4 items-center">
      <a 
        href="/" 
        class="transition-all hover:text-primary {activeTab === 'home' ? 'text-primary' : 'text-secondary'}"
        aria-current={activeTab === 'home' ? 'page' : undefined}
      >
        Services
      </a>
      <a 
        href="/contact" 
        class="transition-all hover:text-primary {activeTab === 'contact' ? 'text-primary' : 'text-secondary'}"
        aria-current={activeTab === 'contact' ? 'page' : undefined}
      >
        Contact
      </a>
      <a 
        href="/more" 
        class="transition-all hover:text-primary {activeTab === 'more' ? 'text-primary' : 'text-secondary'}"
        aria-current={activeTab === 'more' ? 'page' : undefined}
      >
        More
      </a>
      <button 
        onclick={handleAuth}
        class="pill gradient-pill text-primary text-sm motion-safe:hover:scale-105"
      >
        {authenticated ? 'Sign Out' : 'Sign In'}
      </button>
    </nav>
  </div>
</header> 