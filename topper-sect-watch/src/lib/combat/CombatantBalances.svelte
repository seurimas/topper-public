<script lang="ts">
	import type { Balances } from "./types";

    let { balances }: { balances: Balances } = $props();
</script>

{#snippet balance(icon: string, name: string, value: number | undefined)}
    <span class="balance-item" title={name}>
        <span class="icon">{icon}</span>
        {#if value !== undefined && value > 0}
            <span class="value">{value.toFixed(2)}</span>
        {:else}
            <span class="value zero">Ready</span>
        {/if}
    </span>
{/snippet}

<div class="balances">
    {@render balance("🌲", 'Tree Tattoo', balances.get('Tree'))}
    {@render balance("🧠", 'Focus', balances.get('Focus'))}
    {@render balance("😤", 'Fitness', balances.get('Fitness'))}
    {@render balance("🔮", 'Class Cure', balances.get('ClassCure1'))}
</div>
<div class="rebounding">
    {@render balance("🛡️", 'Rebounding', balances.get('Rebounding'))}
</div>

<style>
    @reference "tailwindcss";

    .balances {
        @apply clear-both w-sm grid grid-cols-4;
    }

    .balance-item {
        @apply inline-block mr-4;
    }
</style>