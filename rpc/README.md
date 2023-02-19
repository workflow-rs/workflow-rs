## `workflow-rpc` `(wRPC)`

Part of the [`workflow-rs`](https://github.com/workflow-rs) application framework.

***
[<img alt="github" src="https://img.shields.io/badge/github-workflow--rs-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/workflow-rs/workflow-rpc)
[<img alt="crates.io" src="https://img.shields.io/crates/v/workflow-rpc.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/workflow-rpc)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-workflow--rpc-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/workflow-rpc)
<img alt="license" src="https://img.shields.io/crates/l/workflow-rpc.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">
<img src="https://img.shields.io/badge/platform: client-native-informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: client-wasm32/browser -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: client-wasm32/node.js -informational?style=for-the-badge&color=50a0f0" height="20">
<img src="https://img.shields.io/badge/platform: server-native-informational?style=for-the-badge&color=50a0f0" height="20">

RPC functionality built on top of the [`workflow-websocket`](https://crates.io/crates/workflow-websocket) crate offering asynchronous data relay over WebSocket connections supporting a custom high-performance `Borsh` and an extended `JSON-RPC` protocols.


## Features

- High-performance Borsh message encoding protocol
- RPC method and notification handler declarations based on serializable generics
- Client to Server RPC method invocation
- Client to Server notification messages
- Server to Client notification messages
- Server-side handshake scaffolding for custom connection negotiation
- Easy to retain connection data structure for posting async client notifications

This crate provides a high performance, Rust-focused, communication layer. The remote function invocation is done via a single function with two generics `rpc.call<Request,Response>().await?` where the request and response data types must implement serlialization using both Borsh and Serde JSON serialization and deserialization traits.

The data is transmitted via WebSocket binary message frames for Borsh encoding and via text frames for JSON encoding.

## Borsh Protocol

Due to use of generics for `id` and `op` *(method)* types, Borsh header messages can vary in size and it is the responsibility of the developer
to ensure consistency between client and server. Protocol versioning can be negotiated using a handshake during the connection opening phase.

The following format is used by the Borsh Protocol:
Request: [Option<Id>,Option<Ops>,Payload]
- `id: Id`: a generic user-defined type, typically u32 or u64-based type (`Option:None` if the message is a notification).
- `op: Ops`: a generic user-defined type, typically an `enum` representing the operation.
- `payload: Payload`: Any data type that implements BorshSerialize, BorshDeserialize, Serialize and Deserialize traits.
Response: [Option<Id>,Kind,Option<Ops>,Payload]
- `id: Id`: a generic user-defined type (`Option:None` if the message is a notification)
- `kind: Kind`: a byte representing message type: `0x01`: Success, `0x02`: Error, `0xff`: Notification
- `ops: Ops`: a generic user-defined type, typically an `enum` representing the operation.
- `payload: Payload`: serialized data containing `Result<UserType,ServerError>`

NOTE: Borsh provides high-performance serialization and deserialization, however, the change in the data structure
formats transported across RPC can result in the protocol mismatch between server and client. When using Borsh
it is your responsibility to build both server and client from the same codebase.

To provide version resolution, data structures can be encapsulated into enums as follows:
```rust
enum MyEnum {
    V1(DataStructureV1),
    V2(DataStructureV2),
    ...
}
```

## wRPC JSON Protocol

JSON protocol uses Serde ([serde-json](https://crates.io/crates/serde_json)) serialization.

JSON message format extends on top of JSON-RPC protocol as follows:
Client-side message:
- `id`: optional, absent if the message is a notification
- `method`: RPC method or notification name
- `params`: message data 
Server-side message:
- `id`: optional, absent if message is a notification
- `method`: RPC method or notification name
- `params`: message (response or notification) data
- `error`: error data if the RPC method produced an error
`error` data field contains:
- `code`: error code (as of v0.3.0 always `0x00`)
- `message`: error message string
- `data`: additional error data (as of v0.3.0 always absent`)
Differences between JSON-RPC and JSON-wRPC:
- JSON-RPC response returns `result` property in the response. wRPC returns `params` property in case of both response and notification.
- JSON-RPC 2.0 specification does not support server-side (server-to-client) notification.
- JSON-RPC 2.0 contains a `json-rpc="2.0"` property in every message. This is redundant for wRPC - wRPC handshake can be used to describe protocol version.

## Node.js compatibility

NOTE: `workflow-rpc` is built on top of the [`workflow-websocket`](https://crates.io/crates/workflow-websocket) crate. To use `workflow-rpc` in the Node.js environment, you need to introduce a W3C WebSocket object before loading the WASM32 library.
You can use any Node.js module that exposes a W3C-compatible WebSocket implementation. Two of such modules are [WebSocket](https://www.npmjs.com/package/websocket) (provides a custom implementation) and [isomorphic-ws](https://www.npmjs.com/package/isomorphic-ws) (built on top of the [`ws`](https://www.npmjs.com/package/ws) WebSocket module).

You can use the following shims:
```
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
// isomorphic-ws
globalThis.WebSocket = require('isomorphic-ws');
```
