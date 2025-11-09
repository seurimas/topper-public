<script lang="ts">
    import { createGrid, ModuleRegistry, AllCommunityModule, type GridApi } from 'ag-grid-community';
	import { onMount } from 'svelte';
    import { page } from '$app/state';

    ModuleRegistry.registerModules([AllCommunityModule]);

    const columnDefs = [
        {
            field: 'name' as const,
            headerName: 'Sect Watch',
            cellRenderer: (params: { value: string }) => {
                return `<a class="text-indigo-400 hover:underline" href="/logs/${params.value}">View</a>`;
            }
        },
        {
            field: 'myName' as const,
            headerName: 'Shared By',
            sortable: true,
            filter: true,
        },
        {
            field: 'myClass' as const,
            headerName: 'Class',
            sortable: true,
            filter: true,
        },
        {
            field: 'oppName' as const,
            headerName: 'Opponent',
            sortable: true,
            filter: true,
        },
        {
            field: 'oppClass' as const,
            headerName: 'Class',
            sortable: true,
            filter: true,
        },
        {
            field: 'winner' as const,
            headerName: 'Winner',
            sortable: true,
            filter: true,
        },
        {
            field: 'duration' as const,
            headerName: 'Duration',
            valueFormatter: (params: { value: number }) => {
                const minutes = Math.floor(params.value / 6000);
                const seconds = params.value % 6000 / 100;
                return `${minutes}m ${seconds}s`;
            },
            sortable: true,
        }
    ];

    import { themeQuartz } from 'ag-grid-community';

    const myTheme = themeQuartz
        .withParams({
            accentColor: "#15BDE8",
            backgroundColor: "#0C0C0D",
            borderColor: "#ffffff00",
            borderRadius: 20,
            browserColorScheme: "dark",
            cellHorizontalPaddingScale: 1,
            chromeBackgroundColor: {
                ref: "backgroundColor"
            },
            columnBorder: false,
            fontFamily: {
                googleFont: "Roboto"
            },
            fontSize: 16,
            foregroundColor: "#BBBEC9",
            headerBackgroundColor: "#182226",
            headerFontSize: 14,
            headerFontWeight: 500,
            headerTextColor: "#FFFFFF",
            headerVerticalPaddingScale: 0.9,
            iconSize: 20,
            rowBorder: false,
            rowVerticalPaddingScale: 1.2,
            sidePanelBorder: false,
            spacing: 8,
            wrapperBorder: false,
            wrapperBorderRadius: 0
        });


    let gridElement: HTMLDivElement;

    let { data } = $props();
    let { logs } = $derived(data);

    let gridSearch = $state(page.url.searchParams.get('search') || '');
    let gridSearchData: string | null = $state(page.url.searchParams.get('search') || null);
    let gridApi: GridApi | null = $state(null);

    onMount(() => {
        const gridOptions = {
            rowData: logs,
            columnDefs,
            theme: myTheme,
            defaultColDef: {
                flex: 1,
                minWidth: 80,
                resizable: true,
            },
        };
        
        gridApi = createGrid(gridElement, gridOptions);
        gridSearchData = gridSearch;
    });

    $effect(() => {
        if (page.url.searchParams.get('search') !== gridSearchData) {
            if (gridApi) {
                gridApi.setGridOption('rowData', logs);
            }
            gridSearchData = gridSearch;
        }
    });
</script>

<svelte:head>
	<title>Sect Watch - Logs</title>
</svelte:head>

<div class="mx-auto my-8 max-w-7xl px-4 sm:px-6 lg:px-8">
    <h1 class="text-3xl font-bold mb-4">Public Sect Logs</h1>
    <form method="GET" class="ml-4">
        <input
            name="search"
            type="text"
            placeholder="Search all logs... (e.g., class, player name)"
            class="border border-gray-300 rounded-md px-3 py-2 w-full max-w-sm"
            bind:value={gridSearch}
        />
        <button
            type="submit"
            class="ml-2 bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
        >
            Search
        </button>
    </form>
    {#if !page.url.searchParams.get('search')}
        <span class="text-sm text-gray-500">{logs.length} most recent shown</span>
    {:else}
        <span class="text-sm text-gray-500">Showing {logs.length} results for "{gridSearchData}"</span>
    {/if}

    <div class="ag-theme-alpine" style="height: 600px; width: 100%;" bind:this={gridElement}>
        
    </div>
</div>

<style></style>