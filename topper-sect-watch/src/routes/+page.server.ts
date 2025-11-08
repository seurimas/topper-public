import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ cookies, request, locals: { supabase } }) => {
        const formData = await request.formData();
        const logUrl = formData.get('log_url');

        if (typeof logUrl !== 'string' || !logUrl.startsWith('http')) {
            return { success: false, error: 'Invalid log URL' };
        }

        const { data: apiKey } = await supabase.functions.invoke('get-key');
        if (!apiKey) {
            return fail(500, { success: false, error: 'Failed to retrieve API key' });
        }
        const { data, error } = await supabase.functions.invoke('share-log', {
            body: { url: logUrl, apiKey },
        });
        if (error) {
            return fail(500, { success: false, error: error.message });
        } else if (!data) {
            return fail(500, { success: false, error: 'No data returned from function' });
        }
        redirect(303, `/logs/${data.saved}`);
    }
}