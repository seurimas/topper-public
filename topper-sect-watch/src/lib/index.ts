import type { SupabaseClient } from "@supabase/supabase-js";

export const getUserId = async (supabase: SupabaseClient): Promise<string | null> => {
    const {data, error } = await supabase.auth.getUser();
    if (error) {
        return null;
    }
    const { user } = data;
    return user ? user.id : null;
}