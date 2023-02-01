# Example for `workflow-websocket`

This example contains 4 crates:

- `client-common`: `lib` crate that operates uniformly in the native, browser and Node.js environment.
- `client-browser`: `wasm32 lib` crate that is built with `wasm-pack` and operates in the JavaScript / TypeScript environment (browser or Node.js).
- `client-native`: native binary that operates in the native OS environment.
- `server`: server example (server implementation supports only native builds)
