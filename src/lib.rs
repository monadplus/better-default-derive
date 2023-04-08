use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput,
    Error, Fields, Generics, Ident, Type, TypePath, Variant,
};

const DEFAULT_VARIANT_KEYWORD: &str = "default";

#[proc_macro_derive(Default, attributes(default))]
pub fn derive(input: TokenStream) -> TokenStream {
    let output = match __derive(parse_macro_input!(input as DeriveInput)) {
        Ok(output) => output,
        Err(err) => err.into_compile_error(),
    };
    proc_macro::TokenStream::from(output)
}

fn __derive(input: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: input_ident,
        mut generics,
        data,
    } = input;

    let (body, fields) = match data {
        Data::Struct(data) => struct_case(&input_ident, data),
        Data::Enum(data) => enum_case(&input_ident, data),
        Data::Union(_) => Err(Error::new_spanned(
            &input_ident,
            "#[derive(Default)] is not supported for unions",
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
    struct_ident: &Ident,
    data: DataStruct,
) -> Result<(proc_macro2::TokenStream, Fields), Error> {
    let data_constr = default_instance_constr(struct_ident, &data.fields);

    Ok((data_constr, data.fields))
}

fn enum_case(
    root_ident: &Ident,
    data: DataEnum,
) -> Result<(proc_macro2::TokenStream, Fields), Error> {
    if data.variants.is_empty() {
        return Err(Error::new_spanned(
            root_ident,
            "#[derive(Default)] is not supported for empty enums",
        ));
    }

    let mut default_variants = data.variants.into_iter().filter(has_default_attr);

    match (default_variants.next(), default_variants.next()) {
        (Some(default_variant), None) => {
            let default_variant_constr = {
                // Something as below would be great, but `Self::XXX` is not a valid identifier
                // let variant_ident = Ident::new(&format!("Self::{}", &default_variant.ident), Span::call_site());
                let constr =
                    default_instance_constr(&default_variant.ident, &default_variant.fields);
                quote!(Self::#constr)
            };

            Ok((default_variant_constr, default_variant.fields))
        }

        (Some(default_variant), Some(another_default_variant)) => {
            let msg = "#[default] is defined multiple times";
            if cfg!(nightly) {
                let span = another_default_variant
                    .span()
                    .join(default_variant.span())
                    .expect("self and other are not from the same file");
                Err(Error::new(span, msg))
            } else {
                Err(Error::new_spanned(another_default_variant, msg))
            }
        }
        (None, _) => Err(Error::new_spanned(
            root_ident,
            "expected one variant with #[default]",
        )),
    }
}

fn default_instance_constr(data_constr_ident: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => quote!(#data_constr_ident),
        Fields::Unnamed(unnamed) => {
            let fields_constr = unnamed.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote!(#ty::default())
            });
            quote!(#data_constr_ident(#(#fields_constr),*))
        }
        Fields::Named(named) => {
            let fields_constr = named.named.iter().map(|field| {
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("named fields should contain an ident");
                let ty = &field.ty;
                quote!(#field_name : #ty::default())
            });
            quote!(#data_constr_ident{#(#fields_constr),*})
        }
    }
}

fn has_default_attr(variant: &Variant) -> bool {
    variant
        .attrs
        .get(0)
        .map(|attr| attr.path().is_ident(DEFAULT_VARIANT_KEYWORD))
        .unwrap_or_default()
}

fn add_trait_bounds(generics: &mut Generics, fields: &Fields) {
    let used_types: HashSet<Ident> = fields
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

fn type_ident(ty: &Type) -> Option<&Ident> {
    if let &Type::Path(TypePath {
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
