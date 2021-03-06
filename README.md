Useful macros for reducing repetition in Rust code targeting AWS.

Currently, it only contains one macro for the boilerplate around
creating a Lambda handler with tracing enabled using
[lambda-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) and
[tracing](https://docs.rs/tracing/latest/tracing/).

```rust
use aws_codegen::lambda_handler;
use lambda_runtime::{Error, LambdaEvent};
use serde_json::{json, Value};

#[lambda_handler]
async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
```

It has been tested with the following versions:

```toml
[dependencies]
lambda_runtime = "0.5.0"
tokio = { version = "1.17.0", features = ["parking_lot"] }
tracing = { version = "0.1.34", features = ["std"] }
tracing-futures = "0.2.5"
tracing-subscriber = { version = "0.3.11", features = ["fmt", "json"] }
```