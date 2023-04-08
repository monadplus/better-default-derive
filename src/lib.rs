use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DataEnum, DataStruct, DeriveInput};

#[proc_macro_derive(Default, attributes(default))]
pub fn derive(input: TokenStream) -> TokenStream {
    let output = match __derive(parse_macro_input!(input as DeriveInput)) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    };
    proc_macro::TokenStream::from(output)
}

fn __derive(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: input_ident,
        mut generics,
        data,
    } = input;

    let (body, fields) = match data {
        syn::Data::Struct(data) => struct_case(&input_ident, data),
        syn::Data::Enum(data) => enum_case(&input_ident, data),
        syn::Data::Union(_) => Err(syn::Error::new_spanned(
            &input_ident,
            "#[derive(Default)] is only supported for unions",
        )),
    }?;

    add_trait_bounds(&mut generics, &fields);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics std::default::Default for #input_ident #ty_generics #where_clause {
            fn default() -> Self {
                #body
            }
        }
    };

    Ok(output)
}

fn struct_case(
    struct_ident: &syn::Ident,
    data: DataStruct,
) -> Result<(proc_macro2::TokenStream, syn::Fields), syn::Error> {
    let constr = match data.fields.clone() {
        syn::Fields::Unit => quote!(#struct_ident),
        syn::Fields::Unnamed(unnamed) => {
            let fields_constr = unnamed.unnamed.into_iter().map(|field| {
                let ty = field.ty;
                quote!(#ty::default())
            });
            quote!(#struct_ident(#(#fields_constr),*))
        }
        syn::Fields::Named(named) => {
            let fields_constr = named.named.into_iter().map(|field| {
                let field_name = field.ident.expect("named fields should contain an ident");
                let ty = &field.ty;
                quote!(#field_name : #ty::default())
            });

            quote! {
                #struct_ident{#(#fields_constr),*}
            }
        }
    };

    Ok((constr, data.fields))
}

fn enum_case(
    root_ident: &syn::Ident,
    data: DataEnum,
) -> Result<(proc_macro2::TokenStream, syn::Fields), syn::Error> {
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            root_ident,
            "#[derive(Default)] is not supported for empty enums",
        ));
    }

    let mut default_variants = data
        .variants
        .into_iter()
        .filter(|variant| has_default_attr(variant).unwrap_or_default());

    if let Some(default_variant) = default_variants.next() {
        if let Some(another_default_variant) = default_variants.next() {
            return Err(syn::Error::new_spanned(
                another_default_variant,
                "#[default] is defined more than once",
            ));
        }

        let variant_ident = default_variant.ident.clone();
        let default_variant_constr = match default_variant.fields.clone() {
            syn::Fields::Unit => {
                quote! {
                    Self::#variant_ident
                }
            }
            syn::Fields::Unnamed(unnamed) => {
                let fields_constr = unnamed.unnamed.into_iter().map(|field| {
                    let ty = field.ty;
                    quote!(#ty::default())
                });

                quote! {
                    Self::#variant_ident(#(#fields_constr),*)
                }
            }
            syn::Fields::Named(named) => {
                let fields_constr = named.named.into_iter().map(|field| {
                    let field_name = field.ident.expect("named fields should contain an ident");
                    let ty = &field.ty;
                    quote!(#field_name : #ty::default())
                });

                quote! {
                    Self::#variant_ident{#(#fields_constr),*}
                }
            }
        };

        Ok((default_variant_constr, default_variant.fields))
    } else {
        Err(syn::Error::new_spanned(
            root_ident,
            "expected one variant with #[default]",
        ))
    }
}

fn has_default_attr(variant: &syn::Variant) -> Option<bool> {
    let attr = variant.attrs.get(0)?;
    let is_default = attr.path().is_ident("default");
    Some(is_default)
}

fn add_trait_bounds(generics: &mut syn::Generics, fields: &syn::Fields) {
    let used_types: HashSet<syn::Ident> = fields
        .iter()
        .filter_map(|field| type_ident(&field.ty))
        .cloned()
        .collect();

    for type_param in generics.type_params_mut() {
        if used_types.contains(&type_param.ident) {
            type_param
                .bounds
                .push(parse_quote!(::std::default::Default));
        }
    }
}

fn type_ident(ty: &syn::Type) -> Option<&syn::Ident> {
    if let &syn::Type::Path(syn::TypePath {
        qself: None,
        ref path,
    }) = ty
    {
        if path.segments.len() == 1 {
            return Some(&path.segments.first()?.ident);
        }
    }
    None
}
