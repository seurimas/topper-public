<script lang="ts">
	import type { ActionDescriptor } from '$lib/planner/engine';

	export let activeClass = '';
	export let actions: ActionDescriptor[] = [];
	export let selectedAction = '';
	export let latestCommand = '';
	export let latestObservations: string[] = [];
	export let loading = false;
	export let onSelectedActionChange: (actionId: string) => void;
	export let onSendAction: () => void;
</script>

<section class="panel">
	<h2>Manual Action</h2>
	<label>
		{activeClass} action
		<select
			value={selectedAction}
			disabled={loading || actions.length === 0}
			on:change={(event) =>
				onSelectedActionChange((event.currentTarget as HTMLSelectElement).value)}
		>
			{#each actions as action}
				<option value={action.id}>
					{action.label}{action.targeted ? ' (targeted -> Target)' : ' (untargeted)'}
				</option>
			{/each}
		</select>
	</label>
	<button on:click={onSendAction} disabled={loading || !selectedAction}>
		{#if loading}Working...{:else}Send Action{/if}
	</button>

	{#if latestCommand}
		<p><strong>Last command:</strong> {latestCommand}</p>
	{/if}

	{#if latestObservations.length > 0}
		<h3>Last Simulated Observations</h3>
		<ul>
			{#each latestObservations as observation}
				<li>{observation}</li>
			{/each}
		</ul>
	{/if}
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

	button {
		@apply w-fit rounded-md bg-amber-500 px-4 py-2 font-semibold text-black hover:bg-amber-400 disabled:opacity-60;
	}

	ul {
		@apply list-disc space-y-1 pl-5 text-sm;
	}

	h3 {
		@apply text-lg font-semibold;
	}
</style>
