#![allow(unused_assignments)]
extern crate proc_macro;
extern crate rbatis_codegen;

use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::ItemFn;
use syn::Token;

use crate::macros::html_sql_impl::impl_macro_html_sql;
use crate::macros::py_sql_impl::impl_macro_py_sql;
use crate::macros::sql_impl::impl_macro_sql;
use crate::proc_macro::TokenStream;

mod macros;
mod util;

struct ParseArgs {
    pub sqls: Vec<syn::LitStr>,
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Self { sqls: r.into_iter().collect() })
    }
}

/// auto create sql macro,this macro use RB.query_prepare and RB.exec_prepare
/// for example1:
///```log
///     use rbatis::plugin;
///     use rbatis::executor::Executor;
///     #[derive(serde::Serialize,serde::Deserialize)]
///     pub struct MockTable{}
///
///     #[sql("select * from biz_activity where id = ?")]
///     async fn select(rb:&dyn Executor, name: &str) -> MockTable {}
/// ```
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        println!(
            "............#[sql] '{}'...................\n {}",
            func_name_ident, stream
        );
        println!(".......................................................");
    }

    stream
}

/// py sql create macro,this macro use RB.py_query and RB.py_exec
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
///
/// #[py_sql("select * from biz_activity where delete_flag = 0")]
/// async fn py_select_page(rb: &dyn Executor, name: &str) -> Vec<MockTable> { }
/// ```
///  or more example1:
///```log
/// use rbatis::executor::Executor;
/// use rbatis::py_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
///
/// #[py_sql("
///     SELECT * FROM biz_activity
///     if  name != null:
///       AND delete_flag = #{del}
///       AND version = 1
///       if  age!=1:
///         AND version = 1
///       AND version = 1
///     AND a = 0
///       AND version = 1
///     and id in (
///     trim ',': for item in ids:
///       #{item},
///     )
///     and id in (
///     trim ',': for index,item in ids:
///       #{item},
///     )
///     trim 'AND':
///       AND delete_flag = #{del2}
///     choose:
///         when age==27:
///           AND age = 27
///         otherwise:
///           AND age = 0
///     WHERE id  = '2'")]
///   pub async fn py_select_rb(rb: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
/// or read from file
///
/// ```rust
/// //#[rbatis::py_sql(r#"include!("C:/rs/rbatis/target/debug/xx.py_sql")"#)]
/// //pub async fn test_same_id(rb: &dyn Executor, id: &u64) -> Result<Value, Error> { impled!() }
/// ```
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        use rust_format::Formatter;
        use rust_format::RustFmt;
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!(
            "............#[py_sql] '{}'............\n {}",
            func_name_ident, code
        );
        println!(".......................................................");
    }
    stream
}

/// html sql create macro,this macro use RB.py_query and RB.py_exec
/// for example1:
/// ```log
/// use rbatis::executor::Executor;
/// use rbatis::html_sql;
/// #[derive(serde::Serialize,serde::Deserialize)]
/// pub struct MockTable{}
///
/// #[html_sql(r#"
/// <select id="select_by_condition">
///         `select * from activity`
///         <where>
///             <if test="name != ''">
///                 ` and name like #{name}`
///             </if>
///             <if test="dt >= '2023-11-03T21:13:09.9357266+08:00'">
///                 ` and create_time < #{dt}`
///             </if>
///             <choose>
///                 <when test="true">
///                     ` and id != '-1'`
///                 </when>
///                 <otherwise>and id != -2</otherwise>
///             </choose>
///             ` and `
///             <trim prefixOverrides=" and">
///                 ` and name != '' `
///             </trim>
///         </where>
///   </select>"#)]
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
/// or from file
/// ```log
/// #[html_sql("xxxx.html")]
/// pub async fn select_by_name(rbatis: &dyn Executor, name: &str) -> Option<MockTable> {}
/// ```
#[proc_macro_attribute]
pub fn html_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParseArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_html_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    if cfg!(debug_assertions) {
        use quote::ToTokens;
        use rust_format::Formatter;
        use rust_format::RustFmt;
        let func_name_ident = target_fn.sig.ident.to_token_stream();
        let stream_str = stream.to_string().replace("$crate", "rbatis");
        let code = RustFmt::default()
            .format_str(&stream_str)
            .unwrap_or_else(|_e| stream_str.to_string());
        println!(
            "............#[html_sql] '{}'............\n {}",
            func_name_ident, code
        );
        println!(".......................................................");
    }
    stream
}

