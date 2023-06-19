use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Ident, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{spanned::Spanned, Error};

#[proc_macro_attribute]
pub fn end_loop_in_test_or_bench(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let input_span = input.span();
    eprintln!("Input:\n{input}\n\n");

    let tokens: Vec<_> = input.into_iter().collect();

    match tokens.first() {
        Some(TokenTree::Ident(ident))
            if {
                let span = ident.span();
                ident == &Ident::new("loop", span)
            } =>
        {
            let body: TokenStream2 = match tokens.get(1) {
                Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                    group.stream()
                }
                _ => {
                    return Error::new(input_span, "No group after `loop` keyword")
                        .to_compile_error()
                        .into()
                }
            };

            let output = quote! {
                #[cfg(any(test, feature = "bench"))]
                let mut end_loop_in_test_or_bench_counter = 0u8;

                loop {
                    #body

                    #[cfg(any(test, feature = "bench"))]
                    {
                        end_loop_in_test_or_bench_counter += 1;
                        if end_loop_in_test_or_bench_counter > 100 {
                            break;
                        }
                    }
                }
            };

            eprintln!("Output:\n{output}\n\n");
            dbg!(&output);
            output.into()
        }
        _ => Error::new(
            input_span,
            "#[end_loop_in_test_and_bench] can only be used on `loop {}` constructs",
        )
        .to_compile_error()
        .into(),
    }
}
