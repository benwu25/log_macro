extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

// Implementation of the #[log_macro] function
pub fn log_macro_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse function's TokenStream
    let input = parse_macro_input!(input as ItemFn);

    // Destructure function node
    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let statements = block.stmts.clone();
    let signature = sig.clone();
    let params = sig.inputs.clone();

    // Currently: this code extracts each parameter name into a string and prints in debug mode (no type narrowing, no formatting)

    // Next: for each parameter, deduce its type and handle different cases (e.g., structs, nested structs, and their fields)
    // What we have here is parameter names and types as strings: e.g., y: User.
    //  -- for struct types like User, how do we get its field names and types, e.g., id: i32 for User { id: i32 }
    //  -- ideas:
    //     - add a macro to each struct to implement a function which returns all the structs fields and types.
    //       we can call this function on the struct instance to get the information we need
    //     - use built-in functionality from Rust to call a macro or function or typeof operator to get the structs fields and types.

    // Build list of print statements to log each parameter
    let mut ps: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut i = 0;
    while i < params.len() {
        let p: FnArg = params[i].clone();

        // quote!(#p) has the form "parameter_name : parameter_type"
        let s1: String = quote!(#p).to_string();
        let mut p_name: String = String::from("");
        let mut p_name_len = 0;
        // Find last occurence of ':' in s1
        for (j, c) in s1.chars().enumerate() {
            if c == ':' {
                p_name_len = j;
            }
        }
        // Push identifier characters
        for (j, c) in s1.chars().enumerate() {
            if j == p_name_len - 1 {
                break;
            }
            p_name.push(c);
        }
        // Construct a print statement to print the parameter, push to list of print statements ps
        let print: String = String::from("println!(\"{:?}\", ");
        let semicolon: String = String::from(");");
        let print_arg = format!("{print}{p_name}{semicolon}");
        let tree = syn::parse_file(&print_arg).unwrap();
        ps.push(quote! { #tree });
        i = i + 1;
    }

    // Return the new modified function
    quote!(
        // Mutable global variables for this function?
        // static nonce counter?
        // static bool for writing program point declaration on first enter?

        // Add all original function attributes
        #(#attrs)*

        // Keep the same visibility and function signature
        #vis #sig {
            println!("Enter function:: {}", quote!(#signature));

            // log parameters
            #(#ps)*

            // May need to capture by mutable reference (this is immutable reference)
            let __f = || { #(#statements)* };

            // Call function in a lambda
            let __result = __f();

            // log result: no need to if exception occurred in lambda?
            println!("Returns: {:?}", __result);
            println!("Exit function:: {}", quote!(#signature));

            return __result;
        }
    )
    .into()
}
