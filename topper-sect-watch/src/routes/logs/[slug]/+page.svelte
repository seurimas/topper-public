<script lang="ts">
	import LogLine from '$lib/LogLine.svelte';
	import { parseLogId } from '$lib/sect_logs';

    let { data } = $props();
    let { log } = $derived(data);
</script>

<div class="p-6">
    <div class="p-2 border-amber-900 border-2 mb-4 pb-2 overflow-y-auto h-full">
        {#each log.body as line}
            <LogLine {line} />
        {/each}
    </div>
</div>