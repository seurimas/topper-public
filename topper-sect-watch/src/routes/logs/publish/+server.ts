import { json, type RequestHandler } from '@sveltejs/kit';
import { page } from '$app/state';

export const POST: RequestHandler = async ({ url, request, locals: { supabase } }) => {
    const formData = await request.json();
    const { url: logUrl, api_key: apiKey } = formData;

    if (typeof logUrl !== 'string' || !logUrl.startsWith('http')) {
        return json({ success: false, error: 'Invalid log URL' });
    }
    if (!apiKey) {
        return json({ success: false, error: 'Failed to retrieve API key' });
    }
    const { data, error } = await supabase.functions.invoke('share-log', {
        body: { url: logUrl, apiKey },
    });
    if (error) {
        return json({ success: false, error: error.message });
    } else if (!data) {
        return json({ success: false, error: 'No data returned from function' });
    }
    return json({ success: true, saved: `https://${url.host}/logs/${data.saved}` });
};