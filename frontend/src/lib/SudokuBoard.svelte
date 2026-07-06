<script>
  /** @type {{ solve: (board: Uint8Array) => Uint8Array }} */
  let { solve } = $props();

  // 81-cell board; 0 = empty
  let cells = $state(Array(81).fill(0));
  // Which cells are "given" (locked)
  let givens = $state(Array(81).fill(false));
  // Status message
  let status = $state('');
  // Whether the board is solved
  let solved = $state(false);
  // Currently selected cell index
  let selectedIndex = $state(null);

  // Timer mock state
  let timerText = $state('00:00');

  function handleCellClick(index) {
    if (givens[index]) return; // Don't select fixed cells
    selectedIndex = index;
  }

  function handleKeydown(event) {
    if (selectedIndex === null) return;

    if (/^[1-9]$/.test(event.key) && !givens[selectedIndex]) {
      const val = parseInt(event.key, 10);
      cells[selectedIndex] = val;
      solved = false;
      status = '';
    } else if ((event.key === 'Backspace' || event.key === 'Delete' || event.key === 'Clear') && !givens[selectedIndex]) {
      cells[selectedIndex] = 0;
      solved = false;
      status = '';
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex = selectedIndex >= 9 ? selectedIndex - 9 : selectedIndex;
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = selectedIndex < 72 ? selectedIndex + 9 : selectedIndex;
    } else if (event.key === 'ArrowLeft') {
      event.preventDefault();
      selectedIndex = selectedIndex % 9 > 0 ? selectedIndex - 1 : selectedIndex;
    } else if (event.key === 'ArrowRight') {
      event.preventDefault();
      selectedIndex = selectedIndex % 9 < 8 ? selectedIndex + 1 : selectedIndex;
    }
  }

  function clearSelectedCell() {
    if (selectedIndex !== null && !givens[selectedIndex]) {
      cells[selectedIndex] = 0;
      solved = false;
      status = '';
    }
  }

  function solvePuzzle() {
    const board = new Uint8Array(cells);
    const result = solve(board);
    if (!result || result.length === 0) {
      status = '❌ No solution found.';
      solved = false;
    } else {
      cells = Array.from(result);
      status = '✅ Solved!';
      solved = true;
      selectedIndex = null;
    }
  }

  function clearBoard() {
    cells = Array(81).fill(0);
    givens = Array(81).fill(false);
    status = '';
    solved = false;
    selectedIndex = null;
  }

  // Sample "easy" puzzle for the demo
  const SAMPLE = [
    5,3,0, 0,7,0, 0,0,0,
    6,0,0, 1,9,5, 0,0,0,
    0,9,8, 0,0,0, 0,6,0,

    8,0,0, 0,6,0, 0,0,3,
    4,0,0, 8,0,3, 0,0,1,
    7,0,0, 0,2,0, 0,0,6,

    0,6,0, 0,0,0, 2,8,0,
    0,0,0, 4,1,9, 0,0,5,
    0,0,0, 0,8,0, 0,7,9,
  ];

  function loadSample() {
    cells = [...SAMPLE];
    givens = SAMPLE.map(v => v !== 0);
    status = '';
    solved = false;
    selectedIndex = null;
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="flex-1 max-w-6xl w-full mx-auto px-4 md:px-8 py-6">

  <!-- Header Area -->
  <div class="flex flex-col md:flex-row justify-between items-start md:items-end mb-8 gap-4">
    <div>
      <h1 class="text-headline-md font-bold text-on-surface mb-1">Classic Sudoku</h1>
      <div class="flex items-center gap-3">
        <span class="px-3 py-1 bg-surface-container-high rounded-full text-[12px] font-bold text-on-surface-variant">Medium</span>
        <div class="flex items-center gap-1 text-on-surface-variant">
          <span class="material-symbols-outlined text-[16px]">timer</span>
          <span class="font-label-mono text-label-mono">{timerText}</span>
        </div>
      </div>
    </div>

    <!-- Status Indicator -->
    {#if status}
      <div id="puzzle-status" class="flex items-center gap-2 px-4 py-2 rounded-xl transition-all {solved ? 'bg-primary-container/30 text-on-primary-container' : 'bg-error-container/30 text-error'}">
        <span class="material-symbols-outlined text-[18px]">{solved ? 'verified' : 'warning'}</span>
        <span class="font-label-mono text-[12px]">{status}</span>
      </div>
    {/if}
  </div>

  <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

    <!-- Left Side Grid Area -->
    <div class="md:col-span-9 flex flex-col items-center">
      <div class="w-full max-w-[600px] bg-white p-2 md:p-4 rounded-3xl shadow-sm border border-outline-variant">

        <!-- Grid -->
        <div id="grid" class="sudoku-grid rounded-xl overflow-hidden">
          {#each cells as cell, i}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="sudoku-cell font-display-grid-mobile text-display-grid-mobile md:font-display-grid md:text-display-grid {givens[i] ? 'fixed' : ''} {selectedIndex === i ? 'active' : ''} {cell !== 0 && !givens[i] ? 'text-secondary' : ''}"
              onclick={() => handleCellClick(i)}
            >
              {cell === 0 ? '' : cell}
            </div>
          {/each}
        </div>

      </div>

      <!-- Primary Action Mobile/Tablet Row -->
      <div class="w-full mt-6 flex gap-4 md:hidden">
        <button onclick={solvePuzzle} class="flex-1 py-4 bg-primary text-on-primary rounded-xl font-bold shadow-md active:scale-95 transition-all">
          Solve
        </button>
        <button onclick={loadSample} class="flex-1 py-4 bg-surface-container-high text-on-surface rounded-xl font-bold border border-outline-variant active:scale-95 transition-all">
          New Game
        </button>
      </div>
    </div>

    <!-- Right Side Panel -->
    <div class="md:col-span-3 flex flex-col gap-6">
      <div class="bg-surface-container-low p-6 rounded-3xl border border-outline-variant shadow-sm w-full">
        <div class="flex justify-between items-center mb-6">
          <h3 class="font-bold text-on-surface-variant">Controls</h3>
        </div>
        <div class="flex flex-col gap-3">
          <button onclick={clearSelectedCell} class="w-full bg-error-container/20 text-error hover:bg-error-container hover:text-on-error-container border border-error/20 flex items-center justify-center rounded-2xl py-3 font-bold transition-all">
            <span class="material-symbols-outlined mr-2">backspace</span> Clear Cell
          </button>
          <button onclick={clearBoard} class="w-full bg-surface-container hover:bg-surface-container-high text-on-surface flex items-center justify-center rounded-2xl py-3 font-bold transition-all mt-2">
            Clear Board
          </button>
        </div>
      </div>

      <!-- Primary Action Desktop -->
      <div class="hidden md:flex flex-col gap-3">
        <button onclick={solvePuzzle} class="w-full py-4 bg-primary text-on-primary rounded-xl font-bold shadow-lg hover:shadow-xl active:scale-95 transition-all flex items-center justify-center gap-2">
          <span class="material-symbols-outlined" style="font-variation-settings: 'FILL' 1;">auto_awesome</span>
          Solve Puzzle
        </button>
        <button onclick={loadSample} class="w-full py-4 bg-white text-primary border-2 border-primary rounded-xl font-bold hover:bg-primary-container/10 active:scale-95 transition-all flex items-center justify-center gap-2">
          <span class="material-symbols-outlined">add</span>
          New Game
        </button>
      </div>
    </div>

  </div>
</div>

<!-- BottomNavBar (Mobile Only) -->
<nav class="md:hidden fixed bottom-0 left-0 w-full z-50 flex justify-around items-center px-4 pb-4 pt-2 bg-surface-container rounded-t-xl shadow-lg">
  <button onclick={solvePuzzle} class="flex flex-col items-center justify-center bg-primary-container text-on-primary-container rounded-full px-5 py-1 active:scale-90 transition-transform">
    <span class="material-symbols-outlined">lightbulb</span>
    <span class="font-label-mono text-label-mono">Solve</span>
  </button>
  <button onclick={clearBoard} class="flex flex-col items-center justify-center text-on-surface-variant active:scale-90 transition-transform">
    <span class="material-symbols-outlined">layers_clear</span>
    <span class="font-label-mono text-label-mono">Clear</span>
  </button>
  <button onclick={loadSample} class="flex flex-col items-center justify-center text-on-surface-variant active:scale-90 transition-transform">
    <span class="material-symbols-outlined">restart_alt</span>
    <span class="font-label-mono text-label-mono">Reset</span>
  </button>
  <button class="flex flex-col items-center justify-center text-on-surface-variant active:scale-90 transition-transform">
    <span class="material-symbols-outlined">history</span>
    <span class="font-label-mono text-label-mono">History</span>
  </button>
</nav>

<style>
  /* Ensure main view doesn't get covered by BottomNavBar on mobile */
  :global(body) {
    padding-bottom: 80px;
  }
  @media (min-width: 768px) {
    :global(body) {
      padding-bottom: 0;
    }
  }
</style>
