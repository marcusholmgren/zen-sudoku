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

  /** Return extra border classes to draw thick 3×3 box borders */
  function borderClasses(index) {
    const row = Math.floor(index / 9);
    const col = index % 9;
    const classes = [];
    if (col % 3 === 0 && col !== 0) classes.push('border-l-2 border-l-slate-700 dark:border-l-slate-300');
    if (row % 3 === 0 && row !== 0) classes.push('border-t-2 border-t-slate-700 dark:border-t-slate-300');
    return classes.join(' ');
  }

  function handleInput(index, event) {
    const val = parseInt(event.target.value, 10);
    cells[index] = isNaN(val) || val < 1 || val > 9 ? 0 : val;
    givens[index] = cells[index] !== 0;
    solved = false;
    status = '';
  }

  function handleKeydown(event) {
    if (!/^[1-9]$/.test(event.key) && event.key !== 'Backspace' && event.key !== 'Delete' && event.key !== 'Tab') {
      if (!event.ctrlKey && !event.metaKey) {
        event.preventDefault();
      }
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
    }
  }

  function clearBoard() {
    cells = Array(81).fill(0);
    givens = Array(81).fill(false);
    status = '';
    solved = false;
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
  }
</script>

<div class="flex flex-col items-center gap-6">
  <!-- Board -->
  <div
    class="grid grid-cols-9 border-2 border-slate-700 dark:border-slate-300 shadow-lg"
    role="grid"
    aria-label="Sudoku board"
  >
    {#each cells as cell, i}
      <div
        class="relative w-10 h-10 border border-slate-300 dark:border-slate-600 {borderClasses(i)}"
        role="gridcell"
      >
        <input
          type="text"
          inputmode="numeric"
          maxlength="1"
          aria-label="Cell {Math.floor(i / 9) + 1},{(i % 9) + 1}"
          value={cell === 0 ? '' : cell}
          readonly={givens[i] && !solved}
          class="absolute inset-0 w-full h-full text-center text-base font-medium
                 focus:outline-none focus:bg-blue-50 dark:focus:bg-blue-900/30
                 {givens[i]
                   ? 'bg-slate-100 dark:bg-slate-800 text-slate-800 dark:text-slate-200 font-semibold'
                   : solved
                     ? 'bg-white dark:bg-slate-900 text-blue-600 dark:text-blue-400'
                     : 'bg-white dark:bg-slate-900 text-slate-700 dark:text-slate-300'}
                 "
          oninput={(e) => handleInput(i, e)}
          onkeydown={handleKeydown}
        />
      </div>
    {/each}
  </div>

  <!-- Status -->
  {#if status}
    <p
      class="text-sm font-medium {solved ? 'text-green-600 dark:text-green-400' : 'text-red-500 dark:text-red-400'}"
      role="status"
      aria-live="polite"
    >
      {status}
    </p>
  {/if}

  <!-- Controls -->
  <div class="flex gap-3 flex-wrap justify-center">
    <button
      onclick={loadSample}
      class="px-4 py-2 rounded-lg bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600
             text-slate-800 dark:text-slate-200 text-sm font-medium transition-colors"
    >
      Load Sample
    </button>
    <button
      onclick={solvePuzzle}
      class="px-5 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-semibold
             transition-colors shadow"
    >
      Solve
    </button>
    <button
      onclick={clearBoard}
      class="px-4 py-2 rounded-lg bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600
             text-slate-800 dark:text-slate-200 text-sm font-medium transition-colors"
    >
      Clear
    </button>
  </div>

  <p class="text-xs text-slate-400 dark:text-slate-500 max-w-xs text-center">
    Enter digits 1–9 in the empty cells, then click <strong>Solve</strong>. Or click
    <strong>Load Sample</strong> to try a pre-filled puzzle.
  </p>
</div>
