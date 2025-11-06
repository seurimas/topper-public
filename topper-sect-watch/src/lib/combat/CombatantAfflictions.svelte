<script lang="ts">
    import type { Afflictions } from "./types";

    let { afflictions }: { afflictions: Afflictions } = $props();

</script>

<div class="afflictions">
    {#if afflictions.length === 0}
        <span>No afflictions</span>
    {:else}
        {#each afflictions as affliction}
            <span class={['aff', affliction]}>{affliction}</span>
        {/each}
    {/if}
</div>

<style>
    @reference "tailwindcss";

    .afflictions {
        @apply mb-4 flex flex-wrap w-full;
    }

    
    span.aff {
        background-color: white;
        color: black;
        border: 1px solid transparent;
        @apply rounded-md pl-1 pr-1 mr-1 mb-1 min-w-0;

        &.anorexia,
        &.asthma,
        &.slickness {
            background-color: red;
        }

        &.aeon,
        &.paralysis,
        &.dyscrasia,
        &.frozen {
            background-color: magenta;
        }

        &.stupidity,
        &.paresis,
        &.impatience {
            background-color: pink;
        }

        &.perplexity,
        &.self_loathing {
            background-color: yellow;
        }
    }
</style>