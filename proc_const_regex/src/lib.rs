use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, Lit};

mod regex;

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

    let mut fs = extracted_string.chars().fold((0, Vec::new()), |(i, mut v), ac| {
        v.push(GenFunc {
            id: i,
            expecting: Some((ac, i + 1)),
        });
        (i + 1, v)
    }).1;

    fs.push(GenFunc {
        id: fs.len(),
        expecting: None
    });

    let fnl = Final {
        functions: Functions {
            functions: fs
        }
    };

    let x = quote! { #fnl };

    x.into()
}

struct Final {
    functions: Functions
}

impl ToTokens for Final {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let base: Ident = parse_str("_0").unwrap();

        let functions_ts = self.functions.to_token_stream();

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

struct Functions {
    functions: Vec<GenFunc>
}

impl ToTokens for Functions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for f in &self.functions {
            f.to_tokens(tokens);
        }
    }
}

struct GenFunc {
    id: usize,
    expecting: Option<(char, usize)>,
}

impl ToTokens for GenFunc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let function_name = format!("_{}", self.id);
        let function_ident: Ident = parse_str(&function_name).unwrap();

        tokens.extend(quote! {
            const fn #function_ident(mut remaining: const_regex_util::CharSlice) -> bool
        });

        if let Some((c, next)) = self.expecting {
            let next_name = format!("_{}", next);
            let next_ident: Ident = parse_str(&next_name).unwrap();
            tokens.extend(quote! {
                {
                    if remaining.is_empty() { return false; }
                    let ac = remaining.first();
                    let bc = const_regex_util::extend_char(#c);
                    if ac[0] != bc[0] || ac[1] != bc[1] || ac[2] != bc[2] || ac[3] != bc[3] { false } else { Self::#next_ident(remaining.new_advance()) }
                }
            })
        }
        else {
            tokens.extend(quote! { { remaining.is_empty() } });
        }
    }
}