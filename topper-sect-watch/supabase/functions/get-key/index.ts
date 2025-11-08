// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

// Setup type definitions for built-in Supabase Runtime APIs
import "jsr:@supabase/functions-js/edge-runtime.d.ts";
import { createClient } from 'npm:@supabase/supabase-js@2';

Deno.serve(async (req) => {
    
    const authHeader = req.headers.get('Authorization')!;
  
    const supabase = createClient(
      Deno.env.get("SUPABASE_URL")!,
      Deno.env.get("SUPABASE_ANON_KEY")!,
      {
        global: {
          headers: { "Authorization": authHeader || "" },
        },
      }
    );

    const { data, error } = await supabase.from('api_keys').select('*');

    if (error) {
        return new Response(
            JSON.stringify({ error: 'Failed to retrieve API key.' }),
            { status: 500, headers: { "Content-Type": "application/json" } },
        );
    }

    if (!data || data.length === 0) {
      const user = await supabase.auth.getUser();
      await supabase.from('api_keys').insert([{ user_id: user.data.user?.id }]);
    }

    const { data: newData, error: newError } = await supabase.from('api_keys').select('*');

    if (newError || !newData) {
        return new Response(
            JSON.stringify({ error: 'Failed to retrieve API key after insertion.' }),
            { status: 500, headers: { "Content-Type": "application/json" } },
        );
    }

  return new Response(
    JSON.stringify(data[0].key),
    { headers: { "Content-Type": "application/json" } },
  )
})

/* To invoke locally:

  1. Run `supabase start` (see: https://supabase.com/docs/reference/cli/supabase-start)
  2. Make an HTTP request:

  curl -i --location --request POST 'http://127.0.0.1:54321/functions/v1/get-key' \
    --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0' \
    --header 'Content-Type: application/json' \
    --data '{"name":"Functions"}'

*/
