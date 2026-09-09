<script lang="ts">
	import { onMount } from 'svelte';
	import BattleStatsPanel from '$lib/planner/BattleStatsPanel.svelte';
	import ManualActionPanel from '$lib/planner/ManualActionPanel.svelte';
	import PlanDesignerPanel from '$lib/planner/PlanDesignerPanel.svelte';
	import type { PlannerStep } from '$lib/planner/types';
	import SetupPanel from '$lib/planner/SetupPanel.svelte';
	import TimelineControlsPanel from '$lib/planner/TimelineControlsPanel.svelte';
	import {
		advanceTimelineBy,
		advanceTimelineToQeb,
		getActions,
		getPassives,
		getState,
		getSupportedClasses,
		setActiveClass,
		simulateAction,
		type ActionDescriptor,
		type PassiveDescriptor
	} from '$lib/planner/engine';

	let classes: string[] = [];
	let activeClass = 'Ascendril';
	let actions: ActionDescriptor[] = [];
	let passives: PassiveDescriptor[] = [];
	let selectedAction = '';
	let battleStats: unknown = null;
	let latestObservations: string[] = [];
	let latestCommand = '';
	let loading = false;
	let error = '';
	let planSteps: PlannerStep[] = [];

	$: selectedActionLabel = actions.find((action) => action.id === selectedAction)?.label ?? '';

	async function initialize() {
		loading = true;
		error = '';

		try {
			classes = await getSupportedClasses();
			const state = await getState();
			activeClass = state.active_class;
			battleStats = state.battle_stats;
			actions = await getActions();
			passives = await getPassives();
			selectedAction = actions[0]?.id ?? '';
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function changeClass() {
		loading = true;
		error = '';

		try {
			const state = await setActiveClass(activeClass);
			battleStats = state.battle_stats;
			actions = await getActions();
			passives = await getPassives();
			selectedAction = actions[0]?.id ?? '';
			latestObservations = [];
			latestCommand = '';
			planSteps = [];
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function handleClassChange(className: string) {
		activeClass = className;
		await changeClass();
	}

	function setSelectedAction(actionId: string) {
		selectedAction = actionId;
	}

	async function sendAction() {
		if (!selectedAction) {
			return;
		}
		loading = true;
		error = '';
		try {
			const result = await simulateAction(selectedAction);
			latestCommand = result.command;
			latestObservations = result.observations;
			battleStats = result.battle_stats;
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function advanceBy(deltaMs: number) {
		loading = true;
		error = '';
		try {
			const state = await advanceTimelineBy(deltaMs);
			battleStats = state.battle_stats;
			latestObservations = [];
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function advanceToQeb() {
		loading = true;
		error = '';
		try {
			const state = await advanceTimelineToQeb();
			battleStats = state.battle_stats;
			latestObservations = [];
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	function addPlanStep(offsetMs: number) {
		if (!selectedAction) {
			return;
		}
		planSteps = [
			...planSteps,
			{
				actionId: selectedAction,
				actionLabel: selectedActionLabel,
				offsetMs
			}
		];
	}

	function removePlanStep(index: number) {
		planSteps = planSteps.filter((_, idx) => idx !== index);
	}

	function clearPlanSteps() {
		planSteps = [];
	}

	onMount(async () => {
		await initialize();
	});
</script>

<svelte:head>
	<title>Sect Watch - Planner (Alpha)</title>
</svelte:head>

<main class="mx-auto max-w-5xl px-4 py-8 sm:px-6 lg:px-8">
	<header class="rounded-xl border border-amber-400/50 bg-amber-200/10 p-4">
		<p class="alpha">Alpha Feature</p>
		<h1>Planner</h1>
		<p>
			Manually design and simulate a winning sequence of actions. Planner is focused on intentional
			control and timeline planning.
		</p>
		<p>
			Defensive first-aid curing remains in the roadmap. Automated learning is not part of this
			feature direction.
		</p>
	</header>

	<SetupPanel {classes} {activeClass} {passives} {loading} onClassChange={handleClassChange} />

	<ManualActionPanel
		{activeClass}
		{actions}
		{selectedAction}
		{latestCommand}
		{latestObservations}
		{loading}
		onSelectedActionChange={setSelectedAction}
		onSendAction={sendAction}
	/>

	<PlanDesignerPanel
		{loading}
		{selectedActionLabel}
		selectedActionId={selectedAction}
		steps={planSteps}
		onAddStep={addPlanStep}
		onRemoveStep={removePlanStep}
		onClearSteps={clearPlanSteps}
	/>

	<TimelineControlsPanel {loading} onAdvanceBy={advanceBy} onAdvanceToQeb={advanceToQeb} />

	<BattleStatsPanel {battleStats} />

	{#if error}
		<pre class="error">{error}</pre>
	{/if}
</main>

<style>
	@reference "tailwindcss";

	h1 {
		@apply mb-2 text-3xl font-bold;
	}

	p {
		@apply mb-2;
	}

	.alpha {
		@apply inline-block rounded-full border border-amber-300 bg-amber-400 px-2 py-1 text-xs font-bold uppercase tracking-wide text-black;
	}

	.error {
		@apply mt-5 overflow-auto rounded-md border border-red-400 bg-red-300/10 p-3 text-red-100;
	}
</style>
