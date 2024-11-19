

# Rust Lambdas


<!--TOC-->


---


## Design considerations

### Example Lambdas

There are a lot of [example](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples) Lambdas in the [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) repo which you can use for a foundation if required when writing you own functions.

[DynamoDB demo](https://github.com/benbpyle/rust-ddb-get-api/tree/main)


### Notable crates for working with Lambdas

[docs.rs](https://docs.rs/) is one of the main places to go to find out info on crates, their features and ther versions.

There is a wealth of information in some of the repos listed below so use that as your starting point and a place to check when overcoming obstacles.

- [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main) project is the main repo to check out when building Lambdas in Rust. The [README](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) has some very useful info and examples for dealing with errors, deployment approaches, local development, debugging, tracing and logging, event objects.
- [lambda-http](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http) does the grunt work of converting payloads into Rust HTTP objects - making it easy to write Lambdas that deal with API Gateway proxy events. The [README](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-http/README.md) has some very useful info and examples of writing Rust Lambdas using lambda-http.
- [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events) - [Docs](https://docs.rs/crate/aws_lambda_events/latest) - Rust structs for all AWS event payloads.


## Creating (Rust) Lambdas


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






