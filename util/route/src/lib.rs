extern crate proc_macro;
use proc_macro::TokenStream;

use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span};
use syn::{Expr, Lit, Meta};

#[derive(Debug, FromMeta)]
struct Intercepted {
    path: Option<String>,

    #[darling(flatten)]
    method: Option<String>,
}

#[proc_macro_attribute]
pub fn route(mut attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> TokenStream {
    let mut item_fn = match syn::parse::<syn::ItemFn>(item) {
        Ok(module) => module,
        Err(e) => return e.into_compile_error().into(),
    };

    let attr_args = match NestedMeta::parse_meta_list(attr.clone().into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let mut method = None;
    let mut path = None;
    // Iterate through the attributes and process them
    for arg in attr_args {
        match arg {
            NestedMeta::Meta(Meta::Path(path_meta)) => {
                if let Some(segment) = path_meta.segments.last() {
                    let method_str = segment.ident.to_string().to_lowercase();
                    match method_str.as_str() {
                        "get" | "put" | "post" | "delete" | "patch" | "options" | "head"
                        | "trace" => {
                            method = Some(method_str.to_uppercase());
                        }
                        _ => {}
                    }
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident("path") {
                    if let Expr::Lit(expr_lit) = &nv.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            path = Some(convert_openapi_to_axum_path(&lit_str.value()));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let attr_tokens = proc_macro2::TokenStream::from(attr);
    let method_tokens = Ident::new(&method.unwrap_or_else(|| "GET".to_string()), Span::call_site());
    let path_tokens = path.unwrap_or_else(|| "/".to_string());
    quote::quote!(
        #[utoipa::path(#attr_tokens)]
        #[axum_typed_routing::route(#method_tokens #path_tokens)]
        #item_fn
    )
    .into()
}

fn convert_openapi_to_axum_path(openapi_path: &str) -> String {
    let mut axum_path = String::from(openapi_path);
    while let Some(start) = axum_path.find('{') {
        if let Some(end) = axum_path.find('}') {
            let param = &axum_path[start + 1..end];
            axum_path.replace_range(start..=end, &format!(":{}", param));
        } else {
            break;
        }
    }
    axum_path
}
