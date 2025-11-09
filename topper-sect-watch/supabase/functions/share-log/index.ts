// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

// Setup type definitions for built-in Supabase Runtime APIs
import "jsr:@supabase/functions-js/edge-runtime.d.ts"
import { createClient } from 'npm:@supabase/supabase-js@2'
import { cleanup_old_log, parse_html_to_page } from "./share-log-wasm/pkg/share_log_wasm.js";

const VALID_URL_PREFIX = "http://aetolia.com/local/combatlogs/"
const STORAGE_BUCKET_NAME = "sect_logs";

// TODO: Allow users to specify which bodies/commands to strip
const FILTERED_BODIES: string[] = [
  "Tells",
  "Council",
  "a sprawing venantium cuff",
  "an insignia of the Blades",
];

const FILTERED_COMMANDS: string[] = [
  "auction",
  "tell",
  "otells",
  "gtells",
  "gtstells",
  "ctells",
  "clantells",
  "clan",
  "gw",
  "owho",
  "cw",
  "who",
  "rm",
  "rn",
  "nstat",
  "message",
  "msg",
  "tell",
  "ghelp",
  "chelp",
  "clhelp",
  "ohelp",
  "cohelp",
  "clanhelp",
];

type ExplainerPage = {
  id: string;
  body: string[];
  comments: string[];
};

const OLD_LOG_REGEX = /(.*)_(\d\d_\d\d_\d\d)_(\d+)_(\d+)$/

Deno.serve(async (req: Request) => {
  try {
    let { url, page, apiKey } = await req.json()
    
    let explainerPage: ExplainerPage;
    
    if (page) {
      // If page is provided, use it directly
      const oldLog = page as ExplainerPage;
      const cleanedOldLog = JSON.parse(cleanup_old_log(JSON.stringify(oldLog)));
      const match = OLD_LOG_REGEX.exec(cleanedOldLog.id);
      if (match) {
        const baseId = match[1];
        const lineCount = match[3];
        const duration = match[4];
        cleanedOldLog.id = `${baseId}_${lineCount}_${duration}`;
      }
      explainerPage = cleanedOldLog;
    } else if (url) {
      // If url is provided, fetch and parse it
      if (url.startsWith('https://')) {
        url = url.replace('https://', 'http://');
      }
      
      // Validate that URL starts with the expected prefix
      if (typeof url !== 'string' || !url.startsWith(VALID_URL_PREFIX)) {
        return new Response(
          JSON.stringify({ 
            error: "Invalid URL. Must start with http://aetolia.com/local/combatlogs/" 
          }),
          { 
            status: 400,
            headers: { "Content-Type": "application/json" } 
          }
        )
      }

      // Fetch the HTML content from the URL
      const response = await fetch(url)
      
      if (!response.ok) {
        return new Response(
          JSON.stringify({ 
            error: `Failed to fetch URL: ${response.status} ${response.statusText}` 
          }),
          { 
            status: 400,
            headers: { "Content-Type": "application/json" } 
          }
        )
      }

      const html = await response.text()
      
      // Parse the HTML to an ExplainerPage
      const explainerPageString = parse_html_to_page(html, FILTERED_BODIES, FILTERED_COMMANDS);
      explainerPage = JSON.parse(explainerPageString);
    } else {
      return new Response(
        JSON.stringify({ 
          error: "'url' must be provided" 
        }),
        { 
          status: 400,
          headers: { "Content-Type": "application/json" } 
        }
      )
    }

    const supabase = createClient(
      Deno.env.get("SUPABASE_URL")!,
      Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!
    );

    const userId = await supabase.rpc('get_user_id_from_api_key', { api_key: apiKey }).then(res => {
      if (res.error || !res.data) {
        throw new Error(`Invalid API key: ${JSON.stringify(res.error)}`);
      }
      return res.data;
    });

    const { error } = await supabase.storage
      .from(STORAGE_BUCKET_NAME)
      .upload(
        `logs/${explainerPage.id}.json`,
        new Blob([JSON.stringify(explainerPage)], { type: "application/json" }),
        { metadata: { userId }, upsert: true }
      );

    if (error) {
      console.error("Supabase storage upload error:", error);
      return new Response(
        JSON.stringify({ 
          error: "Failed to save the explainer page to storage" 
        }),
        { 
          status: 500,
          headers: { "Content-Type": "application/json" } 
        }
      )
    }
    
    return new Response(
      JSON.stringify({
        saved: explainerPage.id,
      }),
      { headers: { "Content-Type": "application/json" } }
    )
    
  } catch (error) {
    console.error("Error processing request:", error)
    return new Response(
      JSON.stringify({ 
        error: "Internal server error while processing the request" 
      }),
      { 
        status: 500,
        headers: { "Content-Type": "application/json" } 
      }
    )
  }
})

/* To invoke locally:

  1. Run `supabase start` (see: https://supabase.com/docs/reference/cli/supabase-start)
  2. Make an HTTP request:

  curl -i --location --request POST 'http://127.0.0.1:54321/functions/v1/share-log' \
    --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0' \
    --header 'Content-Type: application/json' \
    --data '{"name":"Functions"}'

*/
