<script lang="ts">
    import Readme from "$lib/Readme.svelte";
    import { page } from '$app/state';

    let { data } = $props();
    let { apiKey } = $derived(data);

    let revealed = $state(false);
</script>

<svelte:head>
	<title>Sect Watch - Terms</title>
</svelte:head>

<Readme title="API Information">
    <h2>Your API Key</h2>

    <p>
        To use the Sect Watch API, you will need an API key. You can find your API key below when logged in:
    </p>
    {#if apiKey}
        {#if revealed}
            <pre class="p-4 rounded mb-4">
{`${apiKey}`}
            </pre>
        {:else}
            <button class="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded mb-4" on:click={() => { revealed = true; }}>
                Reveal API Key
            </button>
        {/if}
    {:else}
        <p>
            You do not currently have an API key. Please sign in to generate one.
        </p>
    {/if}

    <h2>Share Your Logs</h2>

    <p>
        You can upload logs directly to the Sect Watch API from your MUD client. To do so, send a POST request to the following endpoint:
    </p>

    <pre class="p-4 rounded mb-4">
POST {page.url.protocol}://{page.url.host}/logs/publish
    </pre>

    <p>
        The request should include a JSON body with the following structure:
    </p>

    <pre class="p-4 rounded mb-4">
{`{
    "url": "https://aetolia.com/local/combatlogs/your_log_file.log",
    "api_key": "your_api_key_here"
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

    <h2>Mudlet Function</h2>

    <p>
        Here is a sample Mudlet function and event handlers that demonstrates how to upload a Sect log to Sect Watch:
    </p>

    <pre class="p-4 rounded mb-4 border-gray-600 border">
{`
SECT_WATCH_URL = "https://${page.url.host}/logs/publish"
SECT_WATCH_API_KEY = "your_api_key_here"  -- Replace with your actual API key

function uploadSectLogToSectWatch(logUrl)
    local body = string.format('{"url": "%s", "api_key": "%s"}', logUrl, SECT_WATCH_API_KEY)
    local headers = {
        ["Content-Type"] = "application/json"
    }

    postHTTP(body, SECT_WATCH_URL, headers)
end

function onHttpSectWatchPostDone(_, url, body)
  if url ~= SECT_WATCH_URL then
    return
  end
  local json = yajl.to_value(body)
  if json['success'] then
    cecho(string.format("<green>Successfully posted log to Sect watch: <white>%s", json['saved']))
  else
    cecho(string.format("<red>Failed to post log to Sect watch: <white>%s", json['error']))
  end
end
registerNamedEventHandler("SectWatch", "Post", "sysPostHttpDone", onHttpSectWatchPostDone)

function onHttpSectWatchPostError(_, response, url)
  if url ~= SECT_WATCH_URL then
    return
  end
  cecho(string.format("<white>Failed to post log to Sect watch: <red>%s", response))
end
registerNamedEventHandler("SectWatch", "PostError", "sysPostHttpError", onHttpSectWatchPostError)`}
    </pre>

    <p>
        You can also add this trigger to automatically upload logs when they are read from the scorebook:
    </p>

    <pre class="p-4 rounded mb-4 border-gray-600 border">{`^https?://aetolia.com/local/combatlogs/\w+_(\d+)_.*html$`}</pre>

    <p>
        And use the following script for the trigger action:
    </p>

    <pre class="p-4 rounded mb-4 border-gray-600 border">
{`local url = line
local code = "uploadSectLogToSectWatch(\"" .. url .. "\")"
tempTimer(2, code)`}
    </pre>
</Readme>

<style>
    @reference "tailwindcss";

    p {
        @apply mb-4;
    }
</style>