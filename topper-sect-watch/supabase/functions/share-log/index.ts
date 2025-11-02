// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

// Setup type definitions for built-in Supabase Runtime APIs
import "jsr:@supabase/functions-js/edge-runtime.d.ts"
import { createClient } from 'npm:@supabase/supabase-js@2'
import { parse_html_to_page } from "./src-wasm/pkg/src_wasm.js";

const VALID_URL_PREFIX = "https://aetolia.com/local/combatlogs/"
const STORAGE_BUCKET_NAME = "sect_logs";

type ExplainerPage = {
  id: string;
  body: string[];
  comments: string[];
};

Deno.serve(async (req: Request) => {
  try {
    const { url } = await req.json()
    
    // Validate that URL starts with the expected prefix
    if (!url || typeof url !== 'string' || !url.startsWith(VALID_URL_PREFIX)) {
      return new Response(
        JSON.stringify({ 
          error: "Invalid URL. Must start with https://aetolia.com/local/combatlogs/" 
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
    const explainerPageString = parse_html_to_page(html);
    const explainerPage: ExplainerPage = JSON.parse(explainerPageString);

    const supabaseAdmin = createClient(
      Deno.env.get("SUPABASE_URL")!,
      Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!
    );

    await supabaseAdmin.storage.createBucket(STORAGE_BUCKET_NAME, {
      public: true,
    }).catch((e: Error) => {
      // Ignore "Bucket already exists" error
      if (!e.message.includes("Bucket already exists")) {
        throw e;
      }
    });

    const { data, error } = await supabaseAdmin.storage
      .from(STORAGE_BUCKET_NAME)
      .upload(
        `logs/${explainerPage.id}.json`,
        new Blob([JSON.stringify(explainerPage)], { type: "application/json" }),
        { upsert: false }
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
