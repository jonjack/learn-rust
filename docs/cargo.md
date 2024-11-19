

# Cargo


Cargo is the official Rust package manager - it builds your projects, fetches dependencies.and creates distributions.

[The official Cargo book](https://doc.rust-lang.org/cargo/index.html)


---


<!--TOC-->


## Terminology

I found this a bit confusing initially so here is my summary of some of the common terms.

- [package](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) - this is basically a project's source. It includes source files and a manifest (Cargo.toml) which describes the package.
- [artifact](https://doc.rust-lang.org/cargo/appendix/glossary.html#artifact) - one or more files generated as a result of compilation. They may be an executable binary, a linkable library, or docs.
- [crate](https://doc.rust-lang.org/cargo/appendix/glossary.html#crate) - this is basically an application, either a distributable library or an executable program. Loosely speaking the term may refer to either:-
	- A compiled target artifact - probably most common usage.
	- Source code.
	- A compressed package (dependency) fetched from a registry. 
- [target](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) has multiple meanings depending on the context. In the context of describing and building projects (ie. a _cargo target_), a _target_ is the definition for what the compilation of the project should produce. They may be inferred implicitly from the directory layout of source files. A target can be:-
	- `[lib]` - a library. A manifest may contain only one.
	- `[bin]` - an executable program. A manifest file can have multiple `[bin]` targets to produce several executables.
	- `[example]` - example uses of a library. By default, they are executable binaries with a `main()` function.
	- `[test]` - these are either unit or integration test files.
	- `[lib]` - these are performance test files.
- [modules](https://doc.rust-lang.org/cargo/appendix/glossary.html#module) - are a way to split code into logical units by providing a unique namespace. They organize code into areas of related functionality or to control visibility. They are similar to _packages_ in the Java ecosystem.


## Configuration

### Cargo.toml

...

### Cargo.lock

[Ensuring Reproducible Builds with the Cargo.lock File](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file)

- This file allows to re-build a project with the same version of dependencies each time. 
- Since it is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.


## Cargo Commands

### At a glance

```sh
# build a project
$ cargo build

```

### Creating projects

[Package commands](https://doc.rust-lang.org/cargo/commands/package-commands.html)

Use either `cargo init` or `cargo new` depending on whether you are creating a brand `new` project or initializing a manifest for existing source.

Note that creating a project will also, by default, create a new version control repository for it, unless you are creating the project within a directory of an existing repository. The default repository type is **git** but this can be controlled using the `--vcs` [flag](https://doc.rust-lang.org/cargo/commands/cargo-new.html#new-options).

### Build commands

[build commands](https://doc.rust-lang.org/cargo/commands/build-commands.html)

