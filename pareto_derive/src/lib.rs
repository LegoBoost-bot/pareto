use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(Dominate, attributes(pareto_invert, pareto_ignore))]
pub fn dominate_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let comp = generate_comparison(&input.data);

    let expanded = quote! {
        impl #impl_generics ::pareto::Dominate for #name #ty_generics #where_clause {
            fn dominates(&self, other: &Self) -> bool {
                #comp
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_comparison(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to:
                    //
                    //     `true && self.x <= other.x && self.y <= other.y`

                    let comparison = fields.named.iter().filter_map(|f| {
                        let mut invert = false;
                        for attr in &f.attrs {
                            if attr.path().is_ident("pareto_invert") {
                                invert = !invert;
                            } else if attr.path().is_ident("pareto_ignore") {
                                return None;
                            }
                        }
                        let name = &f.ident;
                        Some(if invert {
                            quote_spanned! {f.span()=>
                                self.#name >= other.#name
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                self.#name <= other.#name
                            }
                        })
                    });

                    quote!(true #(&& #comparison)*)
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to:
                    //
                    //     `true && self.0 <= other.0 && self.1 <= other.1`

                    let comparison = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let idx = Index::from(i);
                        quote_spanned! {f.span()=>
                            self.#idx <= other.#idx
                        }
                    });

                    quote!(true #(&& #comparison)*)
                }
                Fields::Unit => quote!(true),
            }
        }
        _ => unimplemented!(),
    }
}
