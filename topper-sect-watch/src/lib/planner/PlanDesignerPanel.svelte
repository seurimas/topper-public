<script lang="ts">
	import type { PlannerStep } from '$lib/planner/types';

	export let loading = false;
	export let selectedActionLabel = '';
	export let selectedActionId = '';
	export let steps: PlannerStep[] = [];
	export let onAddStep: (offsetMs: number) => void;
	export let onRemoveStep: (index: number) => void;
	export let onClearSteps: () => void;

	let nextOffsetMs = 250;

	$: planPreview = JSON.stringify(steps, null, 2);
</script>

<section class="panel">
	<h2>Plan Designer (Alpha)</h2>
	<p>
		Save manual action steps with a timeline offset. This is intentionally manual and excludes
		automated learning.
	</p>

	<div class="add-row">
		<div class="action-pill">
			{#if selectedActionLabel}
				Next step: {selectedActionLabel}
			{:else}
				Select an action to add a step
			{/if}
		</div>
		<label for="planner-offset">Offset ms</label>
		<input
			id="planner-offset"
			type="number"
			min="1"
			step="1"
			bind:value={nextOffsetMs}
			disabled={loading}
		/>
		<button
			on:click={() => onAddStep(Math.max(1, Math.floor(nextOffsetMs || 1)))}
			disabled={loading || !selectedActionId}
		>
			Add Step
		</button>
		<button on:click={onClearSteps} disabled={loading || steps.length === 0}> Clear Plan </button>
	</div>

	{#if steps.length > 0}
		<ol>
			{#each steps as step, idx}
				<li>
					<span>+{step.offsetMs} ms: {step.actionLabel}</span>
					<button on:click={() => onRemoveStep(idx)} disabled={loading}> Remove </button>
				</li>
			{/each}
		</ol>
		<h3>Plan JSON</h3>
		<pre>{planPreview}</pre>
	{/if}
</section>

<style>
	@reference "tailwindcss";

	.panel {
		@apply mt-5 grid gap-3 rounded-xl border border-gray-700 bg-gray-800/90 p-4;
	}

	.add-row {
		@apply flex flex-wrap items-end gap-2;
	}

	.action-pill {
		@apply rounded-md border border-amber-400/50 bg-amber-200/10 px-3 py-2 text-sm text-amber-100;
	}

	label {
		@apply text-sm;
	}

	input {
		@apply rounded-md border border-gray-600 bg-gray-900 px-3 py-2;
	}

	button {
		@apply rounded-md bg-emerald-600 px-3 py-2 text-sm font-semibold text-white hover:bg-emerald-500 disabled:opacity-60;
	}

	ol {
		@apply list-decimal space-y-2 pl-5;
	}

	li {
		@apply flex flex-wrap items-center justify-between gap-2 rounded-md border border-gray-700 bg-gray-900/60 px-3 py-2;
	}

	h3 {
		@apply mt-2 text-lg font-semibold;
	}

	pre {
		@apply max-h-80 overflow-auto rounded-md border border-gray-700 bg-gray-900 p-3 text-xs;
	}
</style>
