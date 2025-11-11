<script lang="ts">
    let { title, children } = $props();
</script>
<div role="main" class="font-sans antialiased p-6 leading-normal tracking-wider bg-cover">
	<h1>{title}</h1>
    {@render children()}
</div>

<style>
	@reference "tailwindcss";

	h1 {
		@apply text-4xl font-bold mb-4;
	}
</style>