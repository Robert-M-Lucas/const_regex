use const_regex_regex_transformer::{to_regex};
use const_regex_regex_transformer::automata::{to_dfa, to_nfa};
use const_regex_regex_transformer::regex::{ChainedMatchable, InvertibleMatchable, Matchable};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
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
    let nfa = to_nfa(r);
    let dfa = to_dfa(nfa);

    let mut transitions = TokenStream::new();

    for (i, (s, ts)) in dfa.transitions.iter().enumerate() {
        let success_ident: Ident = parse_str(&format!("T{i}_SUCCESS")).unwrap();
        let ident: Ident = parse_str(&format!("T{i}")).unwrap();
        let s = *s;
        let t_len = ts.len();

        let mut ts_tokens = TokenStream::new();

        for (tt, dst) in ts {
            let dst = *dst;
            let tt = *tt;

            ts_tokens.append_all::<TokenStream>(quote! {
                (#tt, #dst),
            }.into());
        }

        transitions.append_all::<TokenStream>(quote! {
            const #success_ident: bool = #s;
            const #ident: [(const_regex_regex_transformer::automata::TransitionType, usize); #t_len] = [#ts_tokens];
        }.into());
    }

    let mut full_tokens = TokenStream::new();
    for i in 0..dfa.transitions.len() {
        let success_ident: Ident = parse_str(&format!("T{i}_SUCCESS")).unwrap();
        let ident: Ident = parse_str(&format!("T{i}")).unwrap();
        full_tokens.append_all::<TokenStream>(quote! {
            (#success_ident, &#ident),
        }.into());
    }

    let t_len = dfa.transitions.len();
    transitions.append_all::<TokenStream>(quote! {
        const TRANSITIONS: [(bool, &[(const_regex_regex_transformer::automata::TransitionType, usize)]); #t_len] = [#full_tokens];
    }.into());

    let x = quote! {
        {
            struct Regex;

            impl Regex {
                pub const fn test(&self, input: &str) -> bool {
                    #transitions

                    let mut s = 0;
                    'outer: loop {
                        if s >= TRANSITIONS.len() {
                            return true;
                        }
                        let (success_state, ts) = &TRANSITIONS[s];
                        if s == input.as_bytes().len() {
                            return *success_state;
                        }

                        let (c, d) = const_regex_util::next_char(input, s);
                        s += d;

                        let mut i = 0;
                        let len = ts.len();
                        while i < len {
                            let (t, ns) = &ts[i];

                            let r = match t {
                                const_regex_regex_transformer::automata::TransitionType::Single(a) => *a == c,
                                const_regex_regex_transformer::automata::TransitionType::Range(a, b) => *a <= c && c <= *b,
                                const_regex_regex_transformer::automata::TransitionType::ExcludeRange(a, b) => c < *a || *b > c,
                                const_regex_regex_transformer::automata::TransitionType::Any => true
                            };

                            if r {
                                s = *ns;
                                continue 'outer;
                            }

                            i += 1;
                        }

                        return false
                    }
                }
            }
            Regex {}
        }
};

x.into()
}

// fn convert_regex(regex: ChainedMatchable) -> Final {
//     let ts = recurse_handle_chain(&regex);
//
//     Final { function_contents: ts }
// }
//
// fn invertible_matchable(m: &InvertibleMatchable) -> TokenStream {
//     todo!()
// }
//
// fn union_matchable(u: &[Matchable]) -> TokenStream {
//     todo!()
// }
//
// fn matchable(m: Matchable) -> TokenStream {
//     todo!()
// }
//
// fn recurse_handle_chain(chain: &ChainedMatchable) -> TokenStream {
//     let (m, r, n) = (chain.matchables(), chain.repetition(), chain.next());
//
//     let condition = invertible_matchable(m);
//
//     let with_context = quote! {
//         let (next, input) = input.get_advance();
//         let pass = { #condition };
//     };
//
//     todo!()
// }
//
// struct Final {
//     function_contents: TokenStream
// }
//
// impl ToTokens for Final {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let base: Ident = parse_str("_0").unwrap();
//
//         let functions_ts = self.function_contents.to_token_stream();
//
//         tokens.extend(quote! {
//                 {
//                     struct Regex;
//                     impl Regex {
//                         #functions_ts
//                         pub const fn test(&self, s: &str) -> bool {
//                             Self::#base(const_regex_util::CharSlice::new(s))
//                         }
//                     }
//                     Regex {}
//                 }
//         });
//
//     }
// }