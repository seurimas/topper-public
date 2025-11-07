<script lang="ts">
	import CollapseHeader from "$lib/CollapseHeader.svelte";
    import CombatantAfflictions from "./CombatantAfflictions.svelte";
	import CombatantBalances from "./CombatantBalances.svelte";
	import CombatantLimbs from "./CombatantLimbs.svelte";
	import type { Balances, LimbsState } from "./types";

    let {
        name,
        className,
        balances,
        afflictions,
        limbs,
    } : {
        name: string,
        className: string,
        balances: Balances,
        afflictions: string[],
        limbs: LimbsState,
    } = $props();

    let collapsed = $state(false);
</script>

<div class="combatant">
    <CollapseHeader bind:collapsed={collapsed}>
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <h3 class="header" onclick={() => { collapsed = !collapsed }}>{name} ({className})</h3>
        <span class="afflictions-header">Afflictions: {afflictions.length}</span>
    </CollapseHeader>
    <div class={["body", collapsed ? "collapsed" : ""]}>
        <CombatantBalances {balances} />
        <CombatantLimbs {limbs} />
        <CombatantAfflictions {afflictions} />
    </div>
</div>

<style>
	@reference "tailwindcss";

    .combatant {
        @apply mb-6 max-w-min min-w-full;
    }

    .header {
        @apply text-base font-bold text-indigo-400 mb-2 flex-1;
    }

    .afflictions-header {
        @apply justify-end;
    }

    .collapsed {
        @apply hidden;
    }
</style>