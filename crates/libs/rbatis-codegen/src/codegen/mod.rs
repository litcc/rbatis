/// The core logic of code generation is written in func.rs
/// The syntax tree we use is the html tag structure loader_html.rs Element struct
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::ItemFn;
use syn::Token;

pub mod func;
pub mod loader_html;
pub mod parser_html;
pub mod parser_pysql;
pub mod string_util;
pub mod syntax_tree_pysql;

pub struct ParseArgs {
    pub sqls: Vec<syn::LitStr>,
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Self { sqls: r.into_iter().collect() })
    }
}

pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();

    func::impl_fn("", &target_fn.sig.ident.to_string(), &args.to_string(), true, &[])
        .into()
}

pub fn rb_html(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn = syn::parse(func).unwrap();

    parser_html::impl_fn_html(&target_fn, &args)
}

/// support py_sql fn convert
pub fn rb_py(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn = syn::parse(func).unwrap();

    parser_pysql::impl_fn_py(&target_fn, &args)
}
