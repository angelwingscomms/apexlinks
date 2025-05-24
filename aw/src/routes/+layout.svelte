<script lang="ts">
	import '../app.css';
	import Header from '$lib/Header.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>AngelWings Comprehensive College - Primary & Secondary Education Excellence</title>
	<meta name="description" content="AngelWings Comprehensive College - Nurturing young minds through comprehensive primary and secondary education. Discover exceptional learning opportunities for foundation through grade 5." />
</svelte:head>

<div class="min-h-screen">
	<Header />
	{@render children()}
</div>
