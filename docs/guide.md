

# Rust Guide


A summary of my Rust learnings.

These notes have largely been guided by reading the de facto Rust guide - [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html). 

---


<!--TOC-->


## Cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the Rust package manager - it creates projects, downloads your project's dependencies compiles your packages, uploades your distributions to crates.io etc etc.

### Reproducible builds with Cargo.lock

See [Ensuring Reproducible Builds with the Cargo.lock File](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file)

The Cargo.lock file tracks the versions of all the dependencies your projects uses. It uses the lock file to determine which versions it needs each time your project is compiled, which provides two benefits:-

1. Cargo does not have to go and figure out the dependency graph each time it builds youe project (unless you have changed some of the dependency configuration in Cargo.toml between builds).
2. Each build will use the same version of dependencies (unless you update the configuration).

### Cargo `update`

[Updating a Crate to Get a New Version](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#updating-a-crate-to-get-a-new-version)

The `cargo update` command allows you to update versions of dependencies which then updates Cargo.lock.


## General

### Binary & library crates

There are 2 types of crates:-

1. **Binary** - these are executable programs.
2. **Library** - these are non-compiled source code that are intended to be used by other programs and cannot be executed on its own.

### Dependency Management

Any library crates that your crate needs to use need to be specified in Cargo.toml. Cargo understands [SemVar](https://semver.org/) version numbers. The following is actually shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but below 0.9.0.

```sh
[dependencies]
rand = "0.8.5"
```

Cargo considers that any version above 0.8.5 but below 0.9.0 should (as per the specification) have a public API that is compatible with version 0.8.5 and therefore you should be able to use any patch release without breaking your code.

Once you have built your project and `rand` version `0.8.5` has been added to Cargo.lock, every build will use version 0.8.5 unless you either specify a different version in Cargo.toml or you use the **update** command.

When you use `cargo update`, Cargo will ingore Cargo.lock and check for the most recent patch versions of all the dependencies configured in Cargo.toml. In the case of `rand` it will look for anything between 0.8.5 - 0.9.0 (not including 0.9.0). If a version 0.8.6 is available then the source for rand will get updated, recompiled, and the version updated in Cargo.lock.

### Feature Flags

Feature flags (or features) are a way to conditionally enable or disable functionality in your crates. This introduces several benefits:-

1. When you compile your project, any dependencies are also compiled (Rust dependencies use source-based distribution), and any unrequired features are stripped out of the resulting dependency binary. This means that your resulting binary, which packages it's dependant binaries, has a minimla footprint.
2. it also means that compilation is faster when unused features are disabled.

You can use feature flags in your own code in order to provide any consumers of your code the flexibility to choose the functionality they need.

Most crates define a **default** feature set which are enabled implicitly.

```sh
# These are equivalent
[dependencies]
serde = "1.0"
serde = { version = "1.0", features = ["default"] }

# To disable default features:
[dependencies]
serde = { version = "1.0", default-features = false }
```

You need to enable non-default features.

```sh
[dependencies]
some_crate = { version = "1.0", features = ["foo", "bar"] }
```

You can also enable features at build time without changing your configuration which can be helpful for development/testing. Features enabled this way only apply to the current local build.

```sh
cargo build --features "foo bar"
```

### Rust libraries (crates)

Rust calls its libraries "crates" and there are two forms:-

1. Source crate - published on crates.io (package registry, like Maven central or npmjs).
2. Binary crate - `.rlib` (Rust's equivalent of Java's `.jar`)

### Platform specific code

The standard library is designed to be platform-agnostic where possible, but some functionality (like file system operations or networking) needs platform-specific implementations. These different versions ensure you get the right implementation for your target platform.

```sh
use std::os::unix::fs::PermissionsExt; // Only available on Unix-like systems
```

Note that it is possible to write platform-specific code that would prevent cross-compilation to certain targets.


## Compilation

- Rust is an Ahead of Time (AOT) compiled language.
- A unit of compilation in Rust is a crate.
- When you compile a project, all of its dependencies are also compiled because any features (of each dependency) that you have not enabled shall be stripped out of the resulting binary. This is a great optimisation since it means that your resulting binary only contains what it needs and is therefore as small as it can be.
- This is why Rust dependencies use [source-based distribution](https://crates.io/) rather than pre-compiled binaries.

### Rust compiles into platform-specific libraries

Unlike Java (for example), which compiles source code into platform-dependant JARs, Rust compilation generates platform-specific crates. 

- When you compile a Rust library for different platforms (targets), you will get an `.rlib` file for each target platform. This is handled by Rust's cross-compilation system - note that you need appropriate toolchains installed for cross-compilation.
- The Rust's compiler (rustc) will target your current platform's architecture and operating system unless specified otherwise. This default target is called the "host" target.

### Inlining (optimization)

[Inline In Rust](https://matklad.github.io/2021/07/09/inline-in-rust.html)

Inlining is an optimizing transformation which replaces a call to a function with its body during compilation.

```
// The following source code gets replaced at compile time 
fn f(w: u32) -> u32 {
    inline_me(w, 2)
}
fn inline_me(x: u32, y: u32) -> u32 {
    x * y
}

// with this
fn f(w: u32) -> u32 {
    w * 2
}
```


## Organising code

### Preludes (import groups)

The [std::prelude](https://doc.rust-lang.org/std/prelude/index.html) is the set of utilities in the standard library that are automatically imported into every program without you needing to manually import them. This set is kept small and focussed and just includes features that are commonly used across all Rust programs.

In addition to the standard library prelude, various other libraries have their own preludes eg. [std::io::prelude](https://doc.rust-lang.org/std/io/prelude/index.html) includes many common I/O traits so if you are writing some I/O heavy module you may want to just import the io::prelude rather than every part of the io library that you need. All these other preludes have to be manually imported into your modules.

### Imports

- Any feature not in the prelude of the standard libary that you want to use in your code has to be explicitly brought into scope with a `use` statement.

### Re-exporting

Re-exporting allows you to expose internal items through a module's public interface using the `pub use` syntax and is a useful feature for:-

1. Creating a more convenient public API.
2. Organizing your code internally while presenting a different structure externally.
3. Making deeply nested items available at a higher level

```
// In lib.rs
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
}

// Re-export specific items from shapes
pub use shapes::Circle;

// Now users can do:
use my_crate::Circle;
// Instead of:
use my_crate::shapes::Circle;
```

You can also export items (within your crate) from external crates, so users can import them as though they are importing them from your own crate. This is commonly used in the Rust ecosystem for convenience and is particularly iuseful when:-

- Building a wrapper/facade library
- Creating a unified API that combines multiple dependencies
- Providing convenience exports for commonly used items

```
// your_library imports the Serde library and Reqwest
pub use serde::{Serialize, Deserialize};
pub use reqwest::Client as HttpClient;

// Now users of your library can do:
use your_library::Serialize;  // Instead of serde::Serialize
use your_library::HttpClient; // Instead of reqwest::Client
```

### main() function

The `main` function is the entrypoint to a program.

## Memory management (Ownership)

[What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

All programming languages have to manage memory and there are generally 3 approaches:-

1. Some languages have garbage collection that regularly looks for redundant data and takes care of freeing it up for us.
2. In some languages the programmer must explicitly allocate and free the memory. 
3. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.

### Literals, Stack & Heap

**Literal** values are baked into the executable binary code and so are very fast and efficient. They can only be hardcoded into the program because they are immutable and we know at compile time how much memory they need allocating within the program.

Data on the **stack** is fast to access since it is written/removed on a LIFO basis. A program pushes data on top of the stack as it works with it and pops it off when it is done with it. All data on the stack must be of a known size. 

Data of an unknown size (ie. anything mutable) must live on the **heap**.


## Programming Concepts

### Variables & mutability

- `let` keyword creates a new variable.
- All variables are immutable by default.
- `let mut` creates a mutable variable.

**Variables**

```sh
let planet = "earth";           # immutable - cannot assign a new value to planet
let mut greeting = "hello";     # mutable - greeting can be assigned a new value
```

**Constants**

- You cannot use `mut` with constants soince they are implicitly immutable.
- You must annotate the constant with a type.
- They can be declared in any scope.
- They can only be set to a constant expression ie. their value cannot be computed runtime.

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Calling functions (`::`)

The `::` syntax allows us to invoke a function associated with a type. The following calls the `new()` function which is implemented on the `String` type.

```
let mut guess = String::new();
```

### References (`&`)

You can indicate that an argument to a function is a **reference** which allows multiple parts of your code to access the same piece of data without needing to copy that data into memory multiple times. 

- References are a relatively complex feature of Rust.
- Like variables, they are immutable by default so if you want a mutable reference then you need to use `&mut`

```
io::stdin().read_line(&guess)           // immutable reference
io::stdin().read_line(&mut guess)       // mutable reference
```

### Pattern matching to handle errors (`Option` & `Result`)

Operations that can fail will typically return an instance of [Result](https://doc.rust-lang.org/std/result/enum.Result.html) or [Option](https://doc.rust-lang.org/std/option/enum.Option.html).

```
enum Result<T, E> {
    Ok(T),    // Success case with value of type T
    Err(E),   // Error case with error of type E
}

enum Option<T> {
    Some(T),    // Contains a value of type T
    None        // Represents absence of a value
}
```

The key difference between **Option** and **Result** is that Result carries error information in its **Err** variant, while Option just indicates absence with **None**. This makes Result more suitable for operations where you need to know why something failed.

**Example of Option**

The [get()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get) method on [Vector](https://doc.rust-lang.org/std/vec/struct.Vec.html#) returns an Option. Instead of indexing an array or vector directly (which might panic), you can use **get()** and then match over the Option returned to check whether there is a value or not.

```
fn main() {
    let numbers = vec![1, 2, 3];
    match numbers.get(5) {
        Some(number) => println!("Found number: {}", number),
        None => println!("Index out of bounds!"),
    }
}
```

**Example of Result**

Here are a couple of example of processing instances of Result.

```
fn main() {

    // File::open returns Result<File, std::io::Error>
    let file_result = File::open("nonexistent.txt");
    
    // Pattern matching approach
    // In this example we do a further match over the Err to 
    // handle the different error variants
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found!"),
            ErrorKind::PermissionDenied => println!("Permission denied!"),
            other_error => println!("Other error: {:?}", other_error),
        },
    }

    // A more concise way of handling the variants using if let and else
    let file_result = File::open("nonexistent.txt");

    if let Ok(file) = file_result {
        println!("File opened successfully: {:?}", file);
    } else {
        println!("Failed to open file");
    }
    
    // You can also use the ? operator to propagate errors
    // In this case, the caller of read_file() would need to manage the Err
    fn read_file() -> Result<File, std::io::Error> {
        let file = File::open("nonexistent.txt")?;  // Returns error if Err
        Ok(file)
    }
}
```

### Handling Panic (program crashes)

When we say a program "panics," it means the program encountered an unrecoverable error and is terminating abruptly rather than trying to handle the error gracefully.

Common situations that cause panics include:-

- Array/vector access beyond bounds.
- Integer overflow in debug builds.
- Explicitly calling panic!() macro.
- Unwrapping a None value with unwrap().
- Division by zero.

Rust provides two main ways to handle potential panic situations:-

- Using `Result<T, E>` for recoverable errors.
- Using `Option<T>` for values that might be absent.

### Shadowing

[Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) is when you reuse a variable name - a feature often used when you want to convert a value from one type to another. 

```sh
let mut guess = String::new();

io::stdin().read_line(&mut guess).expect("Failed to read line");

# trim the original guess string and parse it into an u32 number type
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

What is interesting about shadowing is that we are essentially giving an immutable variable a new value by essentially creating a new variable which replaces the original one. We can apply transformations to a variable (change its value and even its type) but then the variable shall still be immutable once we have finished.

Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

```sh
let spaces = "   ";         # spaces is a string type
let spaces = spaces.len();  # now its a number type

let mut spaces = "   ";
spaces = spaces.len();      # compiler error - we tried to change the type

let mut spaces = spaces.len();  # this works though because we used let
```

If we were to try this with a mutable variable without using


### Loops

`loop` will create an infinite loop which will exit either when the program panics (because of some unhandled error), or when encountering a `break` statement.

```sh
loop {

    println!("Please input your guess.");

    let mut guess = String::new();

    # wait for user input
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    # if user inputs a string which cannot be parsed then we loop again
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,     # continue to loop again
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;                   # break the loop
        }
    }
}
```


## Types

An adhoc list of common types I have come across.


### `String`

[parse()](https://doc.rust-lang.org/std/primitive.str.html#method.parse) converts a string slice into any type that implements the [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait. It will return `Err` if its not possible to parse into the desired type.

### `Result`

[Result](https://doc.rust-lang.org/std/result/enum.Result.html) is a common return type of functions since it represents either a success or failure . Result is an [enumeration](https://doc.rust-lang.org/book/ch06-00-enums.html) (or enum) which is a type with a fixed set of states, which are commonly referrd to as _variants_, and `Result`'s variants are `Ok` and `Err`.

`Ok` variant represents a successful operation and wraps the successfully generated value.        
`Err` variant means the operation failed, and Err contains information about how or why the operation failed.

#### `expect()`

Result has an [expect(](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)) method which you can call:-

- If the instance of Result is an `Ok` value then expect() will return the value so you can use it.
- If the Result is an `Err` value then expect() will crash the program and display the message passed to it.

When a method returns an instance of Result and you do not use that return vale then the progtram will compile but you will get a warning about a potential unhandled `Err`. To get rid of the warning you can add error handling by either:-

1. Calling `expect()` which will cause the program to panic - this is gnerally discourgaed unless this is what you want the program to do.
2. Use pattern matching to handle any potential error gracefully - this is the preferred approach.

### `Ordering`

[Ordering](https://doc.rust-lang.org/std/cmp/enum.Ordering.html) is an enum type that has the variants `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values. The `cmp()` method compares two values (any two values that can be compared) and returns one of the variants 

```sh
use std::cmp::Ordering;

let adult_age = 18;
let her_age = 24;
match her_age.cmp(&adult_age) {
    Ordering::Less => println!("She is a child!"),
    Ordering::Equal | Ordering::Greater => println!("She is an adult"),
}
```


## Tools

### Evcxr Rust REPL

[evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md) is a Read-Eval-print loop for Rust.          
[evcxr usage guide](https://github.com/evcxr/evcxr/blob/main/COMMON.md).

Before you can use the REPL, you must download a local copy of Rust's source code:

```
$ rustup component add rust-src
```

Now you can go ahead and install the binary.

```
$ cargo install --locked evcxr_repl
```

And start the REPL.

```
$ evcxr  
Welcome to evcxr. For help, type :help
>> 
>> let x = 5;
>> let y = 10;

>> println!("x = {x}")
x = 5

>> println!("x + y = {} & x * y = {}", y + 2, x * y)
x + y = 12 & x * y = 50
```

Import a crate.

```sh
# import latest version of a crate
:dep rand  

   Compiling libc v0.2.164
   Compiling byteorder v1.5.0
   Compiling getrandom v0.2.15
   Compiling rand_core v0.6.4
   Compiling quote v1.0.37
   Compiling syn v2.0.87
   Compiling zerocopy v0.7.35
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5

# import specific version
:dep rand = { version = "0.8.5" } 
```

Define a function.

```sh
>> fn guess() -> u8 { 
    return rand::random();
}

>> println!("Random number: {}", guess())
Random number: 185
```

Import components with `use`

```sh
use std::cmp::Ordering;

let adult_age = 18;
let her_age = 24;

match her_age.cmp(&adult_age) {
    Ordering::Less => println!("She is a child!"),
    Ordering::Equal | Ordering::Greater => println!("She is an adult"),
}
```


## Resources

Some resources I have found a long the way that may be useful.

[The rustdoc book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)        
[The Cargo Book](https://doc.rust-lang.org/cargo/)           
[crates.io](https://crates.io/) - the official package registry for Rust.         
[The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) (a set of short notes on performance considerations)      



