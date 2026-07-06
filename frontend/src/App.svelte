<script>
  import { onMount } from 'svelte';
  import SudokuBoard from './lib/SudokuBoard.svelte';

  let solverReady = $state(false);
  let solveWasm = $state(null);

  onMount(async () => {
    try {
      const wasm = await import('./wasm/zen_sudoku_solver.js');
      await wasm.default();
      solveWasm = wasm.solve;
      solverReady = true;
    } catch (e) {
      console.error('Failed to load WASM solver:', e);
    }
  });
</script>

<div class="min-h-screen bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-slate-100">
  <header class="py-6 text-center border-b border-slate-200 dark:border-slate-700">
    <h1 class="text-3xl font-bold tracking-tight">🧩 Zen Sudoku</h1>
    <p class="mt-1 text-slate-500 dark:text-slate-400 text-sm">
      Powered by Rust&nbsp;+&nbsp;WASM
    </p>
  </header>

  <main class="flex flex-col items-center py-10 px-4">
    {#if !solverReady}
      <p class="text-slate-500 dark:text-slate-400 animate-pulse">Loading solver…</p>
    {:else}
      <SudokuBoard solve={solveWasm} />
    {/if}
  </main>

  <footer class="py-4 text-center text-xs text-slate-400 dark:text-slate-600 border-t border-slate-200 dark:border-slate-700">
    Built with Svelte, TailwindCSS and Rust WASM ·
    <a
      href="https://github.com/marcusholmgren/zen-sudoku"
      class="underline hover:text-slate-600 dark:hover:text-slate-300"
      target="_blank"
      rel="noreferrer"
    >Source</a>
  </footer>
</div>
