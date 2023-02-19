## `workflow-log`

Part of the [`workflow-rs`](https://github.com/workflow-rs) application framework.

***

Application logging functionality


[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-log.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-log)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--log-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-log)
<img alt="license" src="https://img.shields.io/crates/l/workflow-log.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform- native -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform- solana_os -informational?style=for-the-badge&color=50a0f0" height="20">


## Features

* Log output functions that functions uniformly on supported platforms.
  * **Native** uses `stdout`
  * **WASM** (browser) uses `console.log()` and similar functions.
  * **Solana OS (BPF)** uses `solana_program::log::sol_log()` (`same as msg!() macro`)
* Attach to the standard [log](https://crates.io/crates/log) crate.
* Register a custom log sink to consume all application output externally.
* Re-export and a custom bypass for [console](https://crates.io/crates/console) crate, allowing to use ANSI terminal features while discarding them when running under BPF.

This crate offers the following macros:
* `log_trace!()`
* `log_debug!()`
* `log_info!()`
* `log_warning!()`
* `log_error!()`

