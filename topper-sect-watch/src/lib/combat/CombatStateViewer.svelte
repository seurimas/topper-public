<script lang="ts">
	import type { WasmTimeline } from 'topper';

    let { timelineState, myName, oppName, myClass, oppClass }: { timelineState: WasmTimeline, myName: string, oppName: string, myClass: string, oppClass: string } = $props();
</script>

<div class="fixed top-16 right-0 w-64 h-[calc(100vh-4rem)] bg-gray-900 border-l border-gray-700 p-4 overflow-y-auto">
    <h2 class="text-lg font-semibold mb-4">Combatants</h2>
    <div class="mb-6">
        <h3 class="text-md font-medium text-indigo-400 mb-2">{myName} ({myClass})</h3>
        <p class="text-sm text-gray-300">Balance: {timelineState.get_balances(myName).join(', ')}</p>
    </div>
    <div>
        <h3 class="text-md font-medium text-red-400 mb-2">{oppName} ({oppClass})</h3>
        <p class="text-sm text-gray-300">Balance: {timelineState.get_balances(oppName).join(', ')}</p>
    </div>
</div>