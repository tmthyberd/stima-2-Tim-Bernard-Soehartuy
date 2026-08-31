# DOM Traversal Explorer

This is a small web app for exploring how HTML turns into a DOM tree, and how search algorithms move through that tree. You paste in some HTML (or give it a URL to fetch), type a CSS selector, and pick either BFS or DFS. The app parses the page, builds the tree, runs the chosen algorithm, and shows you the traversal step by step on an interactive graph.

It started as a university assignment (Strategi Algoritma) about applying BFS and DFS to a real, visual problem instead of a plain graph on paper.

## How it works

- The backend parses raw HTML into a DOM tree by hand (no external HTML parsing library) and implements its own CSS selector matching, supporting tags, classes, ids, the universal selector `*`, and combinators (` `, `>`, `+`, `~`).
- Breadth-First Search walks the tree level by level using a queue.
- Depth-First Search walks down each branch first, using recursion.
- Both algorithms return which nodes matched the selector and the full order in which nodes were visited, so the frontend can animate the search.

## Tech stack

**Backend**
- Rust
- Axum (web framework)
- Tokio (async runtime)
- Reqwest (for fetching pages by URL)
- Serde / serde_json (JSON handling)

**Frontend**
- React with TypeScript
- Vite
- React Router
- @xyflow/react and dagre (for laying out and rendering the DOM tree as a graph)
- Tailwind CSS

**Other**
- Docker and Docker Compose, for running everything with one command

## Project structure

```
.
├── backend/
│   ├── src/
│   │   ├── algorithms/    BFS and DFS implementations
│   │   ├── models/        DOM tree and node data structures
│   │   ├── parser/        HTML parser
│   │   ├── routes/        API endpoints
│   │   ├── scraper/       fetches HTML from a URL
│   │   └── selectors/     CSS selector parsing and matching
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/
│   ├── src/
│   ├── package.json
│   └── Dockerfile
└── docker-compose.yml
```

## Running it

You'll need Rust and Cargo (1.70 or newer) and Node.js (18 or newer) with npm. Or, if you'd rather skip installing all of that, just use Docker.

### With Docker (easiest)

Make sure Docker is installed and running, then from the project root:

```bash
docker compose up -d --build
```

The frontend will be available at `http://localhost:3000` and the backend at `http://localhost:8080`.

To stop everything:

```bash
docker compose down
```

### Running manually

Start the backend:

```bash
cd backend
cargo run
```

This runs the server at `http://localhost:8080`. For a production build:

```bash
cargo build --release
./target/release/backend
```

In a separate terminal, start the frontend:

```bash
cd frontend
npm install
npm run dev
```

Open `http://localhost:3000` in your browser. To build for production:

```bash
npm run build
```

## Authors

| Name | NIM |
|---|---|
| Fahd Muhammad Zahid | 13524078 |
| Daniel Anindito Nugroho | 13524002 |
| Timothy Bernard Soeharto | 13524092 |
