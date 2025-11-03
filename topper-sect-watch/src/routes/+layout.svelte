<script lang="ts">
	import "../app.css";
	import { invalidate } from '$app/navigation'
	import { onMount } from 'svelte'
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from "$lib/Navbar.svelte";

	let { data, children } = $props();
	let { session, supabase } = $derived(data);

	let logged_in = $state(false);

	onMount(() => {
		const { data } = supabase.auth.onAuthStateChange((_, newSession) => {
			if (newSession?.expires_at !== session?.expires_at) {
				invalidate('supabase:auth');
			}
			logged_in = newSession !== null;
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
</style>