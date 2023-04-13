use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod attributes;
pub(crate) mod blocks;
pub(crate) mod column;
pub(crate) mod engine;
pub(crate) mod errors;
pub(crate) mod info;
pub(crate) mod relation;
pub(crate) mod utils;

use info::Info;

#[proc_macro_derive(WeldsModel, attributes(welds))]
pub fn model_gen(input: TokenStream) -> TokenStream {
    match model_gen_inner(input) {
        Ok(q) => q,
        Err(err) => quote! { std::compile_error!(#err); }.into(),
    }
}

fn model_gen_inner(input: TokenStream) -> errors::Result<TokenStream> {
    // Gather the Info needed to build all the code snipits
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let info = Info::new(&ast)?;

    // write all the code snipits
    let p1 = blocks::has_schema(&info);
    let p2 = blocks::write_to_args(&info);
    let p3 = blocks::define_schema(&info);
    let p4 = blocks::table_info(&info);
    let p5 = blocks::table_columns(&info);
    let p6 = blocks::relations(&info);
    let p7 = blocks::unique_identifier(&info);
    let p8 = blocks::impl_struct(&info);

    let q = quote! {
        #p1
        #p2
        #p3
        #p4
        #p5
        #p6
        #p7
        #p8
    };

    Ok(q.into())
}