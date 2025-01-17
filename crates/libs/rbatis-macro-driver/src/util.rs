use quote::quote;
use quote::ToTokens;
use syn::FnArg;
use syn::ItemFn;
use syn::Pat;
use syn::ReturnType;

//find and check method return type
pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    if let ReturnType::Type(_, b) = &target_fn.sig.output {
        return_ty = b.to_token_stream();
    }
    let s = format!("{}", return_ty);
    if !s.contains(":: Result") && !s.starts_with("Result") {
        return_ty = quote! {
             rbatis::Result <#return_ty>
        };
    }
    return_ty
}

pub(crate) fn get_fn_args(target_fn: &ItemFn) -> Vec<&Pat> {
    target_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(t) = arg {
                Some(t.pat.as_ref())
                //println!("arg_name {}", arg_name);
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

//find and check method return type
pub(crate) fn find_fn_body(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    //del todos
    let mut target_fn = target_fn.clone();
    let mut new_stmts = vec![];
    for x in &target_fn.block.stmts {
        let token =
            x.to_token_stream().to_string().replace("\n", "").replace(" ", "");
        if token.eq("todo!()") ||
            token.eq("unimplemented!()") ||
            token.contains("impled!()")
        {
            //nothing to do
        } else {
            new_stmts.push(x.to_owned());
        }
    }
    target_fn.block.stmts = new_stmts;
    target_fn.block.to_token_stream()
}

pub(crate) fn is_query(return_source: &str) -> bool {
    !return_source.contains("ExecResult")
}

pub(crate) fn is_rb_ref(ty_stream: &str) -> bool {
    if ty_stream.contains("RBatis") ||
        ty_stream.contains("RBatisConnExecutor") ||
        ty_stream.contains("RBatisTxExecutor") ||
        ty_stream.contains("RBatisTxExecutorGuard") ||
        ty_stream.contains("Executor")
    {
        return true;
    }
    false
}
