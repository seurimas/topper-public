<script lang="ts">
    let { data } = $props();
    let { log } = $derived(data);
</script>

{log.body.length} Lines