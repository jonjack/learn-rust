

# Cargo Lambda


An extension to Cargo that helps you Build, Run and Deploy Rust functions on AWS Lambda.

[Official Cargo Lambda site](https://www.cargo-lambda.info/)


## Overview

This is a really cool tool if you are writing Lambdas in Rust and has the following features:-

- For local development it will build a binary to run on whatever platform you are developing on (eg. macOS).
- It uses Zig to cross compile from your platform (eg. macOS) to Amazon Linux (which is what Lambdas run on on AWS).
- It allows you to [run your Lambdas locally](https://www.cargo-lambda.info/commands/watch.html#cargo-lambda-watch) for development and testing.

