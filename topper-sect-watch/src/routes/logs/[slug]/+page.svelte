<script lang="ts">
	import LogLine from '$lib/LogLine.svelte';
	import { parseLogId } from '$lib/sect_logs.js';

    let { data } = $props();
    let { log } = $derived(data);

    const { name } = $derived(parseLogId(log.id));
</script>


<svelte:head>
	<title>Sect Watch - {name}</title>
</svelte:head>

<div class="p-6">
    <div class="p-2 border-amber-900 border-2 mb-4 pb-2">
        {#each log.body as line, lineIdx}
            <LogLine {line} />
        {/each}
    </div>
</div>