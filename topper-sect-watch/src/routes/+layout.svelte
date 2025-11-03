<script lang="ts">
	import "../app.css";
	import { invalidate } from '$app/navigation'
	import { onMount } from 'svelte'
	import favicon from '$lib/assets/favicon.ico';
	import Navbar from "$lib/Navbar.svelte";

	let { data, children } = $props();
	let { session, user, supabase } = $derived(data);

	let logged_in = $derived(user !== null);

	onMount(() => {
		const { data } = supabase.auth.onAuthStateChange((_, newSession) => {
			if (newSession?.expires_at !== session?.expires_at) {
				invalidate('supabase:auth');
			}
		});

		return () => data.subscription.unsubscribe();
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>


<Navbar {logged_in} />
{@render children()}

<style>
	@reference "tailwindcss";

	:global(body) {
		@apply bg-gray-900 text-white;
	}

	:global(h2) {
		@apply text-2xl font-bold mt-6 mb-2;
	}

    :global(p) {
        @apply mb-4;
    }
</style>