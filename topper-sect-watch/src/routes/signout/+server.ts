import { redirect } from '@sveltejs/kit'

import type { RequestHandler } from './$types'

export const GET: RequestHandler = async ({ url, locals: { supabase } }) => {
    const { error } = await supabase.auth.signOut();

    if (!error) {
        redirect(303, '/');
    }

    return new Response('Error initiating OAuth sign-in', { status: 500 });
}