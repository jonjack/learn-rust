
# This is quite a primitive script for creating a deployable Lambda.zip containing our Rust Lambda
# You would not really do this in practise - it is just for knowledge purposes
# In practise it would be better to use AWS CDK or cargo-lambda tool.

# Overview
# --------
# It requires docker to be installed - on mac use 'brew install --cask docker'
# al2build will run a docker container and build our binary 
# zipRustLambda will create a lambda.zip that we can then upload to our AWS account via the console or CLI

LAMBDA_ARCH="linux/arm64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.
RUST_TARGET="aarch64-unknown-linux-gnu" # corresponding with the above, set this to aarch64 or x86_64 -unknown-linux-gnu for ARM or x86 functions.
RUST_VERSION="latest" # Set this to a specific version of rust you want to compile for, or to latest if you want the latest stable version.
PROJECT_NAME="rust-lambda-reading-dynamo"

# This will run a linux docker container and execute a cargo build within it
# To save the result of the build from our container to our local system we map a volume using -v
# -v ${PWD}:/usr/src/myapp maps the local path we run the script from {PWD} to container path /usr/src/myapp 
# -w is working directory in container
# So we start a linux container, map our project root {PWD} to container path /usr/src/myapp and 
# execute the cargo build command in this container path - which is our mapped project root
# the path mapping means that the result of the build ends up in 'target' in our project dir
al2build() {
	docker run --platform ${LAMBDA_ARCH} \
	  --rm --user "$(id -u)":"$(id -g)" \
	  -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
  	cargo build --release --target ${RUST_TARGET} # This line can be any cargo command
}

# copies the binary file built from al2build to temp folder 'bootstrap' and creates a deployable zip
zipRustLambda() {
	cp ./target/${RUST_TARGET}/release/${PROJECT_NAME} ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
}