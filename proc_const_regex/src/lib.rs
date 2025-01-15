use const_regex_regex_transformer::{to_regex, ChainedMatchable, InvertibleMatchable, Matchable};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, Lit};


#[proc_macro]
pub fn regex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input TokenStream into a Rust syntax element
    let input = parse_macro_input!(input as syn::Expr);

    // Match on the parsed input to find a string literal
    let extracted_string = if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Str(lit_str), .. }) = input {
        lit_str.value() // Extract the inner string
    } else {
        panic!("Expected a string literal as input.");
    };

    let r = to_regex(&extracted_string);

    let fnl = convert_regex(r);

    let x = quote! { #fnl };

    x.into()
}

fn convert_regex(regex: ChainedMatchable) -> Final {
    let ts = recurse_handle_chain(&regex);

    Final { function_contents: ts }
}

fn invertible_matchable(m: &InvertibleMatchable) -> TokenStream {
    todo!()
}

fn union_matchable(u: &[Matchable]) -> TokenStream {
    todo!()
}

fn matchable(m: Matchable) -> TokenStream {
    todo!()
}

fn recurse_handle_chain(chain: &ChainedMatchable) -> TokenStream {
    let (m, r, n) = (chain.matchables(), chain.repetition(), chain.next());

    let condition = invertible_matchable(m);

    let with_context = quote! {
        let (next, input) = input.get_advance();
        let pass = { #condition };
    };

    todo!()
}

struct Final {
    function_contents: TokenStream
}

impl ToTokens for Final {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let base: Ident = parse_str("_0").unwrap();

        let functions_ts = self.function_contents.to_token_stream();

        tokens.extend(quote! {
                {
                    struct Regex;
                    impl Regex {
                        #functions_ts
                        pub const fn test(&self, s: &str) -> bool {
                            Self::#base(const_regex_util::CharSlice::new(s))
                        }
                    }
                    Regex {}
                }
        });

    }
}