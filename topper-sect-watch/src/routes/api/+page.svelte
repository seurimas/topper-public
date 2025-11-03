<script lang="ts">
    import Readme from "$lib/Readme.svelte";
    import { page } from '$app/state';
</script>

<svelte:head>
	<title>Sect Watch - Terms</title>
</svelte:head>

<Readme title="API Information">
    <h2>Share Your Logs</h2>

    <p>TODO: Better instructions here. I will put Lua code here instead.</p>

    <p>
        You can upload logs directly to the Sect Watch API from your MUD client. To do so, send a POST request to the following endpoint:
    </p>

    <pre class="p-4 rounded mb-4">
POST {page.url.host}/
    </pre>

    <p>
        The request should include a JSON body with the following structure:
    </p>

    <pre class="p-4 rounded mb-4">
{`{
    "url": "https://aetolia.com/local/combatlogs/your_log_file.log"
}`}
    </pre>

    <p>
        Replace <code>your_log_file.log</code> with the actual URL of your Sect log file. Make sure that the log file is accessible and that you have permission to share it.
    </p>

    <h2>Response</h2>

    <p>
        Upon successful upload, the API will respond with a JSON object containing the ID of the uploaded log:
    </p>

    <pre class="p-4 rounded mb-4">
{`{
    "saved": "log_id_here"
}`}
    </pre>
</Readme>

<style>
    @reference "tailwindcss";

    p {
        @apply mb-4;
    }
</style>