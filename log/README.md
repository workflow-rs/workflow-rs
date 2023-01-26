## WORKFLOW-LOG

Part of the [WORKFLOW-RS](https://github.com/workflow-rs) application framework.

***

Application logging functionality

[![Crates.io](https://img.shields.io/crates/l/workflow-log.svg?maxAge=2592000)](https://crates.io/crates/workflow-log)
[![Crates.io](https://img.shields.io/crates/v/workflow-log.svg?maxAge=2592000)](https://crates.io/crates/workflow-log)
![platform](https://img.shields.io/badge/platform-Native-informational)
![platform](https://img.shields.io/badge/platform-Web%20%28wasm32%29-informational)
![platform](https://img.shields.io/badge/platform-BPF-informational)

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

