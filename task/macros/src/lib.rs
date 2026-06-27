//! Procedural macros backing the `workflow-task` crate, providing the
//! [`task!`] and [`set_task!`] macros for declaring async task closures.

use proc_macro::TokenStream;
use proc_macro_error3::proc_macro_error;
use quote::{ToTokens, quote};
use syn::parse_macro_input;
mod task;

/// Constructs a `workflow_task::Task` from a closure or expression, wrapping
/// the closure body so it is boxed and pinned as the task's async future.
#[proc_macro]
#[proc_macro_error]
pub fn task(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as task::Task);
    let ts = quote! {
        workflow_task::Task::new(#result)
    };
    ts.into()
}

/// Assigns a task closure to an existing task source by expanding to a
/// `set_task_fn` call on the given target with the supplied closure.
#[proc_macro]
#[proc_macro_error]
pub fn set_task(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as task::SetTask);
    let ts = result.to_token_stream();
    ts.into()
}
