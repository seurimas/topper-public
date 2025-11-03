import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { STORAGE_BUCKET_NAME } from '$lib/sect_logs';

export const load: PageServerLoad = async ({ locals: { supabase } }) => {

    const { data, error: storage_error } = await supabase.storage
      .from(STORAGE_BUCKET_NAME)
      .list(`logs`);

    if (storage_error || !data) {
        error(404, 'Log not found');
    }

    console.log(data);

    return { logs: data.map(item => item.name.replace('.json', '')) };
}