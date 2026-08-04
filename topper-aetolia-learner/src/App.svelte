<script lang="ts">
  import { runLearnerSimulation } from './wasm';

  let attacker = 'me';
  let target = 'enemy';
  let horizonMs = 2000;
  let actions = 'jab, slash, disembowel';
  let loading = false;
  let error = '';
  let result: null | {
    best_action_order: string[];
    score: number;
    expected_observations: number;
    used_timeline: boolean;
    used_observables: boolean;
    notes: string[];
  } = null;

  async function runSimulation() {
    loading = true;
    error = '';
    result = null;

    try {
      result = await runLearnerSimulation({
        attacker,
        target,
        horizon_ms: horizonMs,
        candidate_actions: actions
          .split(',')
          .map((action) => action.trim())
          .filter((action) => action.length > 0)
      });
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  }
</script>

<main>
  <h1>Topper Aetolia Learner</h1>
  <p>
    Rapid simulation sandbox for timeline/observable-driven combat planning experiments.
  </p>

  <section>
    <label>
      Attacker
      <input bind:value={attacker} />
    </label>
    <label>
      Target
      <input bind:value={target} />
    </label>
    <label>
      Horizon (ms)
      <input type="number" min="1" bind:value={horizonMs} />
    </label>
    <label>
      Candidate actions (comma separated)
      <textarea rows="3" bind:value={actions}></textarea>
    </label>

    <button on:click={runSimulation} disabled={loading}>
      {#if loading}Running...{:else}Run Simulation{/if}
    </button>
  </section>

  {#if error}
    <pre class="error">{error}</pre>
  {/if}

  {#if result}
    <section class="result">
      <h2>Result</h2>
      <p>Score: <strong>{result.score.toFixed(3)}</strong></p>
      <p>Best order: {result.best_action_order.join(' -> ')}</p>
      <p>Expected observations: {result.expected_observations}</p>
      <p>Uses timeline: {result.used_timeline ? 'yes' : 'no'}</p>
      <p>Uses observables: {result.used_observables ? 'yes' : 'no'}</p>
      <ul>
        {#each result.notes as note}
          <li>{note}</li>
        {/each}
      </ul>
    </section>
  {/if}
</main>
