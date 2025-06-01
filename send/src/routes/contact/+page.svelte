<script lang="ts">
  let name = $state('');
  let email = $state('');
  let message = $state('');
  let submitted = $state(false);
  
  const handleSubmit = () => {
    // In a real app, we would send this to an API
    console.log('Form submitted:', { name, email, message });
    submitted = true;
    // Reset form
    name = '';
    email = '';
    message = '';
  };
</script>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold mb-8">Contact Us</h1>
  
  {#if submitted}
    <div class="neumorphic p-6 mb-6">
      <h2 class="text-xl font-bold text-primary mb-4">Thank You!</h2>
      <p class="mb-4">Your message has been received. We'll get back to you soon.</p>
      <button 
        onclick={() => submitted = false}
        class="btn"
      >
        Send Another Message
      </button>
    </div>
  {:else}
    <div class="neumorphic p-6 mb-6">
      <form class="space-y-6">
        <div>
          <label for="name" class="block text-sm font-medium mb-2">Your Name</label>
          <input
            type="text"
            id="name"
            bind:value={name}
            required
            class="w-full p-3 bg-gradient-dark rounded-lg neumorphic"
          />
        </div>
        
        <div>
          <label for="email" class="block text-sm font-medium mb-2">Email Address</label>
          <input
            type="email"
            id="email"
            bind:value={email}
            required
            class="w-full p-3 bg-gradient-dark rounded-lg neumorphic"
          />
        </div>
        
        <div>
          <label for="message" class="block text-sm font-medium mb-2">Message</label>
          <textarea
            id="message"
            bind:value={message}
            rows="5"
            required
            class="w-full p-3 bg-gradient-dark rounded-lg neumorphic"
          ></textarea>
        </div>
        
        <button
          type="button"
          onclick={handleSubmit}
          class="btn w-full md:w-auto"
        >
          Send Message
        </button>
      </form>
    </div>
  {/if}
  
  <div class="neumorphic p-6">
    <h2 class="text-xl font-bold mb-4">Other Ways to Reach Us</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div>
        <h3 class="text-lg font-semibold mb-2">Email</h3>
        <p class="text-secondary">support@sendme.example.com</p>
      </div>
      
      <div>
        <h3 class="text-lg font-semibold mb-2">Phone</h3>
        <p class="text-secondary">(555) 123-4567</p>
      </div>
      
      <div>
        <h3 class="text-lg font-semibold mb-2">Office</h3>
        <p class="text-secondary">123 Main Street<br />San Francisco, CA 94105</p>
      </div>
      
      <div>
        <h3 class="text-lg font-semibold mb-2">Hours</h3>
        <p class="text-secondary">Monday - Friday: 9am - 5pm<br />Saturday: 10am - 2pm</p>
      </div>
    </div>
  </div>
</div> 