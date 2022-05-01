use proc_macro::TokenStream;
use syn::{parse, ItemFn};

#[proc_macro_attribute]
pub fn lambda_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = match parse::<ItemFn>(item) {
        Ok(res) => res,
        Err(_) => panic!("lambda_handler should be a function"),
    };
    let func_name = &func.sig.ident;

    let tokens = quote::quote! {
        #func

        #[tokio::main]
        #[tracing::instrument]
        async fn main() -> Result<(), lambda_runtime::Error> {
            use std::str::FromStr;
            let log_level = std::env::var("LOG_LEVEL")
                .ok()
                .and_then(|lvl| tracing::Level::from_str(&lvl).ok())
                .unwrap_or(Level::INFO);
            tracing_subscriber::fmt()
                .with_max_level(log_level)
                .json()
                .init();

            let func = service_fn(#func_name);
            lambda_runtime::run(func).await?;
            Ok(())
        }
    };

    tokens.into()
}
