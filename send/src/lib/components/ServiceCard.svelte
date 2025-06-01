<script>
  import RatingStars from './RatingStars.svelte';
  
  export let service = {
    id: '',
    name: '',
    rating: 0,
    image: '',
    description: '',
    price: null,
    location: ''
  };
</script>

<div class="card mb-4 transition-transform motion-safe:hover:scale-[1.02]">
  <div class="flex flex-col md:flex-row items-start md:items-center gap-4 p-4">
    <div class="w-16 h-16 rounded bg-gradient-dark shrink-0"></div>
    <div class="flex-1">
      <h3 class="text-lg font-medium">{service.name}</h3>
      <RatingStars rating={service.rating} />
      
      {#if service.description}
        <p class="text-secondary text-sm mt-1">{service.description}</p>
      {/if}
      
      <div class="flex flex-wrap gap-x-4 gap-y-1 mt-2 text-xs text-secondary">
        {#if service.price}
          <span class="flex items-center">
            <svg class="w-3 h-3 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.31-8.86c-1.77-.45-2.34-.94-2.34-1.67 0-.84.79-1.43 2.1-1.43 1.38 0 1.9.66 1.94 1.64h1.71c-.05-1.34-.87-2.57-2.49-2.97V5H10.9v1.69c-1.51.32-2.72 1.3-2.72 2.81 0 1.79 1.49 2.69 3.66 3.21 1.95.46 2.34 1.15 2.34 1.87 0 .53-.39 1.39-2.1 1.39-1.6 0-2.23-.72-2.32-1.64H8.04c.1 1.7 1.36 2.66 2.86 2.97V19h2.34v-1.67c1.52-.29 2.72-1.16 2.73-2.77-.01-2.2-1.9-2.96-3.66-3.42z" fill="currentColor"/>
            </svg>
            ${service.price}
          </span>
        {/if}
        
        {#if service.location}
          <span class="flex items-center">
            <svg class="w-3 h-3 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z" fill="currentColor"/>
            </svg>
            {service.location}
          </span>
        {/if}
      </div>
    </div>
    <a href="/service/i?i={service.id}" class="btn">Request</a>
  </div>
</div> 