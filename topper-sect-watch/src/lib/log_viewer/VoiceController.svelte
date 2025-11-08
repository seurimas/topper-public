<script lang="ts">
    import CollapseHeader from "$lib/CollapseHeader.svelte";
	import { getSelectedVoice, getVoiceSpeed, setSelectedVoice, setVoiceSpeed } from "./voice.svelte";

    let collapsed = $state(true);

    let availableVoices = $derived.by(() => {
        if (!('speechSynthesis' in window)) {
            return [];
        }
        return speechSynthesis.getVoices();
    });
</script>

<!-- {#snippet validVoiceSelector(name: string, controlType: TimelineControl['type'])}
    <label for={`voice-mode-${controlType}`}>{name}</label>
    <input
        type="checkbox"
        id={`voice-mode-${controlType}`}
        checked={isVoiceControlType(controlType)}
        onchange={(e) => {
            const checkbox = e.target as HTMLInputElement;
            if (checkbox.checked != isVoiceControlType(controlType)) {
                toggleVoiceControlType(controlType);
            }
        }}
        class="mx-2"
    />
{/snippet} -->

<div class="timeline-controller">
    <CollapseHeader bind:collapsed={collapsed}>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <h3 class="header" onclick={() => { collapsed = !collapsed; }}>Voice Controller</h3>
    </CollapseHeader>
    {#if !collapsed}
        <!-- <div class="mb-2">
            <div class="mr-4">Enable Voice Modes:</div>
            <div class="flex justify-between mb-2">
                {@render validVoiceSelector('Scroll Lock', 'scrollLock')}
                {@render validVoiceSelector('Time Step', 'timeStep')}
                {@render validVoiceSelector('Manual', 'manual')}
            </div>
        </div> -->
        <div class="mb-2">
            <label for="speed" class="mb-2">TTS Rate: {getVoiceSpeed()}x</label>
            <input
                type="range"
                id="speed"
                min="0.1"
                max="5"
                step="0.1"
                bind:value={getVoiceSpeed, setVoiceSpeed}
                class="w-full"
            />
        </div>
        <div>
            <label for="voices" class="block mb-2">Select Voice:</label>
            <select
                id="voices"
                class="border border-gray-300 rounded p-2 w-full"
                value={getSelectedVoice()}
                onchange={(e) => {
                    const select = e.target as HTMLSelectElement;
                    const voiceName = select.value;
                    setSelectedVoice(voiceName);
                }}
            >
                {#if availableVoices.length === 0}
                    <option value="default" disabled>No voices available</option>
                {:else}
                    {#each availableVoices as voice}
                        <option value={voice.name}>{voice.name}</option>
                    {/each}
                {/if}
            </select>
        </div>
    {/if}
</div>

<style>
    @reference "tailwindcss";

    .header {
        @apply flex-1;
    }

    #voices {
        @apply max-w-min;
    }
</style>