/// proxy rbatis_codegen rb_py
#[proc_macro_attribute]
pub fn rb_py(args: TokenStream, func: TokenStream) -> TokenStream {
    rbatis_codegen::rb_py(args, func)
}

/// proxy rbatis_codegen rb_html
#[proc_macro_attribute]
pub fn rb_html(args: TokenStream, func: TokenStream) -> TokenStream {
    rbatis_codegen::rb_html(args, func)
}

#[proc_macro_attribute]
pub fn snake_name(args: TokenStream, func: TokenStream) -> TokenStream {
    macros::snake_name::snake_name(args, func)
}

#[proc_macro_derive(RefModel)]
pub fn derive_ref_model(item: TokenStream) -> TokenStream {
    use quote::ToTokens;
    let input = parse_macro_input!(item as syn::DeriveInput);

    let struct_name = &input.ident;
    let token = match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if let syn::Fields::Named(ref fields_name) = fields {
                let mut field_name_list = Vec::new();
                let mut field_type_list = Vec::new();

                fields_name.named.iter().for_each(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = field.ty.clone();
                    field_name_list.push(field_name);
                    field_type_list.push(field_type);
                });

                let change_fields: Vec<_> = fields_name
                    .named
                    .iter()
                    .map(|field| {
                        let field_type = field.ty.clone();
                        let field_name = field.ident.as_ref().unwrap();

                        let change_type = quote::quote! {
                            Option<std::borrow::Cow<'__ref_struct, #field_type>>
                        };

                        if !&field.attrs.is_empty() {
                            let attrs = &field.attrs;
                            quote::quote! {
                                #( #attrs )*
                                pub #field_name: #change_type
                            }
                        } else {
                            quote::quote! {
                                pub #field_name: #change_type
                            }
                        }
                    })
                    .collect();

                let change_struct = quote::format_ident!("{}Ref", struct_name);
                let struct_attrs = &input.attrs;
                let ref_body = quote::quote! {
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                    #( #struct_attrs )*
                    pub struct #change_struct<'__ref_struct> {
                        #( #change_fields, )*
                    }

                    impl #struct_name {
                        /// Returns the names of all the fields in the table
                        pub fn all_fields() -> &'static [&'static str] {
                            static FIELDS: &'static [&'static str] = &[
                                #( stringify!(#field_name_list), )*
                            ];
                            FIELDS
                        }

                        /// Determines if the specified field exists in the table
                        pub fn has_field(field_name: &str) -> bool {
                            Self::all_fields().contains(&field_name)
                        }
                    }

                    impl<'__ref_struct> #change_struct<'__ref_struct>{
                        /// Determine if all fields in the table are empty
                        pub fn all_empty(&self) -> bool {
                            return !(#( self.#field_name_list.is_some() )||*)
                        }

                        /// Determines if the specified field exists in the table
                        pub fn has_field(field_name: &str) -> bool {
                            #struct_name::all_fields().contains(&field_name)
                        }

                        /// Returns the names of all the fields in the table
                        pub fn all_fields() -> &'static [&'static str] {
                            #struct_name::all_fields()
                        }

                    }

                    impl<'__ref_struct> From<#struct_name> for #change_struct<'__ref_struct> {
                        fn from(value: #struct_name) -> Self {
                            #change_struct {
                                #( #field_name_list: Some(std::borrow::Cow::Owned(value.#field_name_list)), )*
                            }
                        }
                    }

                    impl<'__ref_struct> Default for #change_struct<'__ref_struct> {
                        fn default() -> Self {
                            #change_struct {
                                #( #field_name_list: None, )*
                            }
                        }
                    }
                };
                Ok(ref_body)
            } else {
                Err(vec![syn::Error::new(
                    struct_name.span(),
                    "Sorry, RefModel does not support complex structures",
                )])
            }
        }
        _ => Err(vec![syn::Error::new(
            struct_name.span(),
            "Sorry, RefModel only supports Struct.",
        )]),
    };

    token.expect("Invalid Rust code provided").to_token_stream().into()
}
