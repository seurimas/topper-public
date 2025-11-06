<script lang="ts">
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
</script>

<div class="combatant">
    <h3 class="header">{name} ({className})</h3>
    <span class="afflictions-header">Afflictions: {afflictions.length}</span>
    <CombatantBalances {balances} />
    <CombatantLimbs {limbs} />
    <CombatantAfflictions {afflictions} />
</div>

<style>
	@reference "tailwindcss";

    .combatant {
        @apply mb-6 w-min;
    }

    .header {
        @apply text-base font-bold text-indigo-400 mb-2 float-left;
    }

    .afflictions-header {
        @apply float-right;
    }
</style>