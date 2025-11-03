import { redirect } from '@sveltejs/kit'

import type { RequestHandler } from './$types'

export const GET: RequestHandler = async ({ url, locals: { supabase } }) => {
    const { data, error } = await supabase.auth.signInWithOAuth({
        provider: 'discord',
    });

    if (data.url) {
        redirect(303, data.url);
    }

    return new Response('Error initiating OAuth sign-in', { status: 500 });
}