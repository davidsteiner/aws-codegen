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
            let log_env = std::env::var_os("LOG_LEVEL")
                .and_then(|val| val.to_str().map(|str| str.to_owned()))
                .unwrap_or_else(|| "info".to_owned());
            let log_level = tracing::Level::from_str(&log_env)?;

            tracing_subscriber::fmt()
                .with_max_level(log_level)
                .json()
                .init();

            let handler = lambda_runtime::handler_fn(#func_name);
            lambda_runtime::run(handler).await?;

            Ok(())
        }
    };

    tokens.into()
}
