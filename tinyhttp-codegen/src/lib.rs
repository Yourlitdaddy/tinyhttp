use std::ops::Deref;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: syn::ItemFn = syn::parse(item.clone()).unwrap();
    //eprintln!("{:#?}\n{:#?}", attr, item);
    let args = parse_macro_input!(attr as syn::AttributeArgs);

    let sig = item_fn.sig;
    let name = sig.ident.clone();
    let body = item_fn.block.deref();
    let path_token = args[0].clone();
    let return_type = sig.output;

    let body_args = sig.inputs;
    let is_body_args = !body_args.is_empty();
    //eprintln!("LEN: {}", body_args.len());

    let mut path;
    match path_token.clone() {
        syn::NestedMeta::Meta(_) => panic!("IN TOKEN MATCH!"),
        syn::NestedMeta::Lit(e) => match e {
            syn::Lit::Str(e) => {
                path = e.value();
            }
            syn::Lit::ByteStr(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Byte(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Char(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Int(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Float(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Bool(_) => panic!("IN TOKEN MATCH!"),
            syn::Lit::Verbatim(_) => panic!("IN TOKEN MATCH!"),
        },
    };

    let new_wildcard = if path.contains("/:") {
        let path_clone = path.clone();
        let mut iter = path_clone.split(":");
        path = iter.next().unwrap().to_string();
        let id = iter.next().unwrap().to_string();
        if path.len() != 1 {
            path.pop();
        };
        quote! {get_route = get_route.set_wildcard(#id.into());}
    } else {
        quote! {}
    };

    let mut return_type_str = String::new();

    match return_type {
        syn::ReturnType::Default => return_type_str = "NO RETURN TYPE!".to_string(),
        syn::ReturnType::Type(_, value) => match *value.clone() {
            syn::Type::Path(stream) => {
                return_type_str = stream.path.segments.last().unwrap().ident.to_string()
            }
            _ => return_type_str = "NO RETURN TYPE!".to_string(),
        },
    };

    let is_ret_type_res = if return_type_str == "Response" {
        true
    } else {
        false
    };

    let new_get_body = if is_ret_type_res {
        quote! {
            fn body(#body_args) -> Response {
                #body.into()
            }

            get_route = get_route.set_body_with_res(body);
        }
    } else if is_body_args {
        quote! {
           fn body(#body_args) -> Vec<u8> {
                #body.into()
            }

            get_route = get_route.set_body_with(body);
        }
    } else {
        quote! {
            fn body() -> Vec<u8> {
                #body.into()
            }

            get_route = get_route.set_body(body);
        }
    };

    let output = quote! {
        fn #name() -> Box<dyn Route> {
            let mut get_route = GetRoute::new()
                .set_path(#path.into())
                .set_is_args(#is_body_args)
                .set_is_ret_res(#is_ret_type_res);

            #new_wildcard
            #new_get_body

            Box::new(get_route)
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    //eprintln!("{:#?}\n{:#?}", attr, item);
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let item: syn::ItemFn = syn::parse(item).unwrap();

    let fn_args = item.sig.inputs;
    let name = item.sig.ident.clone();
    let body = item.block.deref();
    let return_type = item.sig.output;

    let path_token = args[0].clone();
    let is_body_args = !fn_args.is_empty();

    let mut path;
    match path_token.clone() {
        syn::NestedMeta::Meta(_) => panic!("IN MATCH RETURN TYPE!"),
        syn::NestedMeta::Lit(e) => match e {
            syn::Lit::Str(e) => {
                path = e.value();
            }
            syn::Lit::ByteStr(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Byte(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Char(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Int(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Float(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Bool(_) => panic!("IN MATCH RETURN TYPE!"),
            syn::Lit::Verbatim(_) => panic!("IN MATCH RETURN TYPE!"),
        },
    };
    let new_wildcard = if path.contains("/:") {
        let path_clone = path.clone();
        let mut iter = path_clone.split(":");
        path = iter.next().unwrap().to_string();
        let id = iter.next().unwrap().to_string();
        if path.len() != 1 {
            path.pop();
        };
        quote! {post_route = post_route.set_wildcard(#id.into());}
    } else {
        quote! {}
    };

    let mut return_type_str = String::new();

    match return_type {
        syn::ReturnType::Default => return_type_str = "NO RETURN TYPE!".to_string(),
        syn::ReturnType::Type(_, value) => match *value.clone() {
            syn::Type::Path(stream) => {
                return_type_str = stream.path.segments.last().unwrap().ident.to_string()
            }
            _ => return_type_str = "NO RETURN TYPE!".to_string(),
        },
    };

    let is_ret_type_res = if return_type_str == "Response" {
        true
    } else {
        false
    };

    let new_post_body = if is_ret_type_res {
        quote! {
            fn body(#fn_args) -> Response {
                #body.into()
            }

            post_route = post_route.set_body_with_res(body);
        }
    } else if is_body_args {
        quote! {
            fn body(#fn_args) -> Vec<u8> {
                #body.into()
            }

            post_route = post_route.set_body_with(body);
        }
    } else {
        quote! {
            fn body() -> Vec<u8> {
                #body.into()
            }

            post_route = post_route.set_body(body);
        }
    };

    let output = quote! {
        fn #name() -> Box<Route> {
            let mut post_route = PostRoute::new()
                .set_path(#path.into())
                .set_is_args(#is_body_args)
                .set_is_ret_res(#is_ret_type_res);

            #new_wildcard
            #new_post_body

            Box::new(post_route)
        }
    };

    /*let output = quote! {
        fn #name() -> (String, Vec<u8>, Method) {
            fn body() #output {
                #body
            }

            (#path.into(), body().into(), Method::POST)
        }
    };*/

    output.into()
}
