<script lang="ts">
  import { onMount } from 'svelte';
  import {
    advanceTimelineBy,
    advanceTimelineToQeb,
    getActions,
    getPassives,
    getState,
    getSupportedClasses,
    setActiveClass,
    simulateAction,
    type ActionDescriptor,
    type PassiveDescriptor
  } from './wasm';

  let classes: string[] = [];
  let activeClass = 'Ascendril';
  let actions: ActionDescriptor[] = [];
  let passives: PassiveDescriptor[] = [];
  let selectedAction = '';
  let battleStats: unknown = null;
  let latestObservations: string[] = [];
  let latestCommand = '';
  let loading = false;
  let error = '';

  $: battleStatsPretty = battleStats
    ? JSON.stringify(battleStats, null, 2)
    : '{}';

  async function initialize() {
    loading = true;
    error = '';

    try {
      classes = await getSupportedClasses();
      const state = await getState();
      activeClass = state.active_class;
      battleStats = state.battle_stats;
      actions = await getActions();
      passives = await getPassives();
      selectedAction = actions[0]?.id ?? '';
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function changeClass() {
    loading = true;
    error = '';

    try {
      const state = await setActiveClass(activeClass);
      battleStats = state.battle_stats;
      actions = await getActions();
      passives = await getPassives();
      selectedAction = actions[0]?.id ?? '';
      latestObservations = [];
      latestCommand = '';
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function sendAction() {
    if (!selectedAction) {
      return;
    }
    loading = true;
    error = '';
    try {
      const result = await simulateAction(selectedAction);
      latestCommand = result.command;
      latestObservations = result.observations;
      battleStats = result.battle_stats;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function advanceBy(deltaMs: number) {
    loading = true;
    error = '';
    try {
      const state = await advanceTimelineBy(deltaMs);
      battleStats = state.battle_stats;
      latestObservations = [];
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  async function advanceToQeb() {
    loading = true;
    error = '';
    try {
      const state = await advanceTimelineToQeb();
      battleStats = state.battle_stats;
      latestObservations = [];
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await initialize();
  });
</script>

<main>
  <h1>Topper Aetolia Learner</h1>
  <p>
    Manual combat simulation sandbox with class-driven action catalogs and timeline controls.
  </p>

  <section>
    <h2>Setup</h2>
    <p>Active player: <strong>Learner</strong>. Target player: <strong>Target</strong>.</p>
    <label>
      Class
      <select bind:value={activeClass} on:change={changeClass} disabled={loading}>
        {#each classes as className}
          <option value={className}>{className}</option>
        {/each}
      </select>
    </label>

    <h3>Passive Actions</h3>
    <ul>
      {#each passives as passive}
        <li>{passive.label} every {passive.period_ms} ms</li>
      {/each}
    </ul>
  </section>

  <section>
    <h2>Manual Action</h2>
    <label>
      Ascendril action
      <select bind:value={selectedAction} disabled={loading || actions.length === 0}>
        {#each actions as action}
          <option value={action.id}>
            {action.label}{action.targeted ? ' (targeted -> Target)' : ' (untargeted)'}
          </option>
        {/each}
      </select>
    </label>
    <button on:click={sendAction} disabled={loading || !selectedAction}>
      {#if loading}Working...{:else}Send Action{/if}
    </button>
    {#if latestCommand}
      <p><strong>Last command:</strong> {latestCommand}</p>
    {/if}
    {#if latestObservations.length > 0}
      <h3>Last Simulated Observations</h3>
      <ul>
        {#each latestObservations as observation}
          <li>{observation}</li>
        {/each}
      </ul>
    {/if}
  </section>

  <section>
    <h2>Timeline Controls</h2>
    <div class="button-row">
      <button on:click={() => advanceBy(10)} disabled={loading}>+10 ms</button>
      <button on:click={() => advanceBy(25)} disabled={loading}>+25 ms</button>
      <button on:click={() => advanceBy(50)} disabled={loading}>+50 ms</button>
      <button on:click={() => advanceBy(100)} disabled={loading}>+100 ms</button>
      <button on:click={advanceToQeb} disabled={loading}>Advance to QEB</button>
    </div>
  </section>

  <section class="result">
    <h2>Battle Stats (JSON)</h2>
    <pre>{battleStatsPretty}</pre>
  </section>

  {#if error}
    <pre class="error">{error}</pre>
  {/if}
</main>
