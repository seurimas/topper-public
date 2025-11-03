import { redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ cookies, request, locals: { supabase } }) => {
        const formData = await request.formData();
        const logUrl = formData.get('log_url');

        if (typeof logUrl !== 'string' || !logUrl.startsWith('http')) {
            return { success: false, error: 'Invalid log URL' };
        }

        const { data, error } = await supabase.functions.invoke('share-log', {
            body: { url: logUrl },
        });
        if (error) {
            return { success: false, error: error.message };
        } else if (!data) {
            return { success: false, error: 'No data returned from function' };
        }
        redirect(303, `/logs/${data.saved}`);
        return { success: true, data };
    }
}