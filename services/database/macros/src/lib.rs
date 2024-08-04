use proc_macro::TokenStream;

mod sql;
mod wrapper;

#[proc_macro_derive(IntoSqlInsert, attributes(sql, column))]
pub fn derive_sql_insert(item: TokenStream) -> TokenStream {
    sql::insert::derive_sql_insert(item.into()).into()
}

#[proc_macro_derive(Wrapper, attributes(wrapper, wrap))]
pub fn derive_wrapper(item: TokenStream) -> TokenStream {
    wrapper::derive_wrapper(item.into()).into()
}
