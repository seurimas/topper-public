<script lang="ts">
	export let battleStats: unknown = null;

	$: battleStatsPretty = battleStats ? JSON.stringify(battleStats, null, 2) : '{}';
</script>

<section class="panel">
	<h2>Battle Stats (JSON)</h2>
	<pre>{battleStatsPretty}</pre>
</section>

<style>
	@reference "tailwindcss";

	.panel {
		@apply mt-5 grid gap-3 rounded-xl border border-gray-700 bg-gray-800/90 p-4;
	}

	pre {
		@apply max-h-80 overflow-auto rounded-md border border-gray-700 bg-gray-900 p-3 text-xs;
	}
</style>
