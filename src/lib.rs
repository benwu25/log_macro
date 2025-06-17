// extern crate proc_macro;
use log_impl::log_macro_impl;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn log_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    log_macro_impl(args, input)
}
