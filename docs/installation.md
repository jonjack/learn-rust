

# Install Rust


The below instructions follow the [recommended approach](https://www.rust-lang.org/tools/install) for installation on MacOS using [rustup](https://rust-lang.github.io/rustup/concepts/index.html). For more extensive installation information, refer to the [official docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

On MacOS or Linux.

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

On mac you should also install a C compiler as some Rust packages depend on C code.

```sh
$ xcode-select --install
```

To check your Rust installation.

```sh
❯ rustc --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

To troubleshoot the installation first make sure Rust is in your %PATH%

```sh
$ echo $PATH
```

It should typically be installed to `~/.cargo/bin`


## Upgrade Rust

Upgrading Rust and rustc (Rust's compiler) on a Mac is straightforward using rustup, the official tool for managing Rust versions. Here's how to do it:

#### 1. Check Current Rust Version

First, check your current Rust version to see if you need an update:

```bash
rustc --version
```

#### 2. Update rustup

To ensure that rustup itself is up to date (you’ll use this to manage Rust versions), run:

```sh
rustup self update
```

#### 3. Upgrade Rust and rustc

Now, upgrade Rust and the rustc version:

```sh
rustup update
```

This command updates your toolchain to the latest stable version of Rust and rustc. If you're using a specific toolchain (e.g., nightly or beta), it will also update those versions.

#### 4. Verify the Upgrade

Check if the upgrade was successful by running:

```sh
rustc --version
```

You should see the updated version of rustc.

#### 5. If You Want a Specific Version

If you want to install or switch to a specific version of Rust (e.g., a particular stable or nightly release), you can do it like this:

```sh
rustup install 1.56.0  # Replace with your desired version
```

Switch to the newly installed version:

```sh
rustup default 1.56.0  # Replace with the version you want to use
```



