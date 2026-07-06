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

<!-- TopAppBar -->
<header class="fixed top-0 left-0 w-full z-50 flex justify-between items-center px-4 md:px-8 py-3 bg-surface shadow-sm">
  <div class="flex items-center gap-4">
    <span class="font-display-grid text-display-grid font-bold text-primary">ZenSudoku</span>
  </div>
  <div class="flex items-center gap-4">
    <button class="p-2 rounded-full hover:bg-surface-container-high transition-colors active:scale-95 text-on-surface-variant">
      <span class="material-symbols-outlined">help_outline</span>
    </button>
    <button class="p-2 rounded-full hover:bg-surface-container-high transition-colors active:scale-95 text-on-surface-variant">
      <span class="material-symbols-outlined">settings</span>
    </button>
  </div>
</header>

<!-- SideNavBar (Desktop Only) -->
<nav class="hidden lg:flex flex-col fixed left-0 top-0 h-full z-40 py-6 w-72 bg-surface-container-low border-r border-outline-variant pt-24 transition-all">
  <div class="px-6 mb-8">
    <div class="flex items-center gap-4 mb-2">
      <div class="w-12 h-12 bg-primary-container text-on-primary-container rounded-2xl flex items-center justify-center font-bold text-xl shadow-sm">
        P
      </div>
      <div>
        <h2 class="font-bold text-on-surface">Player One</h2>
        <p class="text-sm text-on-surface-variant">Novice Solver</p>
      </div>
    </div>
  </div>
  <div class="flex flex-col gap-2 px-4">
    <button class="flex items-center gap-4 px-4 py-3 bg-secondary-container text-on-secondary-container rounded-xl font-bold transition-colors">
      <span class="material-symbols-outlined">grid_on</span>
      Current Puzzle
    </button>
    <button class="flex items-center gap-4 px-4 py-3 hover:bg-surface-container text-on-surface-variant hover:text-on-surface rounded-xl transition-colors">
      <span class="material-symbols-outlined">history</span>
      History &amp; Stats
    </button>
    <button class="flex items-center gap-4 px-4 py-3 hover:bg-surface-container text-on-surface-variant hover:text-on-surface rounded-xl transition-colors">
      <span class="material-symbols-outlined">emoji_events</span>
      Achievements
    </button>
  </div>
  <div class="mt-auto px-6 pb-6">
    <div class="bg-surface-container p-4 rounded-2xl border border-outline-variant/50">
      <p class="text-sm font-bold text-on-surface mb-1">Daily Challenge</p>
      <p class="text-xs text-on-surface-variant mb-3">Complete in under 5 mins</p>
      <button class="w-full py-2 bg-white text-primary rounded-lg text-sm font-bold shadow-sm hover:bg-surface-bright transition-colors">Start</button>
    </div>
  </div>
</nav>

<!-- Main Content Area -->
<main class="pt-20 pb-24 md:pb-8 lg:pl-72 min-h-screen flex flex-col transition-all">
  {#if !solverReady}
    <div class="flex-1 flex items-center justify-center">
      <p class="text-on-surface-variant animate-pulse font-body-lg">Loading solver…</p>
    </div>
  {:else}
    <SudokuBoard solve={solveWasm} />
  {/if}
</main>
