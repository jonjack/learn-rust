

# Rust Lambdas


<!--TOC-->


## Resources

[AWS: Building Lambda functions with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html)     
[AWS: Processing HTTP events with Rust](https://docs.aws.amazon.com/lambda/latest/dg/rust-http-events.html)      


## Starting Point

The AWS developer guide for [Building Lambdas with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html) is a very good starting point for writing your code and following best practises, as is the documentation in the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) repository.

The documentation for each relevant crate on crates.io also provides sometimes useful information eg. see [lambda_runtime](https://crates.io/crates/lambda_runtime), [lambda_http](https://crates.io/crates/lambda_http), [aws_lambda_events](https://crates.io/crates/aws_lambda_events).

The next section summarises some of the main points.

### Handler code

The `async fn main()` function is the entrypoint to your Lambda.

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
 lambda_runtime::run(service_fn(handler)).await
}
```

Since AWS does not yet provide a dedicated Rust runtime environment this entrypoint code is required. At some point, if Rust gets its own Lambda runtime, this main() method may not be required and we shall just be able to provide a handler() function as with the other Lambda runtimes. 

`tokio` is the async runtime used by the Rust client.

### Handling AWS events

See [AWS: Processing HTTP events with Rust](https://docs.aws.amazon.com/lambda/latest/dg/rust-http-events.html)      

- API Gateway, Load Balancers, and Lambda function URLs can allsend HTTP events to Lambda. 
- The [aws_lambda_events](https://crates.io/crates/aws_lambda_events) crate provides the functionality to serialize and deserialize AWS Lambda events into strongly-typed Rust structs. It provides many Lambda event types so to reduce compilation time, use feature flags to activate only the events you need eg.
    ```rust
    aws_lambda_events = { version = "0.8.3", 
        default-features = false, features = ["apigw"] }
    ```

An [example](https://docs.aws.amazon.com/lambda/latest/dg/rust-http-events.html) from the AWS developer guide. See lambda_http for an equivalent implementation but using a generic `Request` type.

```rust
use lambda_runtime::{service_fn, Error, LambdaEvent};
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>) 
						-> Result<ApiGatewayProxyResponse, Error> {
let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some("Hello AWS Lambda HTTP request".into()),
        headers,
    };
    Ok(resp)
}
```

### lambda_http

The [lambda_http](https://crates.io/crates/lambda_http) crate provides an abstraction over the lambda_runtime and aws_lambda_events crates so that you can work with native HTTP types, regardless of which upstream AWS service sends the request. 

The following is equivalent to the previous example and works with API Gateway, ELBs and Lambda function URLs out of the box. It is also noticeably simpler.

```rust
use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};

async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request")
        .map_err(Box::new)?;
    Ok(resp)
}
```

Note that the lambda_runtime and aws_lambda_events crates do not need to be imported since lambda_http uses them anyway underneath.

### Example Lambdas

There are a lot of [example](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples) Lambdas in the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) repo which you can use for a foundation if required when writing you own functions.

[DynamoDB demo](https://github.com/benbpyle/rust-ddb-get-api/tree/main)

### Axum web framework

Here is an example of a Lambda that uses lambda_http and the Axum web framework - [http-axum](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/http-axum/src/main.rs).

TODO: Figure out what [Axum](https://github.com/tokio-rs/axum/tree/main) does and whether it may have any uses for us.

See [list of projects and tutorials](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase).

### Notable crates for working with Lambdas

[docs.rs](https://docs.rs/) is one of the main places to go to find out info on crates, their features and ther versions.

There is a wealth of information in some of the repos listed below so use that as your starting point and a place to check when overcoming obstacles.

- [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main) project is the main repo to check out when building Lambdas in Rust. The [README](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) has some very useful info and examples for dealing with errors, deployment approaches, local development, debugging, tracing and logging, event objects.
- [lambda-http](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http) does the grunt work of converting payloads into Rust HTTP objects - making it easy to write Lambdas that deal with API Gateway proxy events. The [README](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-http/README.md) has some very useful info and examples of writing Rust Lambdas using lambda-http.
- [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events) - [Docs](https://docs.rs/crate/aws_lambda_events/latest) - Rust structs for all AWS event payloads.

### Adding Dependencies & using feature flags

When you add a dependency to a project (Cargo.toml), by default, all features will be added. For example, take the [aws_lambda_events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events) crate which provides Rust structs and serialization/deserialization support for all AWS Lambda event payloads , eg. events sent from API Gateway, ALB etc. You can see by looking at the [Rust crate docs](https://docs.rs/crate/aws_lambda_events/latest/features) that there are a ton of features. These will all be packaged with your application if added as a default dependency as follows.

```sh
$ cargo add aws_lambda_events

# resulting entry in Cargo.toml
[dependencies]
aws_lambda_events = "0.15.1"
```

You can cherry pick a subset of features however, by specifying the list of features you wish to use/packaege - this  helps minimize the footprint of your binary. For example, if we are only using API Gateway and ALB events in our Lambda functions then we can just take them and exclude all others as follows.

```sh
$ cargo add aws_lambda_events --no-default-features --features apigw,alb

# resulting entry in Cargo.toml
[dependencies]
aws_lambda_events = { version = "0.15.1", default-features = false, features = ["apigw"] }
```




## Build & Deployment Methods

The [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) project documents several approaches for [deploying your functions](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md#2-deploying-the-binary-to-aws-lambda) to AWS Lambda, ie.:-

- Cargo Lambda
- AWS CLI
- AWS SAM

An obvious choice is to use [Cargo Lambda](https://www.cargo-lambda.info/commands/deploy.html).


### Using Cargo-Lambda

[Cargo Lambda](https://www.cargo-lambda.info/) -> Run, Build, and Deploy Rust functions on AWS Lambda natively from your computer.

This is a really cool tool if you are writing Lambdas in Rust and has the following features:-

- For local development it will build a binary to run on whatever platform you are developing on (eg. macOS).
- It uses Zig to cross compile from your platform (eg. macOS) to Amazon Linux (which is what Lambdas run on on AWS).
- It allows you to [run your Lambdas locally](https://www.cargo-lambda.info/commands/watch.html#cargo-lambda-watch) for development and testing.






