<script lang="ts">
	import { getSectionsFromLine } from "./getSectionsFromLine";
	import type { TimelineControl } from "./combat/types";

    let {
        timelineControl,
        line,
        lineIdx,
        addTimeRef
    }: {
        timelineControl: TimelineControl,
        line: string,
        lineIdx: number,
        addTimeRef: (idx: number, time: number, ref: HTMLElement) => void
    } = $props();

    // svelte-ignore non_reactive_update // We're not interested in reactivity for this ref.
    let timeRef: HTMLElement;

    let sections = $derived(getSectionsFromLine(line));
</script>

<div class={["log", timelineControl.type]}>
    {#each sections as section}
        {#if section.time !== undefined}
            <span bind:this={() => timeRef, (ref) => {timeRef = ref; addTimeRef(lineIdx, section.time, ref)}} style="color: {section.color}">{section.text}</span>
        {:else}
            <span style="color: {section.color}">{section.text}</span>
        {/if}
    {/each}
</div>

<style>
    @reference "tailwindcss";

    .log {
        @apply font-mono text-sm whitespace-pre-wrap block;
    }
</style>