import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { STORAGE_BUCKET_NAME, type ExplainerPage } from '$lib/sect_logs';

export const load: PageServerLoad = async ({ params, locals: { supabase } }) => {
    const { slug } = params;

    const { data, error: storage_error } = await supabase.storage
      .from(STORAGE_BUCKET_NAME)
      .download(
        `logs/${slug}.json`,
        );

    if (storage_error || !data) {
        error(404, 'Log not found');
    }

    const text = await data.text();
    const parsed = JSON.parse(text) as ExplainerPage;
    return { log: parsed };
}