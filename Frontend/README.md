# Poneglyph — Frontend

The web client for Poneglyph, built with **React 19**, **Vite 8**, and **Tailwind CSS v4**.
It ships a prebuilt WebAssembly module (`Agartha`) in [`src/pkg/`](src/pkg/) that is
loaded at runtime — no extra WASM toolchain is required to run the app.

## Prerequisites

| Tool | Version used | Notes |
|------|--------------|-------|
| [Bun](https://bun.sh) | `1.3.13`+ | Package manager and script runner |
| Node.js | `22.x` | Provides the runtime Vite needs under the hood |

Install Bun on Linux (if you don't have it):

```bash
curl -fsSL https://bun.sh/install | bash
```

Verify your toolchain:

```bash
bun --version
node --version
```

## Setup

From the repository root, move into this folder and install dependencies:

```bash
cd Poneglyph/Frontend
bun install
```

This reads [`package.json`](package.json) and creates `node_modules/`.

## Run (development)

Start the Vite dev server with hot module replacement:

```bash
bun run dev
```

Vite prints a local URL (default <http://localhost:5173>). Open it in your browser;
edits to files in [`src/`](src/) reload automatically.

## Build (production)

Produce an optimized, minified bundle in `dist/`:

```bash
bun run build
```

Preview the production build locally before deploying:

```bash
bun run preview
```

## Lint

Run ESLint across the project:

```bash
bun run lint
```

## Available scripts

| Command | Description |
|---------|-------------|
| `bun run dev` | Start the dev server (Vite) |
| `bun run build` | Build the production bundle into `dist/` |
| `bun run preview` | Serve the production build locally |
| `bun run lint` | Lint all source files with ESLint |

## Project structure

```
Frontend/
├── index.html          # App entry HTML
├── vite.config.js      # Vite + React + Tailwind plugins
├── eslint.config.js    # ESLint flat config
├── public/             # Static assets served as-is
└── src/
    ├── main.jsx        # React entry point
    ├── App.jsx         # Root component
    ├── components/     # UI components
    ├── utils/          # Helpers (e.g. Prism syntax integration)
    ├── assets/         # Imported assets
    ├── index.css       # Tailwind layers + global styles
    └── pkg/            # Prebuilt Agartha WASM module + JS bindings
```

## Troubleshooting

- **Port already in use** — run `bun run dev -- --port 3000` to pick another port.
- **Stale dependencies after a pull** — delete `node_modules/` and `bun.lock`, then
  re-run `bun install`.
- **WASM fails to load in the browser** — ensure you're accessing the app over the
  Vite dev server (not by opening `index.html` directly); WASM requires an HTTP origin.
