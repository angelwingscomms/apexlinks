<script lang="ts">
  import type { Service } from '$lib/services/serviceService';
  
  export let service: Service;
</script>

<div class="card-glass transition-all hover:scale-[1.01] mb-6">
  <div class="flex flex-col md:flex-row gap-4 p-6">
    <div class="w-20 h-20 rounded-xl glass-sm shrink-0 flex items-center justify-center text-3xl">
      {#if service.type === 'mechanics'}
        🔧
      {:else if service.type === 'towing'}
        🚗
      {:else if service.type === 'cleaning'}
        ✨
      {:else if service.type === 'delivery'}
        📦
      {:else}
        🏢
      {/if}
    </div>
    
    <div class="flex-1">
      <h3 class="text-xl font-semibold dreamy-text mb-1">{service.name}</h3>
      
      <div class="flex items-center mb-2">
        <div class="flex">
          {#each Array(5) as _, i}
            <svg class="w-4 h-4 {i < service.rating ? 'text-yellow-500' : 'text-gray-300'}" 
              fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
              <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path>
            </svg>
          {/each}
        </div>
        <span class="ml-2 text-sm text-gray-600">{service.rating.toFixed(1)}</span>
      </div>
      
      {#if service.description}
        <p class="text-gray-600 mb-3">{service.description}</p>
      {/if}
      
      <div class="flex flex-wrap gap-3 text-sm text-gray-600">
        {#if service.price !== undefined && service.price !== null}
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.31-8.86c-1.77-.45-2.34-.94-2.34-1.67 0-.84.79-1.43 2.1-1.43 1.38 0 1.9.66 1.94 1.64h1.71c-.05-1.34-.87-2.57-2.49-2.97V5H10.9v1.69c-1.51.32-2.72 1.3-2.72 2.81 0 1.79 1.49 2.69 3.66 3.21 1.95.46 2.34 1.15 2.34 1.87 0 .53-.39 1.39-2.1 1.39-1.6 0-2.23-.72-2.32-1.64H8.04c.1 1.7 1.36 2.66 2.86 2.97V19h2.34v-1.67c1.52-.29 2.72-1.16 2.73-2.77-.01-2.2-1.9-2.96-3.66-3.42z" fill="currentColor"/>
            </svg>
            ${service.price}
          </span>
        {/if}
        
        {#if service.location}
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z" fill="currentColor"/>
            </svg>
            {service.location}
          </span>
        {/if}
        
        {#if service.type}
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2l-5.5 9h11L12 2zm0 3.84L13.93 9h-3.87L12 5.84zM17.5 13c-2.49 0-4.5 2.01-4.5 4.5s2.01 4.5 4.5 4.5 4.5-2.01 4.5-4.5-2.01-4.5-4.5-4.5zm0 7c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5zM3 21.5h8v-8H3v8zm2-6h4v4H5v-4z" fill="currentColor"/>
            </svg>
            {service.type}
          </span>
        {/if}
      </div>
    </div>
    
    <div class="flex md:flex-col gap-2 md:items-end justify-end">
      <a href={`/service/i?i=${service.id}`} class="btn-primary">
        View Details
      </a>
      <button class="btn">Request</button>
    </div>
  </div>
</div> 