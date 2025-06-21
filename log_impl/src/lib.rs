extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

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

    // Build list of print statements to log each parameter
    let mut ps: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut i = 0;
    while i < params.len() {
        let p: FnArg = params[i].clone();
        // ps.push(quote! { println!("{}", quote!(#p) ); });
        // Note: proc_macro says not to do this string manipulation.
        //       It advises to match with TokenTree? But TokenTree::Ident fields are private?? How can I get the identifier? Ask if you're going to be useful.
        //       This manipulation relies on whitespace and format of p, which is not ideal and not robust.

        // has the form "ident : type", like "a : i32".
        let s1: String = quote!(#p).to_string();
        let mut p_name: String = String::from("");
        let mut p_name_len = 0;
        // Find last occurence of ':' in "ident : type".
        for (j, c) in s1.chars().enumerate() {
            if c == ':' {
                p_name_len = j;
            }
        }
        // Push characters until last ':' character.
        for (j, c) in s1.chars().enumerate() {
            // Cut off space between identifier end and ':'.
            if j == p_name_len - 1 {
                break;
            }
            p_name.push(c);
        }
        // Concatentate together a print statement using the parameter name p_name.
        let print: String = String::from("println!(\"{:?}\", ");
        let semicolon: String = String::from(");");
        let print_arg = format!("{print}{p_name}{semicolon}");
        let tree = syn::parse_file(&print_arg).unwrap();
        ps.push(quote! { #tree });
        i = i + 1;
    }

    quote!(
        #(#attrs)*

        #vis #sig {
            println!("Enter function:: {}", quote!(#signature));
            // log parameters
            #(#ps)*

            let __f = || { #(#statements)* };
            // let __result = {
            //     #(#statements)*
            // };
            let __result = __f();

            // log result
            // Need a way to get the return expression in a string. Fixed -- use debug specifier :? or :#?.
            //   -- it was complaining that main's void return () couldn't be formatted with default specifier.
            println!("Returns: {:?}", __result);
            println!("Exit function:: {}", quote!(#signature));
            // println!("Returns: {:?}", #(#statements)*);
            return __result;
        }
    )
    .into()
}
