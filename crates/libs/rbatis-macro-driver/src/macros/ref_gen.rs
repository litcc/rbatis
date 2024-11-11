use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) fn derive_ref_model(p0: TokenStream) -> TokenStream {
    use quote::ToTokens;
    let input = parse_macro_input!(p0 as syn::DeriveInput);
    // eprintln!("input: {:#?}", input);
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
                    // #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    token
        .expect("Invalid Rust code provided")
        .to_token_stream()
        .into()
}


// #[serde(serialize_with = "path")]
// #[serde(deserialize_with = "path")]
// #[serde(with = "module")]