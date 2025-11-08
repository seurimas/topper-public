<script lang="ts">
    import { page } from '$app/state';

    let { logged_in }: { logged_in: boolean } = $props();

    let mobileVisible = $state(false);

    let current = $derived(page.url.pathname);

    function toggleMobileMenu() {
        mobileVisible = !mobileVisible;
    }

    function closeMobileMenu() {
        mobileVisible = false;
    }
</script>

{#snippet button(name: string, path: string)}
    {#if current === path}
        <a href={path} onclick={closeMobileMenu} aria-current="page" class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white">{name}</a>
    {:else}
        <a href={path} onclick={closeMobileMenu} class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-white/5 hover:text-white">{name}</a>
    {/if}
{/snippet}

{#snippet all_buttons()}
    {@render button('Main', '/')}
    {@render button('Public Logs', '/logs')}
    {@render button('API Info', '/api')}
    {@render button('Terms', '/terms')}
    {#if logged_in}
      {@render button('Sign Out', '/signout')}
    {:else}
      {@render button('Sign In', '/signin')}
    {/if}
{/snippet}

<nav class="relative bg-gray-800">
  <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
    <div class="relative flex h-16 items-center justify-between">
      <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
        <!-- Mobile menu button-->
        <button type="button" onclick={toggleMobileMenu} class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-white/5 hover:text-white focus:outline-2 focus:-outline-offset-1 focus:outline-indigo-500">
          <span class="absolute -inset-0.5"></span>
          <span class="sr-only">Open main menu</span>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" data-slot="icon" aria-hidden="true" class="size-6 in-aria-expanded:hidden">
            <path d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" data-slot="icon" aria-hidden="true" class="size-6 not-in-aria-expanded:hidden">
            <path d="M6 18 18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
      </div>
      <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
        <div class="flex items-center">
          <span class="text-white font-bold text-lg">Sect Watch</span>
        </div>
        <div class="hidden sm:ml-6 sm:block">
          <div class="flex space-x-4">
            {@render all_buttons()}
          </div>
        </div>
      </div>
    </div>
  </div>

  <div id="mobile-menu" class={mobileVisible ? '' : 'sm:hidden'}>
    <div class="space-y-1 px-2 pt-2 pb-3">
        {@render all_buttons()}
    </div>
  </div>
</nav>