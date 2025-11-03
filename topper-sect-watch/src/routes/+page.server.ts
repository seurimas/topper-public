import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ depends, locals: { supabase } }) => {
    depends("supabase:auth");

    return { logged_in: (await supabase.auth.getSession()).data?.session?.user !== null };
};