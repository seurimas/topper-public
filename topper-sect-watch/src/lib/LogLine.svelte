<script lang="ts">
	import { onMount } from "svelte";
	import { getSectionsFromLine } from "./getSectionsFromLine";

    let { line, lineIdx, addTimeRef }: { line: string, lineIdx: number, addTimeRef: (idx: number, ref: HTMLElement) => void } = $props();

    let timeRef: HTMLElement;

    let sections = $derived(getSectionsFromLine(line));
</script>

<span class="font-mono text-sm whitespace-pre-wrap block">
    {#each sections as section}
        {#if section.timeSection}
            <span bind:this={() => timeRef, (ref) => {timeRef = ref; addTimeRef(lineIdx, ref)}} style="color: {section.color}">{section.text}</span>
        {:else}
            <span style="color: {section.color}">{section.text}</span>
        {/if}
    {/each}
</span>