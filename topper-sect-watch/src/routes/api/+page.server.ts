import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { STORAGE_BUCKET_NAME, type ExplainerPage } from '$lib/sect_logs';

export const load: PageServerLoad = async ({ locals: { supabase } }) => {
    const { data: api_key } = await supabase.functions.invoke('get-key');

    return { apiKey: api_key };
}