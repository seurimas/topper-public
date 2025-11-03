<script lang="ts">
    let { logged_in }: { logged_in: boolean } = $props();
</script>

{#if logged_in}
  <form class="font-sans antialiased mx-8 p-8 flex flex-row" >
    <input type="url" name="log_url" placeholder="Sect Log URL" class="w-full rounded-md border border-gray-300 bg-gray-800 px-3 py-2 text-white placeholder-gray-400 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm mb-4" />
    <button type="submit" class="ml-4 rounded-md bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:text-sm mb-4">Publish Log</button>
  </form>
{/if}

<style>

</style>