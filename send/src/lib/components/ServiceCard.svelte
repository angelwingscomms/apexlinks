<script>
  import RatingStars from './RatingStars.svelte';
  
  export let service = {
    id: '',
    name: '',
    rating: 0,
    image: ''
  };
</script>

<div class="card mb-4 transition-transform motion-safe:hover:scale-[1.02]">
  <div class="flex items-center gap-4">
    <div class="w-16 h-16 rounded bg-gradient-dark"></div>
    <div class="flex-1">
      <h3 class="text-lg">{service.name}</h3>
      <RatingStars rating={service.rating} />
    </div>
    <a href="/service/{service.id}" class="btn">Request</a>
  </div>
</div> 