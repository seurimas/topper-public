<script lang="ts">
	import CollapseHeader from "$lib/CollapseHeader.svelte";
    import { type TimelineControl } from "../combat/types";

    let {
        timelineControl = $bindable(),
        timelineTime = $bindable(),
    }: {
        timelineControl: TimelineControl,
        timelineTime: number,
    } = $props();

    let collapsed = $state(false);
</script>

<div class="timeline-controller">
    <CollapseHeader bind:collapsed={collapsed}>
        <h3 class="header">Timeline Controller</h3>
    </CollapseHeader>
    {#if !collapsed}
    {/if}
</div>

<style>
    @reference "tailwindcss";
</style>