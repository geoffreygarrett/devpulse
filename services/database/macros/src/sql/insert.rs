use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenStream};
use quote::quote;

use crate::sql::models::{Column, SqlInsert};

pub fn derive_sql_insert(item: TokenStream2) -> TokenStream2 {
    let mut input = syn::parse::<syn::DeriveInput>(item.into()).unwrap();

    let errors = deluxe::Errors::new();
    let SqlInsert {
        schema,
        name: table_name,
    } = deluxe::extract_attributes_optional(&mut input, &errors);

    let name = input.ident;

    let schema = schema.unwrap_or_else(|| "public".to_string());
    let table_name = table_name.unwrap_or_else(|| name.to_string().to_lowercase());

    let mut fields = Vec::new();
    if let syn::Data::Struct(s) = &mut input.data {
        for field in s.fields.iter_mut() {
            // Use `syn::Attribute` instead of `Column`
            let column_attrs: Column = deluxe::extract_attributes_optional(field, &errors);
            let ident = field.ident.as_ref().unwrap();
            let column_name = column_attrs.rename.unwrap_or_else(|| ident.to_string());
            fields.push((ident.clone(), column_name));
        }
    } else {
        panic!("SqlInsert can only be used with structs with named fields.");
    }

    let field_names: Vec<_> = fields.iter().map(|(_, name)| name.clone()).collect();
    let field_placeholders: Vec<_> = (1..=fields.len()).map(|i| format!("${}", i)).collect();
    let field_idents: Vec<_> = fields
        .iter()
        .map(|(name, _)| field_self_expr(&name.to_string()))
        .collect();
    //
    //
    // // 3) Insert a new ticket
    // let row: (i64,) = sqlx::query_as("insert into ticket (name) values ($1) returning id")
    //     .bind("a new ticket")
    //     .fetch_one(&pool)
    //     .await?;

    // let expanded = quote! {
    //     impl #name {
    //         pub fn sql_insert(&self) -> String {
    //             let mut query = String::new();
    //             let mut fields = Vec::new();
    //             let mut values = Vec::new();
    //             let mut params = Vec::new();
    //
    //             #(
    //                 if !self.#field_idents.is_empty() {
    //                     fields.push(#field_names);
    //                     values.push(#field_placeholders);
    //                     params.push(&self.#field_idents as &dyn ::sqlx::Type<dyn ::sqlx::postgres::PgExecutor>);
    //                 }
    //             )*
    //
    //             format!("INSERT INTO \"{}\".\"{}\" ({}) VALUES ({})", #schema, #table_name, fields.join(", "), values.join(", "))
    //         }
    //     }
    // };
    let expanded = quote! {
        impl #name {
            pub async fn sql_insert(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<i64, sqlx::Error> {
                let mut query = sqlx::query_as(
                    concat!(
                        "INSERT INTO ", #schema, ".", #table_name, " (",
                        #(#field_names),*,
                        ") VALUES (",
                        #(", $" to_string() (1usize..=#field_names.len()).collect::<Vec<_>>()),*,
                        ") RETURNING id"
                    )
                );

                // Dynamically add parameters based on field presence
                #(
                    if let Some(value) = &self.#field_idents {
                        query = query.bind(value);
                    }
                )*

                let result = query.fetch_one(pool).await?;
                Ok(result.id)
            }
        }
    };
    TokenStream2::from(expanded)
}

// Helper function to create the field self reference
fn field_self_expr(name: &str) -> TokenStream {
    let ident = Ident::new(name, Span::call_site());
    quote! { #ident }
}
