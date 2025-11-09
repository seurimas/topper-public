import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { parseLogId, STORAGE_BUCKET_NAME } from '$lib/sect_logs';

export const load: PageServerLoad = async ({ url, locals: { supabase } }) => {
    
    const { data, error: storage_error } = await supabase.storage
      .from(STORAGE_BUCKET_NAME)
      .list(`logs`, {
        limit: url.searchParams.get('search') ? 1000 : 100,
        search: url.searchParams.get('search') || undefined,
      });

    if (storage_error || !data) {
        error(404, 'Log not found');
    }

    const names = data.map(item => item.name.replace('.json', '')).filter(name => name !== '.emptyFolderPlaceholder');
    const logs = names.map(parseLogId);

    return { logs };
}