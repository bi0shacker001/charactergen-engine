# CharacterGen Engine

## Build requirements

- Rust 1.97.1 (the repository toolchain file selects it automatically)
- Node.js 22 or newer for the host console
- A C/C++ build toolchain supported by Rust on the target platform

On Windows, install Visual Studio Build Tools with the Desktop development with C++ workload.

## Build and test the server

```bash
cargo build --workspace
cargo test --workspace
```

Run the development server:

```bash
cargo run -p charactergen-server
```

The server listens on `127.0.0.1:8787`. Override this with `CHARACTERGEN_BIND`.

## Build and run the host console

```bash
cd web
pnpm install --frozen-lockfile
pnpm build
```

For development:

```bash
pnpm dev
```

The console uses `http://127.0.0.1:8787/api` by default. Set `VITE_API_BASE` to use another server.

## Repository layout

```text
crates/charactergen-core       Canonical domain types and extension contracts
crates/charactergen-importers  Source-data staging adapters
crates/charactergen-providers  Character-engine and model-provider adapters
crates/charactergen-server     Authoritative HTTP server
web/                           Browser host console
```

## Automated builds

Pull requests and pushes to `main` run Rust formatting, linting, tests, and the web production build. Tags beginning with `v` build release server binaries for Linux, Windows, and macOS and attach them to a GitHub release.
