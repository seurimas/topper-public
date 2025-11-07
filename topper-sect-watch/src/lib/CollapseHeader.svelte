<script lang="ts">
    let { children, collapsed = $bindable() }: { children: any, collapsed: boolean } = $props();
</script>


<div class="header-section">
    <button class="collapser" onclick={() => {collapsed = !collapsed}}>
        {#if collapsed}
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