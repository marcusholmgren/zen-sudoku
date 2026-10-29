<script>
  import { onMount } from 'svelte';

  /**
   * @type {{
   *   solve: (board: Uint8Array) => Uint8Array,
   *   generate: (difficulty: string) => Uint8Array,
   *   timerRunning?: boolean,
   *   secondsElapsed?: number,
   *   difficulty?: string,
   *   solved?: boolean,
   *   onToggleTimer?: () => void,
   *   onSolveCompleted?: () => void,
   *   onOpenNewGameModal?: () => void,
   *   onOpenHistory?: () => void
   * }}
   */
  let {
    solve,
    generate,
    timerRunning = false,
    secondsElapsed = 0,
    difficulty = 'Medium',
    solved = $bindable(false),
    onToggleTimer,
    onSolveCompleted,
    onOpenNewGameModal,
    onOpenHistory
  } = $props();

  // 81-cell board; 0 = empty
  let cells = $state(Array(81).fill(0));
  // Which cells are "given" (locked)
  let givens = $state(Array(81).fill(false));
  // Status message
  let status = $state('');
  // Currently selected cell index
  let selectedIndex = $state(null);
  // Prevent duplicate history entries for the same solved instance
  let hasSavedHistory = $state(false);

  // Format time (seconds to MM:SS)
  let timerText = $derived(() => {
    const mins = Math.floor(secondsElapsed / 60);
    const secs = secondsElapsed % 60;
    return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
  });

  // Default sample puzzle fallback if generate isn't available
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

  export function loadBoardForDifficulty(diff) {
    let newBoard;
    if (generate) {
      try {
        const generated = generate(diff.toLowerCase());
        if (generated && generated.length === 81) {
          newBoard = Array.from(generated);
        }
      } catch (e) {
        console.error('Failed to generate puzzle with WASM:', e);
      }
    }
    if (!newBoard) {
      newBoard = [...SAMPLE];
    }
    cells = newBoard;
    givens = newBoard.map(v => v !== 0);
    status = '';
    solved = false;
    selectedIndex = null;
    hasSavedHistory = false;
  }

  // Load puzzle when component mounts
  onMount(() => {
    loadBoardForDifficulty(difficulty);
  });

  export function resetPuzzleWithDifficulty(newDiff) {
    loadBoardForDifficulty(newDiff);
  }

  function handleCellClick(index) {
    if (givens[index]) return; // Don't select fixed cells
    selectedIndex = index;
  }

  function checkCompletion(currentCells) {
    // Check if board has no 0s and is valid solution
    if (currentCells.includes(0)) return false;
    if (!solve) return false;
    const board = new Uint8Array(currentCells);
    const result = solve(board);
    if (!result || result.length === 0) return false;
    for (let i = 0; i < 81; i++) {
      if (currentCells[i] !== result[i]) return false;
    }
    return true;
  }

  function handleSolvedState() {
    solved = true;
    if (!hasSavedHistory) {
      hasSavedHistory = true;
      saveToHistory();
    }
    if (onSolveCompleted) {
      onSolveCompleted();
    }
  }

  function handleKeydown(event) {
    if (selectedIndex === null) return;

    if (/^[1-9]$/.test(event.key) && !givens[selectedIndex]) {
      const val = parseInt(event.key, 10);
      cells[selectedIndex] = val;
      status = '';
      if (checkCompletion(cells)) {
        status = '🎉 Puzzle Completed!';
        handleSolvedState();
      }
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

  function handleNumPadInput(num) {
    if (selectedIndex === null || givens[selectedIndex]) return;
    cells[selectedIndex] = num;
    status = '';
    if (checkCompletion(cells)) {
      status = '🎉 Puzzle Completed!';
      handleSolvedState();
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
      status = '✅ Solved with AI Assist!';
      selectedIndex = null;
      handleSolvedState();
    }
  }

  function clearBoard() {
    cells = cells.map((val, idx) => (givens[idx] ? val : 0));
    status = '';
    solved = false;
    selectedIndex = null;
  }

  function saveToHistory() {
    try {
      const historyJson = sessionStorage.getItem('sudoku_history');
      const history = historyJson ? JSON.parse(historyJson) : [];
      const now = new Date();
      const dateStr = now.toLocaleDateString([], { month: 'short', day: 'numeric', year: 'numeric' }) + ' ' + now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
      const newEntry = {
        id: 'puzzle_' + now.getTime(),
        difficulty: difficulty,
        timeTakenSeconds: secondsElapsed,
        timeTakenFormatted: timerText(),
        date: dateStr,
        completedAt: now.toISOString()
      };
      history.unshift(newEntry);
      sessionStorage.setItem('sudoku_history', JSON.stringify(history));
    } catch (e) {
      console.error('Failed to save to SessionStorage:', e);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="flex-1 max-w-6xl w-full mx-auto px-4 md:px-8 py-6">

  <!-- Header Area -->
  <div class="flex flex-col md:flex-row justify-between items-start md:items-end mb-8 gap-4">
    <div>
      <h1 class="text-headline-md font-bold text-on-surface mb-1">Classic Sudoku</h1>
      <div class="flex items-center gap-3">
        <span class="px-3 py-1 bg-surface-container-high rounded-full text-[12px] font-bold text-on-surface-variant capitalize">{difficulty}</span>
        <div class="flex items-center gap-2 bg-surface-container-low px-3 py-1 rounded-full border border-outline-variant">
          <span class="material-symbols-outlined text-[18px] text-primary">timer</span>
          <span class="font-label-mono text-label-mono text-on-surface font-semibold">{timerText()}</span>
          <button
            onclick={onToggleTimer}
            disabled={solved}
            class="ml-1 px-2.5 py-0.5 text-xs font-bold rounded-md transition-all active:scale-95 disabled:opacity-50 {timerRunning ? 'bg-error-container text-error hover:bg-error/20' : 'bg-primary text-on-primary hover:bg-primary/90'}"
          >
            {timerRunning ? 'Stop' : 'Start'}
          </button>
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

      <!-- Number Pad (On screen input for touch/mouse) -->
      <div class="w-full max-w-[600px] mt-6 grid grid-cols-9 gap-1 md:gap-2">
        {#each [1, 2, 3, 4, 5, 6, 7, 8, 9] as num}
          <button
            onclick={() => handleNumPadInput(num)}
            class="number-pad-btn bg-surface-container hover:bg-primary-container/40 text-on-surface font-bold text-lg md:text-xl rounded-xl flex items-center justify-center border border-outline-variant/60 shadow-xs"
          >
            {num}
          </button>
        {/each}
      </div>

      <!-- Primary Action Mobile/Tablet Row -->
      <div class="w-full mt-6 flex gap-4 md:hidden">
        <button onclick={solvePuzzle} class="flex-1 py-4 bg-primary text-on-primary rounded-xl font-bold shadow-md active:scale-95 transition-all">
          Solve
        </button>
        <button onclick={onOpenNewGameModal} class="flex-1 py-4 bg-surface-container-high text-on-surface rounded-xl font-bold border border-outline-variant active:scale-95 transition-all">
          + New Game
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
          <button onclick={clearBoard} class="w-full bg-surface-container hover:bg-surface-container-high text-on-surface flex items-center justify-center rounded-2xl py-3 font-bold transition-all">
            Reset Answers
          </button>
        </div>
      </div>

      <!-- Primary Action Desktop -->
      <div class="hidden md:flex flex-col gap-3">
        <button onclick={solvePuzzle} class="w-full py-4 bg-primary text-on-primary rounded-xl font-bold shadow-lg hover:shadow-xl active:scale-95 transition-all flex items-center justify-center gap-2">
          <span class="material-symbols-outlined" style="font-variation-settings: 'FILL' 1;">auto_awesome</span>
          Solve Puzzle
        </button>
        <button onclick={onOpenNewGameModal} class="w-full py-4 bg-white text-primary border-2 border-primary rounded-xl font-bold hover:bg-primary-container/10 active:scale-95 transition-all flex items-center justify-center gap-2">
          <span class="material-symbols-outlined">add</span>
          New Game
        </button>
      </div>
    </div>

  </div>
</div>
