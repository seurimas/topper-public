<script lang="ts">
	import CombatStateViewer from '$lib/log_viewer/LogStateViewer.svelte';
	import type { TimelineControl } from '$lib/combat/types.js';
	import { calculateBoundingBox, recalculateBoundingBoxes } from '$lib/log_viewer/bounding.js';
	import { scrollHandler } from '$lib/log_viewer/scroll.js';
	import LogLine from '$lib/LogLine.svelte';
	import { parseLogId } from '$lib/sect_logs.js';
	import { onMount } from 'svelte';
    import init, { WasmTimeSlices, WasmTimeline } from 'topper';
	import { FRAME_INTERVAL, intervalHandler } from '$lib/log_viewer/interval.js';
	import { speakIfEnabled } from '$lib/log_viewer/voice.svelte.js';

    let timeSlices: WasmTimeSlices | undefined = undefined;
    // svelte-ignore non_reactive_update // wasm objects don't do reactivity well.
    let timelineState: WasmTimeline | undefined = undefined;
    // Use a simple, reactive primitive for the time. Changes here will trigger necessary re-renders.
    let timelineTime: number | undefined = $state(undefined);
    // Control for timeline scrolling behavior.
    let timelineControl: TimelineControl = $state({type: 'scrollLock'});

    let { data } = $props();
    let { log } = $derived(data);

    const { name, myName, oppName, myClass, oppClass } = $derived(parseLogId(log.id));

    // We need to keep references to the elements which display the times for each time slice.
    // These are used for matching scroll position to time.
    const timeRefs: Record<number, [HTMLElement, number]> = {};
    let boundingBoxes: Record<number, DOMRect> = {};

    function setTimelineTime(time: number) {
        if (!timeSlices || !timelineState) {
            console.warn('WASM not initialized yet, cannot set time.');
            return;
        }
        const lastCombatActions = timelineState.set_timeline_time(timeSlices, time);
        if (lastCombatActions.length > 0) {
            speakIfEnabled(lastCombatActions[lastCombatActions.length - 1], timelineControl.type);
        }
        timelineTime = time;
    }

    function addTimeRef(lineIdx: number, time: number, el?: HTMLElement) {
        if (!el) return;
        timeRefs[lineIdx] = [el, time];
        boundingBoxes[lineIdx] = calculateBoundingBox(el);
        el.onclick = () => {
            if (!timeSlices || !timelineState) {
                console.warn('WASM not initialized yet, cannot set time.');
                return;
            }
            if (timelineControl.type === 'manual') {
                setTimelineTime(time);
            }
        };
    }

    function preventScrollInTimeStep(e: Event) {
        if (timelineControl.type === 'timeStep') {
            if (e.target instanceof HTMLElement) {
                if (e.target.closest('.combatant')) {
                    // Allow scrolling inside combatant panels.
                    return;
                }
            }
            e.preventDefault();
        }
    }

    onMount(() => {
        // The space button is already handled in TimelineController for play/pause.
        window.addEventListener('wheel', preventScrollInTimeStep, { passive: false });
        window.addEventListener('touchmove', preventScrollInTimeStep, { passive: false });

        window.addEventListener('resize', () => {
            boundingBoxes = recalculateBoundingBoxes(timeRefs);
        });

        window.addEventListener('scroll', () => {
            if (!timeSlices || !timelineState || timelineTime === undefined) {
                return;
            }
            scrollHandler(timelineControl, timeRefs, boundingBoxes, timelineTime, setTimelineTime);
        });

        let lastInterval = Date.now();
        setInterval(() => {
            if (!timeSlices || !timelineState || timelineTime === undefined) {
                return;
            }
            const now = Date.now();
            const deltaTime = now - lastInterval;
            intervalHandler(timelineControl, timeRefs, boundingBoxes, timelineTime, setTimelineTime, deltaTime);
            lastInterval = now;
        }, FRAME_INTERVAL);

        init().then(() => {
            console.log('WASM initialized for log page.');
            timeSlices = new WasmTimeSlices(JSON.stringify(log));
            console.log('Time slices loaded:', timeSlices.get_times());
            timelineState = new WasmTimeline(myName);
            console.log('Timeline initialized:', timelineState);
            timelineTime = 0;
        });
    });
</script>


<svelte:head>
	<title>Sect Watch - {name}</title>
</svelte:head>

<div class="p-6">
    <div class={['p-2 border-amber-900 border-2 mb-4 pb-2', timelineControl.type]}>
        {#each log.body as line, lineIdx}
            <LogLine {line} {lineIdx} {addTimeRef} />
        {/each}
    </div>
</div>
{#if timelineTime !== undefined}
    <CombatStateViewer timelineState={timelineState!} bind:timelineTime={timelineTime} {timelineControl} {myName} {oppName} {myClass} {oppClass} />
{/if}