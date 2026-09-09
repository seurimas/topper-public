<script lang="ts">
	import type { PassiveDescriptor } from '$lib/planner/engine';

	export let classes: string[] = [];
	export let activeClass = '';
	export let passives: PassiveDescriptor[] = [];
	export let loading = false;
	export let onClassChange: (className: string) => void;
</script>

<section class="panel">
	<h2>Setup</h2>
	<p>
		Active player: <strong>Planner</strong>. Target player: <strong>Target</strong>.
	</p>
	<label>
		Class
		<select
			value={activeClass}
			on:change={(event) => onClassChange((event.currentTarget as HTMLSelectElement).value)}
			disabled={loading}
		>
			{#each classes as className}
				<option value={className}>{className}</option>
			{/each}
		</select>
	</label>

	<h3>Passive Actions</h3>
	<ul>
		{#each passives as passive}
			<li>{passive.label} every {passive.period_ms} ms</li>
		{/each}
	</ul>
</section>

<style>
	@reference "tailwindcss";

	.panel {
		@apply mt-5 grid gap-3 rounded-xl border border-gray-700 bg-gray-800/90 p-4;
	}

	label {
		@apply grid gap-1 text-sm;
	}

	select {
		@apply rounded-md border border-gray-600 bg-gray-900 px-3 py-2;
	}

	h3 {
		@apply mt-1 text-lg font-semibold;
	}

	ul {
		@apply list-disc space-y-1 pl-5 text-sm;
	}
</style>
