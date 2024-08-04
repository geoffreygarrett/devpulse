use deluxe::ExtractAttributes;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenStream};
use quote::quote;
use syn::{GenericArgument, PathArguments, Type};

#[derive(ExtractAttributes)]
#[deluxe(attributes(wrapper))]
#[derive(Default)]
pub struct Wrapper {
    pub name: Option<String>,
}

#[derive(ExtractAttributes, deluxe::ParseMetaItem, Default)]
#[deluxe(attributes(wrap))]
pub struct WrappedField(bool);

#[derive(ExtractAttributes)]
#[deluxe(attributes(columns))]
pub struct WrappedFields(#[deluxe(flatten)] pub Vec<WrappedField>);

pub fn derive_wrapper(item: TokenStream2) -> TokenStream2 {
    let item_clone = item.clone();
    let item_return = item.clone();
    let mut input = syn::parse::<syn::DeriveInput>(item_clone.into()).unwrap();
    let struct_name = input.ident.clone();
    let errors = deluxe::Errors::new();
    let Wrapper { name } = deluxe::extract_attributes_optional(&mut input, &errors);
    let mut wrapped_name;
    match name {
        Some(n) => {
            wrapped_name = Ident::new(&n, struct_name.span());
        }
        None => {
            wrapped_name = Ident::new(&format!("{}Wrapper", struct_name), struct_name.span());
        }
    }

    let mut fields = Vec::new();
    let mut field_wrap = Vec::new();
    let mut field_unwrap = Vec::new();
    if let syn::Data::Struct(s) = &mut input.data {
        for field in s.fields.iter_mut() {
            let field_attrs: WrappedField = deluxe::extract_attributes_optional(field, &errors);
            let ident = field.ident.as_ref().unwrap();
            let vis = &field.vis;
            let ty = &field.ty;
            let wrapped = field_attrs.0;
            let (field_ty, wrap_expr, unwrap_expr) =
                if let Some(inner_ty) = is_option_type(&field.ty) {
                    if wrapped {
                        // Wrap the inner type of Option
                        (
                            quote! { Option<GenericWrapper<#inner_ty>> },
                            quote! { #ident: item.#ident.map(GenericWrapper::new) },
                            quote! { #ident: self.#ident.map(|wrapper| wrapper.unwrap()) },
                        )
                    } else {
                        // No wrapping required, just clone the Option type
                        (
                            quote! { #ty },
                            quote! { #ident: item.#ident.clone() },
                            quote! { #ident: self.#ident.clone() },
                        )
                    }
                } else {
                    if wrapped {
                        // Wrap the entire type
                        (
                            quote! { GenericWrapper<#ty> },
                            quote! { #ident: GenericWrapper::new(item.#ident.clone()) },
                            quote! { #ident: self.#ident.unwrap() },
                        )
                    } else {
                        // No wrapping, just clone the field
                        (
                            quote! { #ty },
                            quote! { #ident: item.#ident.clone() },
                            quote! { #ident: self.#ident.clone() },
                        )
                    }
                };
            fields.push(quote! { #vis #ident: #field_ty });
            field_wrap.push(wrap_expr);
            field_unwrap.push(unwrap_expr);

        }
    } else {
        panic!("SqlInsert can only be used with structs with named fields.");
    }

    let expanded = quote! {

        #[derive(sqlx::FromRow)]
        pub struct #wrapped_name {
            #(
                #fields
            ),*
        }

        impl #wrapped_name {
            pub fn wrap(item: #struct_name) -> Self {
                #wrapped_name {
                    #(
                        #field_wrap
                    ),*
                }
            }
        }

        impl #wrapped_name {
            pub fn unwrap(&self) -> #struct_name {
                #struct_name {
                    #(
                        #field_unwrap
                    ),*
                }
            }
        };
    };
    TokenStream::from(expanded)
}

// Helper function to create the field self reference
fn field_self_expr(name: &str) -> TokenStream {
    let ident = Ident::new(name, Span::call_site());
    quote! { #ident }
}

fn is_option_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                        return Some(ty);
                    }
                }
            }
        }
    }
    None
}
