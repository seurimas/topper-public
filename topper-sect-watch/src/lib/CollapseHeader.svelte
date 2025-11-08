<script lang="ts">
    let { children, collapsed = $bindable(), force = undefined }: { children: any, collapsed: boolean, force?: boolean } = $props();

    let isCollapsed = $derived.by(() => {
        if (force !== undefined) {
            return force;
        }
        return collapsed;
    });
</script>


<div class="header-section">
    <button class="collapser" disabled={force !== undefined} onclick={() => {collapsed = !collapsed}}>
        {#if isCollapsed}
            ▶
        {:else}
            ▼
        {/if}
    </button>
    {@render children()}
</div>

<style>
    @reference "tailwindcss";

    .header-section {
        @apply w-full flex flex-row items-start;
    }
    
</style>