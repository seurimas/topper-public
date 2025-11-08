<script lang="ts">
	import type { WasmTimeline } from 'topper';
	import Combatant from '../combat/Combatant.svelte';
	import type { TimelineControl } from '../combat/types';
	import TimelineController from './TimelineController.svelte';
	import VoiceController from './VoiceController.svelte';

    let {
        timelineState,
        timelineTime = $bindable(),
        timelineControl,
        myName,
        oppName,
        myClass,
        oppClass,
    }: {
        timelineState: WasmTimeline,
        timelineTime: number,
        timelineControl: TimelineControl,
        myName: string,
        oppName: string,
        myClass: string,
        oppClass: string,
    } = $props();

    let { balances, afflictions, limbs } = $derived.by(() => {
        // Read the timeline time to make this re-run when it changes.
        // timelineState is a rich, wasm object that's not tracked for changes.
        const time = timelineTime;
        return {balances: {
            [myName]: timelineState.get_balances(myName),
            [oppName]: timelineState.get_balances(oppName),
        }, afflictions: {
            [myName]: timelineState.get_afflictions(myName),
            [oppName]: timelineState.get_afflictions(oppName),
        }, limbs: {
            [myName]: timelineState.get_limb_state(myName),
            [oppName]: timelineState.get_limb_state(oppName),
        }};
    })
</script>

<div class="container">
    <div class="combatants"><h2 class="section-header">Combatants</h2>
        <Combatant
            name={myName}
            className={myClass}
            balances={balances[myName]}
            afflictions={afflictions[myName]}
            limbs={limbs[myName]}
        />
        <Combatant
            name={oppName}
            className={oppClass}
            balances={balances[oppName]}
            afflictions={afflictions[oppName]}
            limbs={limbs[oppName]}
        />
    </div>
    <span class="combatants-mobile">View combatants' state on larger screens.</span>
    <TimelineController bind:timelineControl={timelineControl} bind:timelineTime={timelineTime} />
    <VoiceController />
</div>

<style>
	@reference "tailwindcss";

    .container {
        @apply fixed top-16 right-16 w-auto bg-amber-900 border-l border-gray-700 p-4 overflow-y-auto rounded-xl max-h-[calc(100vh-8rem)];
    }

    .combatants {
        @apply hidden lg:block;
    }

    .combatants-mobile {
        @apply block lg:hidden mx-4 mb-4 text-sm text-gray-400;
    }

    .section-header {
        @apply text-lg font-semibold mb-4;
    }
</style>