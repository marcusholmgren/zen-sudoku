<script>
  import { onMount, onDestroy } from 'svelte';
  import SudokuBoard from './lib/SudokuBoard.svelte';

  let solverReady = $state(false);
  let solveWasm = $state(null);
  let generateWasm = $state(null);

  // Game & Timer State
  let timerRunning = $state(false);
  let secondsElapsed = $state(0);
  let timerInterval = null;
  let difficulty = $state('Medium');
  let selectedDifficulty = $state('Medium');
  let solved = $state(false);
  let boardComponent = $state(null);

  // Modals
  let showNewGameModal = $state(false);
  let showHistoryModal = $state(false);
  let showHelpModal = $state(false);
  let historyList = $state([]);

  onMount(async () => {
    try {
      const wasm = await import('./wasm/zen_sudoku_solver.js');
      await wasm.default();
      solveWasm = wasm.solve;
      generateWasm = wasm.generate;
      solverReady = true;
    } catch (e) {
      console.error('Failed to load WASM solver:', e);
    }
  });

  onDestroy(() => {
    if (timerInterval) {
      clearInterval(timerInterval);
    }
  });

  function startTimer() {
    if (solved) return;
    if (timerRunning) return;
    timerRunning = true;
    timerInterval = setInterval(() => {
      secondsElapsed += 1;
    }, 1000);
  }

  function stopTimer() {
    timerRunning = false;
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
  }

  function toggleTimer() {
    if (timerRunning) {
      stopTimer();
    } else {
      startTimer();
    }
  }

  function resetTimer() {
    stopTimer();
    secondsElapsed = 0;
  }

  function openNewGameModal() {
    selectedDifficulty = difficulty;
    showNewGameModal = true;
  }

  function startNewGameWithSelectedDifficulty() {
    difficulty = selectedDifficulty;
    resetTimer();
    solved = false;
    if (boardComponent && boardComponent.resetPuzzleWithDifficulty) {
      boardComponent.resetPuzzleWithDifficulty(difficulty);
    }
    showNewGameModal = false;
  }

  function openHistoryModal() {
    loadHistoryFromSession();
    showHistoryModal = true;
  }

  function loadHistoryFromSession() {
    try {
      const historyJson = sessionStorage.getItem('sudoku_history');
      historyList = historyJson ? JSON.parse(historyJson) : [];
    } catch (e) {
      console.error('Failed to read history from SessionStorage:', e);
      historyList = [];
    }
  }

  function clearHistory() {
    try {
      sessionStorage.removeItem('sudoku_history');
      historyList = [];
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }

  function getBestTimeFormatted(list) {
    if (!list || list.length === 0) return '--:--';
    const validTimes = list
      .map(item => item.timeTakenSeconds)
      .filter(t => typeof t === 'number' && t >= 0);
    if (validTimes.length === 0) return '--:--';
    const minSecs = Math.min(...validTimes);
    const mins = Math.floor(minSecs / 60);
    const secs = minSecs % 60;
    return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
  }
</script>

<!-- TopAppBar -->
<header class="fixed top-0 left-0 w-full z-50 flex justify-between items-center px-4 md:px-8 py-3 bg-surface shadow-sm border-b border-outline-variant/40">
  <div class="flex items-center gap-4">
    <span class="font-display-grid text-display-grid font-bold text-primary">ZenSudoku</span>
  </div>
  <div class="flex items-center gap-2">
    <!-- Help Button (Question Mark Icon - Option A) -->
    <button
      onclick={() => showHelpModal = true}
      title="How to Play & Help"
      class="p-2 rounded-full hover:bg-surface-container-high transition-colors active:scale-95 text-on-surface-variant flex items-center justify-center"
    >
      <span class="material-symbols-outlined">help_outline</span>
    </button>
    <button
      onclick={openHistoryModal}
      title="Puzzle History"
      class="p-2 rounded-full hover:bg-surface-container-high transition-colors active:scale-95 text-on-surface-variant flex items-center justify-center lg:hidden"
    >
      <span class="material-symbols-outlined">history</span>
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
    <button
      onclick={() => { showNewGameModal = false; showHistoryModal = false; showHelpModal = false; }}
      class="flex items-center gap-4 px-4 py-3 bg-secondary-container text-on-secondary-container rounded-xl font-bold transition-colors"
    >
      <span class="material-symbols-outlined">grid_on</span>
      Current Puzzle
    </button>
    <button
      onclick={openHistoryModal}
      class="flex items-center gap-4 px-4 py-3 hover:bg-surface-container text-on-surface-variant hover:text-on-surface rounded-xl transition-colors"
    >
      <span class="material-symbols-outlined">history</span>
      History &amp; Stats
    </button>
  </div>
  <div class="mt-auto px-6 pb-6">
    <!-- Feature 1: Daily Challenge Timer card -->
    <div class="bg-surface-container p-4 rounded-2xl border border-outline-variant/50 shadow-xs">
      <div class="flex items-center justify-between mb-1">
        <p class="text-sm font-bold text-on-surface">Daily Challenge</p>
        <span class="material-symbols-outlined text-primary text-[18px]">timer</span>
      </div>
      <p class="text-xs text-on-surface-variant mb-3">Complete in under 5 mins</p>
      <button
        onclick={toggleTimer}
        disabled={solved}
        class="w-full py-2 rounded-lg text-sm font-bold shadow-sm transition-all active:scale-95 disabled:opacity-50 {timerRunning ? 'bg-error-container text-error hover:bg-error/20' : 'bg-primary text-on-primary hover:bg-primary/90'}"
      >
        {timerRunning ? 'Stop' : 'Start'}
      </button>
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
    <SudokuBoard
      bind:this={boardComponent}
      solve={solveWasm}
      generate={generateWasm}
      timerRunning={timerRunning}
      secondsElapsed={secondsElapsed}
      difficulty={difficulty}
      bind:solved={solved}
      onToggleTimer={toggleTimer}
      onSolveCompleted={stopTimer}
      onOpenNewGameModal={openNewGameModal}
      onOpenHistory={openHistoryModal}
    />
  {/if}
</main>

<!-- Modals -->

<!-- Feature 3: Game Mode Selection Modal -->
{#if showNewGameModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-fade-in backdrop-blur-xs">
    <div class="bg-surface rounded-3xl p-6 md:p-8 max-w-md w-full shadow-2xl border border-outline-variant">
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-xl font-bold text-on-surface">Select Game Mode</h2>
        <button onclick={() => showNewGameModal = false} class="p-2 text-on-surface-variant hover:text-on-surface rounded-full">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <div class="flex flex-col gap-3 mb-6">
        <button
          onclick={() => selectedDifficulty = 'Easy'}
          class="flex justify-between items-center p-4 rounded-2xl border-2 transition-all {selectedDifficulty === 'Easy' ? 'border-primary bg-primary-container/20 shadow-xs' : 'border-outline-variant hover:border-outline'}"
        >
          <div class="text-left">
            <div class="font-bold text-on-surface">Easy Mode</div>
            <div class="text-xs text-on-surface-variant">40 givens • Great for beginners</div>
          </div>
          <span class="px-2.5 py-1 bg-green-100 text-green-800 font-bold text-xs rounded-full">Easy</span>
        </button>

        <button
          onclick={() => selectedDifficulty = 'Medium'}
          class="flex justify-between items-center p-4 rounded-2xl border-2 transition-all {selectedDifficulty === 'Medium' ? 'border-primary bg-primary-container/20 shadow-xs' : 'border-outline-variant hover:border-outline'}"
        >
          <div class="text-left">
            <div class="font-bold text-on-surface">Medium Mode</div>
            <div class="text-xs text-on-surface-variant">32 givens • Standard challenge</div>
          </div>
          <span class="px-2.5 py-1 bg-blue-100 text-blue-800 font-bold text-xs rounded-full">Medium</span>
        </button>

        <button
          onclick={() => selectedDifficulty = 'Hard'}
          class="flex justify-between items-center p-4 rounded-2xl border-2 transition-all {selectedDifficulty === 'Hard' ? 'border-primary bg-primary-container/20 shadow-xs' : 'border-outline-variant hover:border-outline'}"
        >
          <div class="text-left">
            <div class="font-bold text-on-surface">Hard Mode</div>
            <div class="text-xs text-on-surface-variant">26 givens • For experienced solvers</div>
          </div>
          <span class="px-2.5 py-1 bg-amber-100 text-amber-800 font-bold text-xs rounded-full">Hard</span>
        </button>

        <button
          onclick={() => selectedDifficulty = 'Custom'}
          class="flex justify-between items-center p-4 rounded-2xl border-2 transition-all {selectedDifficulty === 'Custom' ? 'border-primary bg-primary-container/20 shadow-xs' : 'border-outline-variant hover:border-outline'}"
        >
          <div class="text-left">
            <div class="font-bold text-on-surface">Custom / Blank Board</div>
            <div class="text-xs text-on-surface-variant">0 givens • Input your own puzzle to solve</div>
          </div>
          <span class="px-2.5 py-1 bg-purple-100 text-purple-800 font-bold text-xs rounded-full">Custom</span>
        </button>
      </div>

      <div class="flex gap-3">
        <button onclick={() => showNewGameModal = false} class="flex-1 py-3 bg-surface-container hover:bg-surface-container-high rounded-xl font-bold text-on-surface transition-all">
          Cancel
        </button>
        <button onclick={startNewGameWithSelectedDifficulty} class="flex-1 py-3 bg-primary text-on-primary rounded-xl font-bold hover:bg-primary/90 transition-all shadow-md">
          Start Game
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Feature 2: History & Stats Modal -->
{#if showHistoryModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-fade-in backdrop-blur-xs">
    <div class="bg-surface rounded-3xl p-6 md:p-8 max-w-lg w-full max-h-[85vh] flex flex-col shadow-2xl border border-outline-variant">
      <div class="flex justify-between items-center mb-4">
        <div>
          <h2 class="text-xl font-bold text-on-surface">Puzzle History</h2>
          <p class="text-xs text-on-surface-variant">Persistently saved in SessionStorage</p>
        </div>
        <button onclick={() => showHistoryModal = false} class="p-2 text-on-surface-variant hover:text-on-surface rounded-full">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <!-- Summary cards -->
      <div class="grid grid-cols-2 gap-3 mb-4">
        <div class="bg-surface-container p-3.5 rounded-2xl border border-outline-variant/60 text-center">
          <div class="text-2xl font-bold text-primary">{historyList.length}</div>
          <div class="text-xs text-on-surface-variant font-medium">Puzzles Solved</div>
        </div>
        <div class="bg-surface-container p-3.5 rounded-2xl border border-outline-variant/60 text-center">
          <div class="text-2xl font-bold text-secondary">{getBestTimeFormatted(historyList)}</div>
          <div class="text-xs text-on-surface-variant font-medium">Best Time</div>
        </div>
      </div>

      <!-- History list -->
      <div class="flex-1 overflow-y-auto min-h-[200px] flex flex-col gap-2 pr-1 mb-4">
        {#if historyList.length === 0}
          <div class="flex-1 flex flex-col items-center justify-center text-center p-8 text-on-surface-variant">
            <span class="material-symbols-outlined text-4xl mb-2 text-outline">history</span>
            <p class="font-bold">No Solved Puzzles Yet</p>
            <p class="text-xs mt-1">Complete a puzzle to see your saved results here!</p>
          </div>
        {:else}
          {#each historyList as item, index}
            <div class="p-3.5 bg-surface-container-low rounded-2xl border border-outline-variant flex items-center justify-between">
              <div>
                <div class="flex items-center gap-2 mb-1">
                  <span class="px-2 py-0.5 text-[11px] font-bold rounded-full capitalize {item.difficulty?.toLowerCase() === 'easy' ? 'bg-green-100 text-green-800' : item.difficulty?.toLowerCase() === 'hard' ? 'bg-amber-100 text-amber-800' : 'bg-blue-100 text-blue-800'}">
                    {item.difficulty || 'Medium'}
                  </span>
                  <span class="text-xs text-on-surface-variant">{item.date || ''}</span>
                </div>
                <div class="text-xs font-mono text-on-surface-variant">
                  ID: #{item.id?.replace('puzzle_', '').slice(-6)}
                </div>
              </div>
              <div class="text-right">
                <div class="text-sm font-bold font-mono text-primary flex items-center gap-1 justify-end">
                  <span class="material-symbols-outlined text-[16px]">timer</span>
                  {item.timeTakenFormatted || '00:00'}
                </div>
                <div class="text-[10px] text-on-surface-variant">Completed</div>
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <div class="flex justify-between items-center pt-3 border-t border-outline-variant">
        {#if historyList.length > 0}
          <button onclick={clearHistory} class="text-xs font-bold text-error hover:underline px-2 py-1">
            Clear History
          </button>
        {:else}
          <div></div>
        {/if}
        <button onclick={() => showHistoryModal = false} class="px-5 py-2.5 bg-primary text-on-primary font-bold rounded-xl text-sm hover:bg-primary/90 transition-all">
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Feature 4: UX Enhancement (Question Mark Icon - Option A Modal) -->
{#if showHelpModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-fade-in backdrop-blur-xs">
    <div class="bg-surface rounded-3xl p-6 md:p-8 max-w-lg w-full max-h-[85vh] flex flex-col shadow-2xl border border-outline-variant">
      <div class="flex justify-between items-center mb-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-primary-container text-on-primary-container flex items-center justify-center">
            <span class="material-symbols-outlined">help_outline</span>
          </div>
          <div>
            <h2 class="text-xl font-bold text-on-surface">How to Play &amp; Help</h2>
            <p class="text-xs text-on-surface-variant">ZenSudoku Guide &amp; Controls</p>
          </div>
        </div>
        <button onclick={() => showHelpModal = false} class="p-2 text-on-surface-variant hover:text-on-surface rounded-full">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto space-y-4 pr-1 mb-6 text-sm text-on-surface">
        <section class="bg-surface-container p-4 rounded-2xl border border-outline-variant/60">
          <h3 class="font-bold text-primary mb-1 flex items-center gap-1.5">
            <span class="material-symbols-outlined text-[18px]">grid_4x4</span>
            Sudoku Objective
          </h3>
          <p class="text-xs text-on-surface-variant leading-relaxed">
            Fill the 9x9 grid with digits so that each column, each row, and each of the nine 3x3 subgrids contain all digits from 1 to 9 without repeating.
          </p>
        </section>

        <section class="bg-surface-container p-4 rounded-2xl border border-outline-variant/60">
          <h3 class="font-bold text-primary mb-1 flex items-center gap-1.5">
            <span class="material-symbols-outlined text-[18px]">keyboard</span>
            Controls &amp; Navigation
          </h3>
          <ul class="text-xs text-on-surface-variant space-y-1.5 list-disc list-inside">
            <li><strong class="text-on-surface">Select Cell:</strong> Click or tap any empty non-fixed cell.</li>
            <li><strong class="text-on-surface">Enter Number:</strong> Press keys 1-9 on your keyboard or tap the on-screen keypad.</li>
            <li><strong class="text-on-surface">Navigate Grid:</strong> Use Arrow keys (Up, Down, Left, Right) to move focus.</li>
            <li><strong class="text-on-surface">Clear Cell:</strong> Press Backspace/Delete or use the Clear Cell button.</li>
          </ul>
        </section>

        <section class="bg-surface-container p-4 rounded-2xl border border-outline-variant/60">
          <h3 class="font-bold text-primary mb-1 flex items-center gap-1.5">
            <span class="material-symbols-outlined text-[18px]">settings_suggest</span>
            Features &amp; Settings
          </h3>
          <ul class="text-xs text-on-surface-variant space-y-1.5 list-disc list-inside">
            <li><strong class="text-on-surface">Daily Challenge Timer:</strong> Use the Start/Stop button in the sidebar or timer badge to track your solve speed. Automatically stops on completion.</li>
            <li><strong class="text-on-surface">Game Modes:</strong> Click "+ New Game" to choose Easy (40 givens), Medium (32 givens), Hard (26 givens), or Custom (blank board to add your own puzzle).</li>
            <li><strong class="text-on-surface">AI Auto-Solve:</strong> Click "Solve Puzzle" to complete the grid instantly using the Rust WASM engine.</li>
            <li><strong class="text-on-surface">Session Storage:</strong> All completed puzzles are automatically stored in Session Storage for history tracking.</li>
          </ul>
        </section>
      </div>

      <div class="flex justify-end pt-3 border-t border-outline-variant">
        <button onclick={() => showHelpModal = false} class="px-6 py-2.5 bg-primary text-on-primary font-bold rounded-xl text-sm hover:bg-primary/90 transition-all shadow-md">
          Got It
        </button>
      </div>
    </div>
  </div>
{/if}
