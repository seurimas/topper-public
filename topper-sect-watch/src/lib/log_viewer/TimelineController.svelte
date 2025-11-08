<script lang="ts">
	import CollapseHeader from "$lib/CollapseHeader.svelte";
	import { onMount } from "svelte";
    import { type TimelineControl } from "../combat/types";

    let {
        timelineControl = $bindable(),
        timelineTime = $bindable(),
    }: {
        timelineControl: TimelineControl,
        timelineTime: number,
    } = $props();

    let collapsed = $state(false);

    let currentTime = $derived.by(() => {
        if (timelineTime === undefined) return 'Time: N/A';
        const totalSeconds = Math.floor(timelineTime / 100);
        const centiseconds = timelineTime % 100;
        const minutes = Math.floor(totalSeconds / 60);
        const seconds = totalSeconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}.${centiseconds.toFixed(0).padStart(2, '0')}`;
    });

    let unpausedType: TimelineControl['type'] = 'scrollLock';

    function setTimelineControlType(value: TimelineControl['type']) {
        timelineControl.type = value;
        if (timelineControl.type == 'timeStep') {
            timelineControl.speed = 1;
        }
        if (timelineControl.type != 'manual') {
            unpausedType = timelineControl.type;
        }
    }

    onMount(() => {
        window.addEventListener('keypress', (e: KeyboardEvent) => {
            if (e.key === ' ') {
                if (timelineControl.type !== 'manual') {
                    unpausedType = timelineControl.type;
                    setTimelineControlType('manual');
                } else {
                    setTimelineControlType(unpausedType);
                }
                e.preventDefault();
            }
        });
    });
</script>

{#snippet radioButton(name: string, value: TimelineControl['type'])}
    <radio class="mr-4">
        <input
            type="radio"
            name="timeline-control-type"
            value={value}
            checked={timelineControl.type === value}
            onchange={() => setTimelineControlType(value)}
            class="mr-2"
        />
        <span>{name}</span>
    </radio>
{/snippet}

<div class="timeline-controller">
    <CollapseHeader bind:collapsed={collapsed}>
        <h3 class="header">Timeline Controller</h3>
        <span class="time-header">{currentTime}</span>
    </CollapseHeader>
    {#if !collapsed}
        {@render radioButton('Manual', 'manual')}
        {@render radioButton('Time Step', 'timeStep')}
        {@render radioButton('Scroll Lock', 'scrollLock')}

        {#if timelineControl.type === 'timeStep'}
            <div class="mt-4">
                <label for="speed" class="block mb-2">Speed: {timelineControl.speed}x</label>
                <input
                    type="range"
                    id="speed"
                    min="0.1"
                    max="5"
                    step="0.1"
                    bind:value={timelineControl.speed}
                    class="w-full"
                />
            </div>
        {/if}
    {/if}
</div>

<style>
    @reference "tailwindcss";

    .header {
        @apply flex-1;
    }

    .timeline-controller {
        @apply justify-end;
    }
</style>