//! Procedural macros for clau.rs

use proc_macro::TokenStream;

/// Derive macro for creating MCP tools
/// 
/// # Example
/// ```ignore
/// #[derive(Tool)]
/// #[tool(name = "calculator", description = "Performs calculations")]
/// struct Calculator;
/// ```
#[proc_macro_derive(Tool, attributes(tool))]
pub fn derive_tool(_input: TokenStream) -> TokenStream {
    // TODO: Implement Tool derive macro
    TokenStream::new()
}