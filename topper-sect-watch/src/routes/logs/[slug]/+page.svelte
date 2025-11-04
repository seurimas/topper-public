<script lang="ts">
	import LogLine from '$lib/LogLine.svelte';
	import { parseLogId } from '$lib/sect_logs.js';

    let { data } = $props();
    let { log } = $derived(data);

    const { name } = $derived(parseLogId(log.id));

    // We need to keep references to the elements which display the times for each time slice.
    // These are used for matching scroll position to time.
    const timeRefs: Record<number, HTMLElement> = {};
    const boundingBoxes: Record<number, DOMRect> = {};

    function addTimeRef(lineIdx: number, el?: HTMLElement) {
        if (!el) return;
        timeRefs[lineIdx] = el;
        const { x, y, width, height } = el.getBoundingClientRect();
        const { scrollX, scrollY } = window;
        boundingBoxes[lineIdx] = new DOMRect(x + scrollX, y + scrollY, width, height);
    }
</script>


<svelte:head>
	<title>Sect Watch - {name}</title>
</svelte:head>

<div class="p-6">
    <div class="p-2 border-amber-900 border-2 mb-4 pb-2">
        {#each log.body as line, lineIdx}
            <LogLine {line} {lineIdx} {addTimeRef} />
        {/each}
    </div>
</div>