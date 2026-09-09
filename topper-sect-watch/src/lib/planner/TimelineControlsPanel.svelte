<script lang="ts">
	export let loading = false;
	export let onAdvanceBy: (deltaMs: number) => void;
	export let onAdvanceToQeb: () => void;

	let customDeltaMs = 250;
</script>

<section class="panel">
	<h2>Timeline Controls</h2>
	<div class="button-row">
		<button on:click={() => onAdvanceBy(10)} disabled={loading}>+10 ms</button>
		<button on:click={() => onAdvanceBy(25)} disabled={loading}>+25 ms</button>
		<button on:click={() => onAdvanceBy(50)} disabled={loading}>+50 ms</button>
		<button on:click={() => onAdvanceBy(100)} disabled={loading}>+100 ms</button>
		<button on:click={onAdvanceToQeb} disabled={loading}>Advance to QEB</button>
	</div>

	<div class="custom-row">
		<label for="custom-delta">Custom ms</label>
		<input
			id="custom-delta"
			type="number"
			min="1"
			step="1"
			bind:value={customDeltaMs}
			disabled={loading}
		/>
		<button
			on:click={() => onAdvanceBy(Math.max(1, Math.floor(customDeltaMs || 1)))}
			disabled={loading}
		>
			Advance by custom
		</button>
	</div>
</section>

<style>
	@reference "tailwindcss";

	.panel {
		@apply mt-5 grid gap-3 rounded-xl border border-gray-700 bg-gray-800/90 p-4;
	}

	.button-row {
		@apply flex flex-wrap gap-2;
	}

	.custom-row {
		@apply flex flex-wrap items-end gap-2;
	}

	label {
		@apply text-sm;
	}

	input {
		@apply rounded-md border border-gray-600 bg-gray-900 px-3 py-2;
	}

	button {
		@apply rounded-md bg-sky-600 px-3 py-2 text-sm font-semibold text-white hover:bg-sky-500 disabled:opacity-60;
	}
</style>
