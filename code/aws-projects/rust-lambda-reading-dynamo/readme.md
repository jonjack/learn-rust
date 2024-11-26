# README

This project is based on this [Youtube tutorial](https://www.youtube.com/watch?v=EqV5wKD233c) for creating a Lambda that writes to a DynamoDB table.

- It is created with Cargo (not cargo-lambda).
- Some of the code is slightly different from the Youtube code since I used the latest crates and so some things had changed. The tutorial code is here - https://github.com/MoonKraken/youtube/tree/main/RustOnAWS
- The project demonstrates a couple of things:-
  - How to do a simple read from a DynamoDB table.
  - How to build a deployable lambda.zip using a build script and Docker (old school).

```sh
cargo new rust-lambda-reading-dynamo --bin
```

Open the project in VS Code and add the following dependencies, either manually or via the terminal.

```sh
cargo add aws-config
cargo add dynamodb
cargo add lambda_runtime
cargo add log
cargo add serde
cargo add serde_derive
cargo add serde_json
cargo add simple_logger
cargo add tokio
cargo add uuid
```

Amend the uuid dependency entry to include the 'v4' feature

You will end up with a Cargo.toml like this.

```sh
[package]
name = "rust-lambda-reading-dynamo"
version = "0.1.0"
edition = "2021"

[dependencies]
aws-config = "1.5.10"
aws-sdk-dynamodb = "1.53.0"
lambda_runtime = "0.13.0"
log = "0.4.22"
serde = "1.0.215"
serde_derive = "1.0.215"
serde_json = "1.0.132"
simple_logger = "5.0.0"
tokio = "1.41.1"
uuid = { version = "1.11.0", features = ["v4"] }
```

The build/deployment file "rust_lambda_helpers.sh"
