<script lang="ts">
	import CombatStateViewer from '$lib/combat/CombatStateViewer.svelte';
	import LogLine from '$lib/LogLine.svelte';
	import { parseLogId } from '$lib/sect_logs.js';
	import { onMount } from 'svelte';
    import init, { WasmTimeSlices, WasmTimeline } from 'topper';

    let timeSlices: WasmTimeSlices | undefined = undefined;
    let timelineState: WasmTimeline | undefined = $state(undefined);

    let { data } = $props();
    let { log } = $derived(data);

    const { name, myName, oppName, myClass, oppClass } = $derived(parseLogId(log.id));

    // We need to keep references to the elements which display the times for each time slice.
    // These are used for matching scroll position to time.
    const timeRefs: Record<number, HTMLElement> = {};
    const boundingBoxes: Record<number, DOMRect> = {};

    function addTimeRef(lineIdx: number, time: number, el?: HTMLElement) {
        if (!el) return;
        timeRefs[lineIdx] = el;
        const { x, y, width, height } = el.getBoundingClientRect();
        const { scrollX, scrollY } = window;
        boundingBoxes[lineIdx] = new DOMRect(x + scrollX, y + scrollY, width, height);
        el.onclick = () => {
            if (!timeSlices || !timelineState) {
                console.warn('WASM not initialized yet, cannot set time.');
                return;
            }
            console.log(`Setting time to ${time}`);
            console.log('Applied slices:', timelineState.set_timeline_time(timeSlices, time));
            console.log('My balances after time set:', timelineState.get_balances(myName));
        };
    }

    onMount(() => {
        window.addEventListener('resize', () => {
            for (const lineIdx in timeRefs) {
                const el = timeRefs[lineIdx];
                const { x, y, width, height } = el.getBoundingClientRect();
                const { scrollX, scrollY } = window;
                boundingBoxes[lineIdx] = new DOMRect(x + scrollX, y + scrollY, width, height);
            }
        });

        init().then(() => {
            console.log('WASM initialized for log page.');
            timeSlices = new WasmTimeSlices(JSON.stringify(log));
            console.log('Time slices loaded:', timeSlices.get_times());
            timelineState = new WasmTimeline(myName);
            console.log('Timeline initialized:', timelineState);
        });
    });
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
{#if timelineState}
    <CombatStateViewer {timelineState} {myName} {oppName} {myClass} {oppClass} />
{/if}