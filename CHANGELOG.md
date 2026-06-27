# Changelog

## 0.19.0

A maintenance and modernization release: migration to Rust edition 2024, a full
security-advisory sweep (cargo-audit **17 → 0** vulnerabilities, **0** warnings),
updates of effectively the entire dependency tree to current versions, and
removal of the last unmaintained/discontinued dependencies.

### Toolchain

- Bumped workspace version `0.18.1` → `0.19.0`.
- Migrated to **edition 2024**; minimum supported Rust version is now **1.96**.
- All crates pass `cargo fmt` and `cargo clippy` (native and `wasm32-unknown-unknown`) clean.
- Every public API item is now documented (`missing_docs`-clean), except the
  vendored binding modules noted below.

### Security (RUSTSEC advisories resolved)

`cargo audit` now reports **0 vulnerabilities and 0 warnings** (was 17 vulnerabilities).

Transitive dependencies advanced to fixed versions:

- `bytes` → 1.12.0 — RUSTSEC-2026-0007
- `time` → 0.3.51 — RUSTSEC-2026-0009
- `tokio` → 1.42.1 — RUSTSEC-2025-0023
- `openssl` → 0.10.81 — RUSTSEC-2025-0004, RUSTSEC-2024-0357, RUSTSEC-2025-0022
- `ring` → 0.17.14 — RUSTSEC-2025-0009
- `quinn-proto` → 0.11.15 — RUSTSEC-2026-0185, RUSTSEC-2026-0037
- `rustls` → 0.23.41 — RUSTSEC-2024-0399
- `rustls-webpki` → 0.103.13 — RUSTSEC-2026-0104, -0098, -0099, -0049
- `idna` (via `url` 2.5.8 / `publicsuffix` / `cookie_store`) — RUSTSEC-2024-0421
- `memmap2` → 0.9.11 — RUSTSEC-2026-0186 (unsound)

Unmaintained / unsound first-party usages removed:

- `instant` → **`web-time`** (maintained drop-in) — RUSTSEC-2024-0384.
- `rand` 0.8 → **0.10** — RUSTSEC-2026-0097 (unsound `rand` 0.8.5).
- `atty` → **vendored `hexplay`** with `std::io::IsTerminal` — RUSTSEC-2024-0375,
  RUSTSEC-2021-0145.
- `adler` and `number_prefix` unmaintained advisories cleared by bumping
  `backtrace`/`miniz_oxide` and `cliclack`/`indicatif`.
- Removed the unused `hickory-dns` passthrough from `workflow-http`, which was the
  sole source of `hickory-proto` — RUSTSEC-2026-0118 (no fixed version),
  RUSTSEC-2026-0119.

### Dependencies

- **wasm-bindgen 0.2.100 → 0.2.126** (with `js-sys` 0.3.103, `web-sys` 0.3.103,
  `wasm-bindgen-futures` 0.4.76). Adapted to the new reference ABI
  (`Abi = WasmPtr<WasmRefCell<T>>`) in `workflow-wasm`.
- **getrandom 0.2 → 0.4** using the `wasm_js` backend; `wasm32-unknown-unknown`
  now requires `--cfg getrandom_backend="wasm_js"` (set in `.cargo/config.toml`).
- **reqwest 0.12 → 0.13** — TLS feature set overhauled; `workflow-http`
  passthroughs renamed (`rustls-tls` → `rustls`, `macos-system-configuration` →
  `system-proxy`, etc.).
- **tungstenite / tokio-tungstenite 0.23 → 0.29** — `WebSocketConfig` is now
  `#[non_exhaustive]` (builder API); `Message` uses `Bytes`/`Utf8Bytes`.
- **borsh 1.5 → 1.7**.
- **syn 1 → 2** across the proc-macro crates; **`proc-macro-error` →
  `proc-macro-error3`** (maintained fork).
- Terminal stack: **cliclack 0.5, crossterm 0.29, dirs 6, convert_case 0.11,
  itertools 0.15**.
- Removed the discontinued **`async-std`** entirely (native `store` file I/O moved
  to `tokio::fs`; other usages mapped to direct equivalents), wasm32-tested.
- General `cargo update` sweep to the latest semver-compatible versions across the
  tree.

### Structural changes

- **Internalized `node-sys`** (unmaintained) into `workflow-node` as the public
  module **`workflow_node::node_sys`**, dropping the external dependency. Fixes its
  incompatibilities with wasm-bindgen ≥ 0.2.126 (duplicate `WriteStream`/`ReadStream`
  JS class names; `#[wasm_bindgen] impl` on imported extern types). Retains a link
  to the original [`node-sys`](https://github.com/interfaces-rs/node-sys) source.
- **Vendored `hexplay`** into `workflow-log` (`log::hex`) to drop `atty`, linking
  the original [`hexplay`](https://crates.io/crates/hexplay) source.
- Added a **`tests/wasm32`** aggregator crate that links every wasm-bindgen
  workflow-rs crate, and a wasm-pack codegen step in `./check`, so wasm-bindgen
  CLI errors (which `cargo clippy` cannot surface) are caught workspace-wide.

### workflow-egui

- **egui / eframe 0.31 → 0.34.** Migrated `eframe::App::update` → the now-required
  `ui` method and `Context::screen_rect` → `content_rect`.
- Enabled eframe's `x11` and `wayland` features so `winit` builds on Linux when
  `default-features = false`.

### CI

- Install `cargo-nextest` with `--locked` (now required by upstream).
