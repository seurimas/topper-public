<script lang="ts">
	import type { LimbsState } from "./types";

    let { limbs }: { limbs: LimbsState } = $props();

    function altText(key: keyof LimbsState): string {
        const state = limbs[key];
        const parts: string[] = [];
        parts.push(`Damage: ${state.damage.toFixed(0)}`);
        if (state.bruise_level > 0) {
            parts.push(`Bruise Level: ${state.bruise_level}`);
        }
        if (state.crippled) parts.push('Crippled');
        if (state.broken) parts.push('Broken');
        if (state.mangled) parts.push('Mangled');
        if (state.amputated) parts.push('Amputated');
        if (state.is_restoring) parts.push('Restoring');
        if (state.is_parried) parts.push('Parried');
        if (state.is_dislocated) parts.push('Dislocated');
        if (state.welt) parts.push('Welted');
        return parts.join(', ');
    }
</script>

{#snippet limb(key: keyof LimbsState)}
    <div class={[
        "limb",
        `damage-${Math.floor(limbs[key].damage / 10)}`,
        `bruise-${limbs[key].bruise_level}`,
        limbs[key].crippled ? 'crippled' : '',
        limbs[key].broken ? 'broken' : '',
        limbs[key].mangled ? 'mangled' : '',
        limbs[key].amputated ? 'amputated' : '',
        limbs[key].is_restoring ? 'restoring' : '',
        limbs[key].is_parried ? 'parried' : '',
        limbs[key].is_dislocated ? 'dislocated' : '',
        limbs[key].welt ? 'welt' : '',
    ]} title={altText(key)}>{key}</div>
{/snippet}

<div class="limbs">
    {@render limb('head')}
    {@render limb('torso')}
    {@render limb('left_arm')}
    {@render limb('right_arm')}
    {@render limb('left_leg')}
    {@render limb('right_leg')}
</div>

<style>
    @reference "tailwindcss";

    .limbs {
        @apply grid grid-cols-2 gap-x-2 gap-y-1 mb-4;
    }

    .limb {
        @apply mx-2 rounded-md text-center border-2 border-transparent;

        &.restoring {
            @apply border-amber-500;
        }

        &.damage-0 {
            @apply text-red-50;
        }

        &.damage-1 {
            @apply text-red-200;
        }

        &.damage-2 {
            @apply text-red-300;
        }

        &.damage-3 {
            @apply text-red-400;
        }

        &.damage-4 {
            font-weight: bolder;
            @apply text-red-500;
        }

        &.damage-5 {
            font-weight: bolder;
            @apply text-red-600;
        }

        &.damage-6 {
            font-weight: bolder;
            @apply text-red-700;
        }

        &.damage-7 {
            font-weight: 900;
            @apply text-red-800;
        }

        &.damage-8 {
            font-weight: 900;
            @apply text-red-900;
        }

        &.damage-9 {
            font-weight: 900;
            @apply text-red-950;
        }

        &.damage-10 {
            font-weight: 900;
            color: black;
        }

        &:not(.crippled) {
            background-color: blue;
        }

        &.crippled {
            background-color: green;
        }

        &.broken {
            background-color: yellow;
        }

        &.mangled {
            background-color: white;
        }

        &.amputated {
            background-color: magenta;
        }

        &.bruise-1 {
            text-decoration: underline dashed;
        }

        &.bruise-2 {
            text-decoration: underline;
        }

        &.bruise-3 {
            text-decoration: underline double;
        }
    }
</style>