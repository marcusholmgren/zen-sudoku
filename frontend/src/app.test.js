// @vitest-environment jsdom
import { describe, it, expect, beforeEach, beforeAll } from 'vitest';
import initWasm, { solve, generate } from './wasm/zen_sudoku_solver.js';
import fs from 'fs';
import path from 'path';

describe('Sudoku WASM Solver & Generator', () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(__dirname, './wasm/zen_sudoku_solver_bg.wasm');
    const wasmBuffer = fs.readFileSync(wasmPath);
    await initWasm(wasmBuffer);
  });

  it('generates correct number of givens for Easy difficulty', () => {
    const puzzle = generate('easy');
    expect(puzzle.length).toBe(81);
    const givens = puzzle.filter(v => v !== 0).length;
    expect(givens).toBe(40);
  });

  it('generates correct number of givens for Medium difficulty', () => {
    const puzzle = generate('medium');
    expect(puzzle.length).toBe(81);
    const givens = puzzle.filter(v => v !== 0).length;
    expect(givens).toBe(32);
  });

  it('generates correct number of givens for Hard difficulty', () => {
    const puzzle = generate('hard');
    expect(puzzle.length).toBe(81);
    const givens = puzzle.filter(v => v !== 0).length;
    expect(givens).toBe(26);
  });

  it('solves a generated puzzle successfully', () => {
    const puzzle = generate('medium');
    const solution = solve(puzzle);
    expect(solution.length).toBe(81);
    expect(solution.includes(0)).toBe(false);

    // Verify givens match solution
    for (let i = 0; i < 81; i++) {
      if (puzzle[i] !== 0) {
        expect(solution[i]).toBe(puzzle[i]);
      }
    }
  });
});

describe('SessionStorage Puzzle History Persistence', () => {
  beforeEach(() => {
    sessionStorage.clear();
  });

  it('saves completed puzzle entries persistently in sessionStorage', () => {
    const historyKey = 'sudoku_history';
    expect(sessionStorage.getItem(historyKey)).toBeNull();

    const entry1 = {
      id: 'puzzle_1001',
      difficulty: 'Easy',
      timeTakenSeconds: 120,
      timeTakenFormatted: '02:00',
      date: 'Mar 30, 2025 02:00 PM',
      completedAt: new Date().toISOString()
    };

    const entry2 = {
      id: 'puzzle_1002',
      difficulty: 'Hard',
      timeTakenSeconds: 450,
      timeTakenFormatted: '07:30',
      date: 'Mar 30, 2025 02:15 PM',
      completedAt: new Date().toISOString()
    };

    let history = [entry1];
    sessionStorage.setItem(historyKey, JSON.stringify(history));

    let retrieved = JSON.parse(sessionStorage.getItem(historyKey));
    expect(retrieved.length).toBe(1);
    expect(retrieved[0].difficulty).toBe('Easy');

    // Add second puzzle
    retrieved.unshift(entry2);
    sessionStorage.setItem(historyKey, JSON.stringify(retrieved));

    let updated = JSON.parse(sessionStorage.getItem(historyKey));
    expect(updated.length).toBe(2);
    expect(updated[0].difficulty).toBe('Hard');
    expect(updated[1].difficulty).toBe('Easy');
  });
});

describe('Daily Challenge Timer Logic', () => {
  it('handles timer start, stop, and completion termination correctly', () => {
    let timerRunning = false;
    let secondsElapsed = 0;
    let solved = false;

    function startTimer() {
      if (solved) return false;
      timerRunning = true;
      return true;
    }

    function stopTimer() {
      timerRunning = false;
    }

    // Initial state
    expect(timerRunning).toBe(false);

    // Start timer
    expect(startTimer()).toBe(true);
    expect(timerRunning).toBe(true);

    // Stop timer
    stopTimer();
    expect(timerRunning).toBe(false);

    // Complete puzzle auto-stops timer
    startTimer();
    expect(timerRunning).toBe(true);
    solved = true;
    stopTimer(); // auto-stop on completion
    expect(timerRunning).toBe(false);

    // Attempting to start timer when already solved is blocked
    const canStart = startTimer();
    expect(canStart).toBe(false);
    expect(timerRunning).toBe(false);
  });
});

describe('Custom Blank Board Logic & Solver Integration', () => {
  beforeAll(async () => {
    const wasmPath = path.resolve(__dirname, './wasm/zen_sudoku_solver_bg.wasm');
    const wasmBuffer = fs.readFileSync(wasmPath);
    await initWasm(wasmBuffer);
  });

  it('creates a blank 81-cell board for custom puzzle input', () => {
    const blankBoard = new Array(81).fill(0);
    expect(blankBoard.length).toBe(81);
    expect(blankBoard.every(v => v === 0)).toBe(true);
  });

  it('solves a custom user puzzle successfully and preserves user givens', () => {
    // Custom puzzle with valid givens
    const customPuzzle = [
      5, 3, 0, 0, 7, 0, 0, 0, 0,
      6, 0, 0, 1, 9, 5, 0, 0, 0,
      0, 9, 8, 0, 0, 0, 0, 6, 0,
      8, 0, 0, 0, 6, 0, 0, 0, 3,
      4, 0, 0, 8, 0, 3, 0, 0, 1,
      7, 0, 0, 0, 2, 0, 0, 0, 6,
      0, 6, 0, 0, 0, 0, 2, 8, 0,
      0, 0, 0, 4, 1, 9, 0, 0, 5,
      0, 0, 0, 0, 8, 0, 0, 7, 9
    ];

    const solution = solve(customPuzzle);
    expect(solution.length).toBe(81);
    expect(solution.includes(0)).toBe(false);

    // Verify user entered givens are preserved in the solution
    for (let i = 0; i < 81; i++) {
      if (customPuzzle[i] !== 0) {
        expect(solution[i]).toBe(customPuzzle[i]);
      }
    }
  });

  it('detects invalid custom puzzle inputs with conflicting digits', () => {
    const invalidCustomPuzzle = new Array(81).fill(0);
    invalidCustomPuzzle[0] = 5;
    invalidCustomPuzzle[1] = 5; // Duplicate in row 0

    const solution = solve(invalidCustomPuzzle);
    expect(solution.length).toBe(0);
  });

  it('supports saving Custom difficulty puzzles in SessionStorage', () => {
    const historyKey = 'sudoku_history';
    sessionStorage.clear();

    const customEntry = {
      id: 'puzzle_custom_1',
      difficulty: 'Custom',
      timeTakenSeconds: 95,
      timeTakenFormatted: '01:35',
      date: 'Mar 30, 2025 03:00 PM',
      completedAt: new Date().toISOString()
    };

    sessionStorage.setItem(historyKey, JSON.stringify([customEntry]));
    const retrieved = JSON.parse(sessionStorage.getItem(historyKey));
    expect(retrieved.length).toBe(1);
    expect(retrieved[0].difficulty).toBe('Custom');
  });
});
