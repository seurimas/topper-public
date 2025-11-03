<script lang="ts">
    let { data } = $props();
    let { logs } = $derived(data);
</script>

<div>{logs.length} logs found</div>

<style></style>