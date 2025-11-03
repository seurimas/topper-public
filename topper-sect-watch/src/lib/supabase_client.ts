import type { SupabaseClient } from "@supabase/supabase-js";

export async function signInWithDiscord(supabase: SupabaseClient) {
  const { data, error } = await supabase.auth.signInWithOAuth({
    provider: 'discord',
  })
}