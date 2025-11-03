// src/routes/auth/confirm/+server.js
import type { EmailOtpType } from '@supabase/supabase-js'
import { redirect } from '@sveltejs/kit'

import type { RequestHandler } from './$types'
import { PUBLIC_REDIRECT_URL } from '$env/static/public';

export const GET: RequestHandler = async ({ url, locals: { supabase } }) => {
    const { data, error } = await supabase.auth.signInWithOAuth({
        provider: 'discord',
        options: {
            redirectTo: PUBLIC_REDIRECT_URL,
        },
    });

    if (data.url) {
        redirect(303, data.url);
    }

    return new Response('Error initiating OAuth sign-in', { status: 500 });
}