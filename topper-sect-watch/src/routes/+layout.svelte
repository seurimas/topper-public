<script lang="ts">
	import "../app.css";
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from "$lib/Navbar.svelte";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>


<Navbar />
{@render children()}

<style>
	@reference "tailwindcss";

	:global(body) {
		@apply bg-gray-900 text-white;
	}
</style>