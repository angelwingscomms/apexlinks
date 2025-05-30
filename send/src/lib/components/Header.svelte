<script>
  export let activeTab = 'home';
</script>

<header class="neumorphic p-4 mb-8">
  <div class="flex justify-between items-center">
    <div class="text-2xl font-bold">SendMe</div>
    <nav class="flex gap-4">
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
    </nav>
  </div>
</header> 