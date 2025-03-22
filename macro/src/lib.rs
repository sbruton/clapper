use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

/// Macro to wrap a `main(args: Parser, terminated: Arc<AtomicBool>)` that returns `impl Future<Output = Result<(), E>>`.
///
/// On error, it prints the error, then calls `std::process::exit(e.exit_code())`.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let sig = &ast.sig;
    let FnArg::Typed(arg) = sig.inputs.first().expect("no arguments found") else {
        panic!("first argument must be a clap::Parser");
    };
    let arg_ty = &arg.ty;
    let out = &sig.output;
    let block = &ast.block;
    let fn_name = &sig.ident;

    let inner_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());

    let output = quote! {
        #[tokio::main]
        async fn #fn_name() {
            use ::clapper::prelude::*;

            let args = #arg_ty::parse();

            let terminated = ::std::sync::Arc::new(::std::sync::atomic::AtomicBool::new(false));

            ::clapper::ctrlc::set_handler({
                let terminated = terminated.clone();
                move || {
                    terminated.store(true, ::std::sync::atomic::Ordering::Relaxed);
                }
            }).expect("failed to set sigterm handler");

            if let Err(err) = #inner_name(args, terminated).await {
                eprintln!("Error: {err}");
                ::std::process::exit(err.exit_code());
            }
        }

        async fn #inner_name(args: #arg_ty, terminated: ::std::sync::Arc<::std::sync::atomic::AtomicBool>) #out #block
    };

    output.into()
}
