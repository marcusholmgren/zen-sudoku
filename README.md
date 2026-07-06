# zen-sudoku

A Sudoku solver single-page application built with **Svelte**, **TailwindCSS**, and a **Rust WASM** backend. Hosted on [GitHub Pages](https://marcusholmgren.github.io/zen-sudoku/).

## Project structure

```
zen-sudoku/
├── backend/       # Rust library — compiled to WebAssembly
│   ├── src/
│   │   └── lib.rs         # Backtracking Sudoku solver + wasm-bindgen bindings
│   └── Cargo.toml
└── frontend/      # Svelte + Vite + TailwindCSS SPA
    ├── src/
    │   ├── App.svelte             # Root component (lazy-loads WASM)
    │   ├── lib/
    │   │   └── SudokuBoard.svelte # 9×9 board UI + controls
    │   ├── app.css                # TailwindCSS entry point
    │   └── main.js
    ├── index.html
    └── vite.config.js
```

> **Note:** `frontend/src/wasm/` is generated at build time and is git-ignored.

## Development

### Prerequisites

- [Rust](https://rustup.rs/) with the `wasm32-unknown-unknown` target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [Node.js](https://nodejs.org/) 18+

### 1 — Build the WASM package

```bash
cd backend
wasm-pack build --target web --out-dir ../frontend/src/wasm
```

### 2 — Start the frontend dev server

```bash
cd frontend
npm install
npm run dev
```

### Run Rust tests

```bash
cd backend
cargo test
```

## Deployment

Pushes to `main` automatically build and deploy to GitHub Pages via the
[`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) workflow.
