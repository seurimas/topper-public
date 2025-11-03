<script lang="ts">
    import { createGrid, ModuleRegistry, AllCommunityModule } from 'ag-grid-community';
	import { onMount } from 'svelte';

    ModuleRegistry.registerModules([AllCommunityModule]);

    const columnDefs = [
        {
            field: 'name',
            headerName: 'Link',
            cellRenderer: (params: { value: string }) => {
                return `<a class="text-indigo-400 hover:underline" href="/logs/${params.value}">${params.value}</a>`;
            }
        },
        {
            field: 'myName',
            headerName: 'Shared By',
            sortable: true,
            filter: true,
        },
        {
            field: 'myClass',
            headerName: 'Class',
            sortable: true,
            filter: true,
        },
        {
            field: 'oppName',
            headerName: 'Opponent',
            sortable: true,
            filter: true,
        },
        {
            field: 'oppClass',
            headerName: 'Class',
            sortable: true,
            filter: true,
        },
        {
            field: 'winner',
            headerName: 'Winner',
            sortable: true,
            filter: true,
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

    const gridOptions = $derived({
        rowData: logs,
        columnDefs,
        theme: myTheme,
    });

    onMount(() => {
        createGrid(gridElement, gridOptions);
    });
</script>

<div class="mx-auto my-8 max-w-7xl px-4 sm:px-6 lg:px-8">
    <h1 class="text-3xl font-bold mb-4">Public Sect Logs</h1>
    <div class="ag-theme-alpine" style="height: 600px; width: 100%;" bind:this={gridElement}>
        
    </div>
</div>

<style></style>