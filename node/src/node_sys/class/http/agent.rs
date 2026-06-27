use crate::node_sys::{
    class::{http, stream},
    interface::{AgentConstructorOptions, GetNameOptions},
};
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "http")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Agent;

    #[wasm_bindgen(constructor)]
    pub fn new(options: AgentConstructorOptions) -> Agent;

    //******************//
    // Instance Methods //
    //******************//

    // FIXME: options type
    #[wasm_bindgen(method, js_name = "createConnection")]
    pub fn create_connection(
        this: &Agent,
        options: &Object,
        callback: Option<&Function>,
    ) -> stream::Duplex;

    #[wasm_bindgen(method, js_name = "keepAliveSocket")]
    pub fn keep_alive_socket(this: &Agent, socket: &stream::Duplex);

    #[wasm_bindgen(method, js_name = "reuseSocket")]
    pub fn reuse_socket(this: &Agent, socket: &stream::Duplex, request: &http::ClientRequest);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Agent);

    #[wasm_bindgen(method, js_name = "getName")]
    pub fn get_name(this: &Agent, options: GetNameOptions);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "freeSockets")]
    pub fn free_sockets(this: &Agent);

    #[wasm_bindgen(method, getter, js_name = "maxFreeSockets")]
    pub fn max_free_sockets(this: &Agent);

    #[wasm_bindgen(method, getter, js_name = "maxSockets")]
    pub fn max_sockets(this: &Agent);

    #[wasm_bindgen(method, getter)]
    pub fn requests(this: &Agent);

    #[wasm_bindgen(method, getter)]
    pub fn sockets(this: &Agent);
}
