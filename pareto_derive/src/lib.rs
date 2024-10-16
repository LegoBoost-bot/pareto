use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(Dominate, attributes(pareto))]
pub fn dominate_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let comp = generate_comparison(&input.data).unwrap_or_else(syn::Error::into_compile_error);

    let expanded = quote! {
        impl #impl_generics ::pareto::Dominate for #name #ty_generics #where_clause {
            fn dominates(&self, other: &Self) -> bool {
                #comp
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_comparison(data: &Data) -> syn::Result<TokenStream> {
    match *data {
        Data::Struct(ref data) => {
            Ok(match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to:
                    //
                    //     `true && self.x <= other.x && self.y <= other.y`

                    let comparison = fields.named.iter().filter_map(|f| {
                        let mut invert = false;
                        let mut ignore = false;
                        let mut comp = quote!(<=);
                        for attr in f.attrs.iter().filter(|a| a.path().is_ident("pareto")) {
                            match attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("maximize") {
                                    comp = quote!(>=);
                                    return Ok(());
                                }

                                if meta.path.is_ident("invert") {
                                    invert = !invert;
                                    return Ok(());
                                }

                                if meta.path.is_ident("ignore") {
                                    ignore = true;
                                    return Ok(());
                                }

                                Err(meta.error("Unrecognized attribute. Possible values: `maximize`, `invert`, `ignore`"))
                            }) {
                                Ok(_) => {}
                                Err(e) => return Some(Err(e)),
                            }
                            if ignore {
                                return None;
                            }
                        }
                        let name = &f.ident;
                        Some(Ok(if !invert {
                            quote_spanned! {f.span()=>
                                self.#name #comp other.#name
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                !(self.#name #comp other.#name)
                            }
                        }))
                    }).collect::<Result<Vec<_>, _>>()?;

                    quote!(true #(&& #comparison)*)
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to:
                    //
                    //     `true && self.0 <= other.0 && self.1 <= other.1`

                    let comparison = fields.unnamed.iter().enumerate().filter_map(|(i, f)| {
                        let idx = Index::from(i);
                        let mut invert = false;
                        let mut ignore = false;
                        let mut comp = quote!(<=);
                        for attr in f.attrs.iter().filter(|a| a.path().is_ident("pareto")) {
                            match attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("maximize") {
                                    comp = quote!(>=);
                                    return Ok(());
                                }

                                if meta.path.is_ident("invert") {
                                    invert = !invert;
                                    return Ok(());
                                }

                                if meta.path.is_ident("ignore") {
                                    ignore = true;
                                    return Ok(());
                                }

                                Err(meta.error("Unrecognized attribute. Possible values: `maximize`, `invert`, `ignore`"))
                            }) {
                                Ok(_) => {},
                                Err(e) => return Some(Err(e))
                            };
                            if ignore { return None; }
                        }
                        Some(Ok(if !invert {
                            quote_spanned! {f.span()=>
                                self.#idx #comp other.#idx
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                !(self.#idx #comp other.#idx)
                            }
                        }))
                    }).collect::<Result<Vec<_>, _>>()?;

                    quote!(true #(&& #comparison)*)
                }
                Fields::Unit => quote!(true),
            })
        }
        _ => unimplemented!(),
    }
}
