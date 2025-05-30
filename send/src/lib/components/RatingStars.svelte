<script>
  export let rating = 0;
  export let showValue = true;
</script>

<div class="rating">
  {#each Array(5) as _, i}
    <span>{i < rating ? '★' : '☆'}</span>
  {/each}
  {#if showValue}
    <span class="text-secondary text-sm ml-1">{rating}</span>
  {/if}
</div> 