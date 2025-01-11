

# Rust Guide


A summary of my Rust learnings largely guided by reading the offical Rust guides:-

[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)        
[The Rust Language Reference](https://doc.rust-lang.org/reference/introduction.html)         
[Rust by Practise](https://practice.course.rs/)       
[The rustdoc book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)        
[The Cargo Book](https://doc.rust-lang.org/cargo/)           
[crates.io](https://crates.io/) - the official package registry for Rust.         
[The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) (a set of short notes on performance considerations)     
[Best Practices for Packages with a Binary and a Library](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#best-practices-for-packages-with-a-binary-and-a-library)        


```rust
// key for code sample comments

✅     code compiles
❌     compiler error: code does not compile
⚠️     compiler warning: compiles but does not follow convention
```


---


<!--TOC-->


## TL;DR Summary

This section which summarises the most useful aspects of what I think I shall need to keep in mind when writing code. It should evolve with further understanding.

- [Keyword](https://doc.rust-lang.org/book/appendix-01-keywords.html) reference.
- [Operators & symbols](https://doc.rust-lang.org/book/appendix-02-operators.html) reference.
- Rust [naming conventions](https://rust-lang.github.io/api-guidelines/naming.html).
- Use `mut` to create a mutable variable.
- All variables have an owning scope and only the scope that has ownership can use them. You can however, create a **reference** to the variable (using the `&` prefix) which allows other scopes to borrow the variable without taking ownership.
- By default, references are immutable (readonly).
- You can create a mutable reference (`&mut` prefix) which allows another scope to use and modify a value. If you have a mutable reference then the compiler enforces that you can have no other references - this is to guard against data race conditions.
- If you pass a reference as an argument to a function then the parameter (in the function signature) must indicate that it is expecting a reference for an argument eg.`(s: &String)`
- The slice type is designed to reference to a contiguous sequence of elements in a collection eg. a substring.
- The type `&str` represents the String slice.
- If you have a function which takes a `&String` reference then it is more useful to define the parameter as a string slice `&str` because this parameter type can take both a `&String` and a `&Str`.
- Use an Array only if you know you are storing a fixed number of elements of the same type, otherwise use a Vector.
- Use a Tuple if you want to store a fixed number of elements of a different type.
- Use Structs to [group data that have a relationship](https://doc.rust-lang.org/book/ch05-02-example-structs.html#refactoring-with-structs-adding-more-meaning) - they can be either immutable or mutable.
- Use Tuple Structs if you just want a tuple (no field names) but you want the Tuple to have a type.
- Functions are not declared within a type, they are standalone.
- Methods are like functions but they are declared within a type (struct, enum or trait), within its `impl` block. They are invoked like `Rectangle.area()`. They must have a first parameter `self`, one of:-
    - `self` - method takes ownership (rare use cases). 
    - `&self` - method takes an immutable reference as the data is only being read.
    - `&mut self` - method takes a mutable reference as it is changing data.
- Associated functions are also declared within a type's `impl` block but they do not take a `self` argument and do not need an instance. They are like static methods in Java. They are invoked like `Rectangle::square(3)`
- Enums are very useful when you have a closed set of variants:-
    - You can store any type of data inside them even other enums.
    - You can define methods on enums.
    - [Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html) is a very useful enum in Rust and its useful to get to know all its methods.
- Enums have an advantage over structs in that each variant can store data fields of different types yet the enum variants themselves are all the same type - so you can write a function that takes the enum type and pass any of the variants in, yet they may contain different types of data.
- Using pattern `match` blocks when handling enums (eg. `Option<T>`) is a very common coding pattern in Rust.
- The `if let` syntax can be more concise than a regular match when you only need to match one variant.
- Use modules to organize code to make navigating it more intuitive and to provide a guide for the location of new code.
- All code within a module is private by default, but you can make it public using the `pub` keyword.
- If you want to make an item like a function or struct private, you put it in a module.
- Items in parent modules cannot use private items inside child modules, but items in child modules can use items in the ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.
- The module tree is just like how you organise files in a directory hierarchy - modules can have child modules and siblings.
- Modules can contain other modules (children), structs, enums, constants, traits, functions.
- When the compiler builds your project it starts with the crate root file (`src/main.rs` or `src/lib.rs`) and it builds the module tree. The contents of the crate root files form the root module called `crate`
- Paths are how you refer to constructs in your code (modules, structs, enums, functions).
    - Absolute paths begin with either the literal `crate` (when referring to current crate) or the crate name (external crates). These paths are generally preferred as they tend to reduce the cost of updation when moving code around.
    - Relative paths are just relative to the current code.
- You can also use `use` keyword to create shortcuts to items within a scope instead of having to keep writing the full path each time you reference it.
    - When using `use` to bring functions into scope, bring the parent module into scope so that you have to invoke the function on the parent module which makes it clear the function is not locally defined, eg. `some_module::some_function()`
    - When using `use` on anything other than functions the idiom is to bring the item fully into scope, eg. `some_module::SomeStruct`.
    - You can resolve name clashes when you import two items with the same name either by just bringing their parent module into scope (like with functions), or create a new name alias using `as` keyword.
- Structs can be marked public but their fields are still private by default and must be individually marked public as well if you wish to expose them - this is so you can hide implementation details of you wish.
- If an enum is marked public then all its variants are implicitly public.
- You can re-export items using `pub use` which makes them available to external code that would otherwise have no access.


---


## Programming Concepts

This section is an ad-hoc collection of features that were most useful to me learning Rust.

### Statements & Expressions

Rust is an expression-based language which means that virtually all of the code you write is an expression. So it is important to understand the distinction between statements and expressions.

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value. 

In some languages such as C and Ruby, an assignment returns the value of the assignment so in those languages they are expressions. Rust assignments do not return the value so they are statements.

### Variables & Constants

```rust
// immutable variables
let immutable_ref = "hello";
immutable_ref= "goodbye";       // ❌ error: cannot reassign immutable reference


// use the 'mut' keyword to create a mutable variable
let mut mutable_ref = "hello";
mutable_ref = "goodbye";       // ✅ this is fine since it's mutable


// naming conventions
let some_ref = "hello";    // ✅ Ok: adheres to naming conventions
let someRef = "hello";     // ⚠️ warning: should be snake case 'some_ref'
let some_Ref = "hello";    // ⚠️ warning: zhould be lowercase 'some_ref'


// CONSTANTS
// implicitly immutable, must have type annotation, can be declared in any scope
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  // ✅ Ok
const TEN_HOURS_IN_SECONDS = 60 * 60 * 10;        // ❌ error: must have type eg. i32
const mut TEN_HOURS_IN_SECONDS = 60 * 60 * 10;    // ❌ error: cannot be mutable
const NON_CONSTANT_VALUE: i32 = rand::random();   // ❌ error: value must be constant
const fiveHrsInSecs: u32 = 60 * 60 * 5;    // ⚠️ should be UPPER_SNAKE_CASE
```

### Crate imports (`use`)

`use` declarations are used for _import_ and _re-export_ of modules. Another way to describe them when importing is that they create a local name binding for some path, or, more simply, they are used to to shorten the path required to refer to a module item.

For full reference see [Rust Reference: Use](https://doc.rust-lang.org/reference/items/use-declarations.html).

```rust
// without imports we can refer to functions like so
let rand: i32 = rand::random();
let my_map: HashMap<i32, String> = std::collections::hash_map::HashMap::new();
```

With an import we can shorten the paths in our code.

```rust
use rand::random;
use std::collections::hash_map::HashMap;

let rand: i32 = random();
let my_map: HashMap<i32, String> = HashMap::new();
```

### References (`&`)

You can indicate that an argument to a function is a **reference** which allows multiple parts of your code to access the same piece of data without needing to copy that data into memory multiple times. 

- References are a relatively complex feature of Rust.
- Like variables, they are immutable by default so if you want a mutable reference then you need to use `&mut`

```rust
io::stdin().read_line(&guess)           // immutable reference
io::stdin().read_line(&mut guess)       // mutable reference
```

### Pattern matching to handle errors (`Option` & `Result`)

Operations that can fail will typically return an instance of [Result](https://doc.rust-lang.org/std/result/enum.Result.html) or [Option](https://doc.rust-lang.org/std/option/enum.Option.html).

```rust
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

```rust
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

```rust
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

```rust
let mut guess = String::new();

io::stdin().read_line(&mut guess).expect("Failed to read line");

# trim the original guess string and parse it into an u32 number type
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

What is interesting about shadowing is that we are essentially giving an immutable variable a new value by essentially creating a new variable which replaces the original one. We can apply transformations to a variable (change its value and even its type) but then the variable shall still be immutable once we have finished.

Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

```rust
let spaces = "   ";         # spaces is a string type
let spaces = spaces.len();  # now its a number type

let mut spaces = "   ";
spaces = spaces.len();      # compiler error - we tried to change the type

let mut spaces = spaces.len();  # this works though because we used let
```

### Loops

`loop` will create an infinite loop which will exit either when the program panics (because of some unhandled error), or when encountering a `break` statement.

```rust
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

### Printing & Debugging

The [println!](https://doc.rust-lang.org/std/macro.println.html) macro provides formatting and printing support and writes to the standard output stream (stdout). It has 3 formats:-

- `{}` - Display format.
- `{:?}` - Debug format.
- `{:#?}` - Dbug format pretty print.

The empty curly brackets (`{}`) tell println to use the default print format which is defined by an implementation of `std::fmt::Display` which many standard types implement. The empty brackets will work for all primitives and many standard library types.

```rust
println!("rect1 is {}", rect1);
```

With custom structs we either need to implement `Display` or use another format.

Another output format is `Debug` which is selected using the `:?` specifier inside the brackets. You need to explicitly opt into the debug functionality for your custom types however, using the outer attribute `#[derive(Debug)]` on your struct.

```rust
#[derive(Debug)]      // outer attribute to derive the Debug trait  
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

**dbg! macro**

Another way to print is the `dbg!` macro which can be really helpful when you’re trying to figure out what your code is doing.

- It takes ownership of an expression to do its work and returns ownership.
- It writes to the standard error stream (stderr).

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),    // result is same as 
        height: 50,                 // though dbg! was not there
    };

    dbg!(&rect1);  // pass reference so dbg does not take ownership
}
```

### Derivable Traits (derive)

You use the `derive` attribute to annotate your own structs or enums in order to add useful behaviour from a number of traits such as `Debug`.

Here are [the Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) that you can use with `derive`.

- Debug for debug formtting in format strings.
- PartialEq and Eq for equality.
- PartialOrd and Ord for ordering.
- Clone and Copy.
- Hash to map an instance from arbitrary to fixed size.
- Default for default values.

### Pattern Matching

[Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)

`match` is an extremely powerful control flow construct that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 

- Match arms have the following structure `pattern => code to execute`
- Each arm is executed in the order they are declared. If a match is found then that arm's code executes.
- The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.

Curly brackets are only required if the match arm code needs to be split across multiple lines, and in which case, the trailing comma is optional.

```rust
fn value_in_cents(coin: Coin) -> u8 {

    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }                    // the comma is optional if using braces
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

The Quarter match arm above demonstrates how to bind to values within the match pattern - the `state` variable will bind to the value of that quarter’s state. 

Combining match and enums is useful in many situations and is a common pattern in Rust code.

#### Using special pattern _

A match block's patterns must cover all possibilities - the compiler will catch any failures to exhaust all matches. 

It is possible to handle just a subset of possibilities explicitly and then have a final catchall match arm that catches anything else. You can use a variable if you want to capture state to use in the code or else use the special pattern `_` which just matches on anything but does not bind to any data.

```rust
// when you want to bind to values in the match 
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),   // 'other' is a variable which binds to whatever was matched
}

// when you dont want to bind to any state
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),              // _ will catch all other possibilities
}
```

#### Pattern Matching on Option<T>

Option has two variants - Some or None - and the common approach to handling an Option is with a match block.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

#### If Let

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle situations where you just want to match one pattern and ignore the rest. 

The following two expressions behave exactly the same.

```rust
// regular match expression which only executes for a certain match
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),      // boilerplate code which is annoying
}

// using if let is more concise
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

If you need exhaustive matching then use a regular match, if you only need to match 1 case and want concise code then use `if let` syntax.

You can also incorporate an `else` expression although whether this is more concise than a regular match depends on how complex and verbose your logic is.

### Structs

These are like Java classes. You could also think of them like Tuples, as a way to group values of different types, but the data is stored in fields (key-value pairs) where the key is the field name and the value is the data. 

- They can be immutable (`let inst = User { ... }`), or mutable (`let mut inst = User { ... }`).
- If its mutable then the entire instance must be mutable.

Rust has some sugar for creating instances.

```rust
// using Field Init Shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,    // we dont need to specify username: username
        email,
        sign_in_count: 1,
    }
}

// Using Struct Update Syntax when creating from existing instances
let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

When using Struct Update Syntax (`..`) any fields (in the original instance eg. `user1`) that are stored on the heap will get moved, not copied, to the new object which means those fields in the original object can no longer be used (this effectively makes the original object useless). If you provide values for any fields that would get moved, such as the email String in the above example, then these are not moved from the original object so leaving it still valid.

#### Tuple Structs

These are just tuples that have a type and are useful in these cases:-

- When you want to give a tuple a name/type. Using the below example, if a function takes a `Color `type parameter then you cannot pass an instance of `Point`, even though they both contain the same type/number of values.
- When naming each field as in a regular struct would be verbose or redundant.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- You can destucture tuple structs into their individual elements just as with regular tuples.
- You can also use `.[index]` to access the values.

#### Unit Structs

It is possible to declare structs that do not contain any fields called _unit-like structs_ because they behave similarly to `()`.` They can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;
```

#### Struct data ownership and Lifetimes

You can define structs with fields which are owned types, eg. the String type (which is owned by its scope) rather than the string slice type `&str` which is a reference. When using the String type, each instance of the struct will own all of its data and that data will be valid for as long as the entire struct is valid.

It is also possible to define structs with references such as the string slice, in which case the actual String may be owned by something other than the struct instance. In order to do this you need to use Lifetimes.


### Functions & Methods

- Methods are basically functions defined within a struct, enum or trait. They have a first parameter called `self` which represents the instance of the struct they are being called on. They typically work on the data of the struct or enum instance.
- Functions are similar to methods except they stand alone and are not tied to a specific instance of a struct or enum.

#### Functions

- Rust uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
- Function bodies are made up of a series of statements optionally ending in an expression.
- If a function has a return value, it is the result of the final expression in the function body. Or else you can return a value early using the **return** keyword.
- If a line of code ends with a semi-colon it is a statement not an expression, so the final line of the function body should not have a semi-colon if it the function return value.
- We don’t name return values, but we must declare their type after an arrow (->).

```rust
fn five() -> i32 {
    32
}
```

#### Associated functions (`::`)

An informative [Reddit](https://www.reddit.com/r/rust/comments/3fimgp/comment/ctqfg33/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) post [comments](https://www.reddit.com/r/rust/comments/3fimgp/comment/idc4hlr/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) about [what :: does](https://www.reddit.com/r/rust/comments/3fimgp/comment/idc5wsy/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button).   

- [Associated Functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions) are so called because they are _associated_ with a type, by being declared within the type's impl block, but they are not methods, so do not take a self reference and do not need an instance of the type on which to be invoked.
- They are like static methods in other languages.
- They are often used as constructors for creating new instances and can be called new but note that new is not a special keyword in Rust so these constructors can be called whatever you like.

In contrast, **methods** can only be invoked on an instance of a type and they use the dot `.` operator

- `::` calls functions associated with a type.
- `.` calls methods on a specific instance ie. instance methods.

```rust
impl Rectangle {

    // Associated function
    // The Self keywords in the return type and in the body of the function are aliases 
    // for the type that appears after the impl keyword, which in this case is Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Method - it takes a self
    fn area(&self) -> u32 {       
        self.width * self.height  
    }
}

// calling associated functions
let sq = Rectangle::square(3);

// associated from() function is like static factory in Java
let mut s = String::from("hello");

// calling methods - these need an instance of a string to work
let length = s.len(); 
let trimmed = s.trim();
```

#### Methods

The main reason for using methods instead of functions is for organization. You generally put all the things you can do with an instance of a type in methods. You could define the same behaviour in functions outside of the type but by grouping all the type's behaviour within its `impl` block, any users of our code do not need to go searching for these capabilities in various places.

You define methods on a struct within an `impl` (implementation block). 

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {       // this example passes a reference &self as we
        self.width * self.height  // are only reading data so dont need ownership
    }
}
```

There are 3 options for how the method want to use `self`:-

- **Borrow immutably** by passing a reference `&self` - where we just need to read the data.
- **Borrow mutably** by using `&mut self` - for cases where the method needs to change the data.
- **Take ownership** by using just `self` - this is rare and only for cases where the method tranforms `self` into something else and you want to prevent the caller from using the original instance after transformation.

Rust has automatic referencing and dereferencing - [read this](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator) to understand what this means.


### Enums

Enums are types in Rust just like struts.

You can define just a set of variants like so:-

```rust
enum IpAddrKind {
  V4,
  V6,
}
```

But you can also store data inside instances of the Enum:-

```rust
enum IpAddr {
  V4(String),
  V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

For each variant of the enum we automatically get a constructor defined eg. `IpAddr::V4()` is a constructor which takes a String and creates a new instance of IpAddr.

Enums also have one advantage over structs in that each variant can have different types and number of data fields. In this example, the V4 variant stores the address as four u8 values, whereas V6 is stored as a String.

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Another advantage over structs is that if say V4 and V6 were represented as structs, you wouldn't be able to define a function that could take either. But since V4 and V6 are both types of IpAddr you can define such a function.

Enum variant data be stored as anything, even structs or other enums. The [standard library definition](https://doc.rust-lang.org/stable/std/net/enum.IpAddr.html) stores each variant as a struct.

You can also define methods on Enums.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {

  fn call(&self) {
    // method body would be defined here
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The `call()` method uses `&self` to get the value that it was invoked on, in this example a String.

An important enum in the standard library is Option which replaces the value null - which Rust does not have and which therefore makes Rust safer. It either has value or it doesn't and is so widely used that it is included in the prelude so you don't ever need to import it.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

In Rust, you should safely be able to assume that any value that has a type other than Option will never be null.

You have to convert an Option to a T before you can perform T operations with it. [Option has a large number of methods](https://doc.rust-lang.org/stable/std/option/enum.Option.html) that allow you to work with the T in a variety of situations. Becoming familiar with these methods is a cornerstone of becoming proficient in Rust.

In order to use an `Option<T>` value, you want to have code that will handle each variant - code that only runs when you have a `Some(T)` value and code that only runs when you have a `None value`. The **match** expression is a control flow construct that does just this when used with enums.


### Notable Types

An adhoc list of common types I have come across.

#### Unit

Unit is a tuple without any values. 

- This value and its corresponding type are both written `()` and represent an empty value or an empty return type. 
- Expressions implicitly return the unit value if they don’t return any other value.
- One practical use of Unit is when we don't care about a generic type, and () makes this explicit. For example, a `Result<(), String>`` can be used as return type for a function that either completes successfully or fails for a variety of reasons.

#### Array

Arrays are another way to group multiple elements.

> The Vector type, unlike Array, is a collection that is allowed to grow or shrink in size. Unless you need to ensure your collection resides on the stack and has a fixed size, you should use Vector over Array.

- All elements must be of the same type - Rust arrays are homogeneous.
- The size must be known at compile time.
- The type must have a known size at compile time.

Arrays are useful:-

- When you want your data allocated on the stack, rather than the heap.
- When you want to ensure you always have a fixed number of elements ie. you know the number of elements will not need to change eg. you are storing reference data like the months of the year.

```rust
// some example arrays

// Primitive types
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let booleans: [bool; 3] = [true, false, true];

// Custom struct
struct Point {
    x: i32,
    y: i32,
}

let points: [Point; 2] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 1 }
];

// Enums
enum Color {
    Red,
    Green,
    Blue,
}

let colors: [Color; 3] = [Color::Red, Color::Green, Color::Blue];
```

In Rust, arrays are always stack-allocated by default, including when they contain custom types like structs. The only time that data in an array is stored on the heap is if any of the data is heap-allocated. The following array will be on the stack but will contain pointers to the String data that will reside on the heap.

```rust
struct Point {
    x: i32,        // stack-allocated
    y: i32,        // stack-allocated
    label: String  // heap-allocated
}


// all the following is on the stack except the Strings.
let points: [Point; 2] = [
    Point { x: 0, y: 0, label: String::from("Origin") },
    Point { x: 1, y: 1, label: String::from("Point 1") }
];
```

Note that the array's memory layout and size is still fixed and known at compile time.

```rust
let arr: [String; 2] = [String::from("hello"), String::from("world")];

// What's on the stack:
// - The array container itself
// - Two String structs, each containing:
//   - A pointer to heap data
//   - A length
//   - A capacity

// What's on the heap:
// - The actual string data "hello"
// - The actual string data "world"
```

Each of the above Strings stored on the stack contain the following three components - which are collectively known as the _"fat pointer"_:-

1. A pointer to heap memory (where the actual characters are stored).
2. A length field.
3. A capacity field.

#### Tuple

Compound types group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

- Tuple is a general way of grouping together a number of values that can have different types.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- You create them using parenthesis `let tup = (1, 2, 3, 4, 5)`
- We can access a tuple element directly by using its index. Tuples, like arrays, are zero indexed.
- To get the individual values out of a tuple you can destructure it using pattern matching like below.

```rust
let tup = (500, 6.4, 1);

// destructure the tuple
let (x, y, z) = tup;

// accessing a value by index
let first_element = tup.0;  // 500
```

The memory storage behaviour of tuples follows the same principles as arrays. The tuples themselves are stored on the stack, but if they contain types that use heap allocation, those values will be stored on the heap and a "fat pointer" to each heap-allocated value is stored in the tuple.

```rust
// All stack-allocated - everything stored on stack
let tuple1: (i32, bool, char) = (42, true, 'a');

// Mixed storage
let tuple2: (String, i32, String) = (
    String::from("hello"),  // String data on heap
    42,                     // i32 on stack
    String::from("world")   // String data on heap
);

// Example with custom types
struct Point {
    x: i32,
    y: i32
}

// All stack-allocated since Point has no heap data
let tuple3: (Point, i32) = (Point { x: 1, y: 2 }, 42);

// Tuple containing Vec (heap-allocated collection)
let tuple4: (Vec<i32>, bool) = (vec![1, 2, 3], true);
// - Tuple and Vec struct (pointer, length, capacity) on stack
// - Actual vector contents on heap
```

#### Result

[Result](https://doc.rust-lang.org/std/result/enum.Result.html) is a common return type of functions since it represents either a success or failure . Result is an [enumeration](https://doc.rust-lang.org/book/ch06-00-enums.html) (or enum) which is a type with a fixed set of states, which are commonly referrd to as _variants_, and `Result`'s variants are `Ok` and `Err`.

`Ok` variant represents a successful operation and wraps the successfully generated value.        
`Err` variant means the operation failed, and Err contains information about how or why the operation failed.

**expect()**

Result has an [expect(](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)) method which you can call:-

- If the instance of Result is an `Ok` value then expect() will return the value so you can use it.
- If the Result is an `Err` value then expect() will crash the program and display the message passed to it.

When a method returns an instance of Result and you do not use that return vale then the progtram will compile but you will get a warning about a potential unhandled `Err`. To get rid of the warning you can add error handling by either:-

1. Calling `expect()` which will cause the program to panic - this is gnerally discourgaed unless this is what you want the program to do.
2. Use pattern matching to handle any potential error gracefully - this is the preferred approach.

#### Ordering

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


## Common Collections

Arrays and Tuples are collections that are stored on the stack and their memory requirements must be known at compile time. There are other collection types which are stored on the heap and which can grow or shrink dynamically at runtime.

### String

[Guide to Working with Strings in Rust](https://dev.to/alexmercedcoder/in-depth-guide-to-working-with-strings-in-rust-1522)      
See [String vs &str](https://www.reddit.com/r/rust/comments/1695k03/string_vs_str/)     
String [ASCII](https://gist.github.com/jonjack/76ae94ad83c07ddb1cd2ee286f69e564)     

Not usually considered a collection but a string is actually a collection of UTF8-encoded character bytes, and they are a more complicated data structure than many programmers appreciate.

Different languages make different choices for handling strings. Rust has chosen to treat all strings as UTF-8 which means programmers have to put more thought into handling UTF-8 data up front. This trade-off exposes more of the complexity of strings than is not apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

#### There are two string types in Rust

1. The string slice `str` which is part of the core language. They are immutable with a fied length so once created they cannot change. It is only really useful in its borrowed form `&str`. See [String Slice](#) section below.
2. The `String` type which is part of the standard library (not core language) and is a growable, mutable, owned, UTF-8 encoded string type.
    - They are actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. So many of the operations of `String` are also available with vector (`Vec[T]`).
    
#### How Strings are stored

[Internal Representation](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#internal-representation)

String is a wrapper over [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)<[u8](https://doc.rust-lang.org/std/primitive.u8.html)\>.

Consider the following examples of valid UTF-8 strings. The first string only takes 4 bytes to encode it in UTF-8 whilst the second string takes 24 bytes - the reason for the difference comes down to understanding [UTF-8 character encoding](https://blog.hubspot.com/website/what-is-utf-8). Each Unicode scalar value in the second string takes 2 bytes of storage.

```rust
let hello = String::from("Hola");           // 4 bytes long
let hello = String::from("Здравствуйте");   // 24 bytes long
```

#### Bytes and Scalar Values and Grapheme Clusters

Another thing to consider is that there are three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

[See more details here](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

Note that getting grapheme clusters from strings is complex and not covered by the standard library so you have to use supporting crates.

#### Indexing

Rust does not support indexing (eg. `some_string[0]`) on strings like some other languages for a couple of reasons. Strings in Rust are always UTF-8 encoded (see [How Strings are stored](https://null)) and Indexes usually correlate to a byte of storage. This UTF-8 string `Здравствуйте` (for example) takes 24 bytes to store it (2 bytes per character) so an index into this string's bytes will not always correlate to a character.

Consider also that Rust provides different ways of interpreting string data (Bytes, Scalar Values, Grapheme Clusters) so indexing will also not necessarily make sense depending on how you are working with the data.

Also, indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

To remove the risk of returning unexpected values, the Rust compiler will fail any code which attempts to index into a string.

#### Warning about String Slices

Even though indexing into strings is not supported in Rust, you do use a range of indexes when creating a string slice - which map to bytes not characters. In the following string slice example, `s` will be equal to `Зд` because four bytes in this string maps to only two characters (first character is actually capital Cyrillic letter Ze not 3).

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

This is important to remember because the compiler will not stop you if you slice only part of a character’s bytes and this can crash your program, eg. something like `&hello[0..1]` would cause Rust to panic at runtime in the same way as if an invalid index were accessed in a vector.
    
#### Creating strings

```rust
// create a new empty String
let mut s = String::new();

// create a String with some data
// the following 3 approaches are all equivalent
let contents = "Some contents";
let s = data.to_string();

let s = "Some contents".to_string();

let s = String::from("Some contents");
```

They are UTF-8 encoded so you can use any properly encoded data:-

```rust
let hello = String::from("Hello");
let hello = String::from("こんにちは");
let hello = String::from("Здравствуйте");
```

#### Modifying strings

```rust
// append a string to an existing string with 'push_str'
let mut s = String::from("foo");
s.push_str("bar");

// append a single character with 'push'
s.push('l');
```

#### Concatenation

[Concatenation with + Operator or format! Macro](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)

The `+` operator uses the `add(self, s: &str) -> String` method behind the scenes which is invoked on an owned string but takes a string reference (slice) as an argument. So the first string gets moved, and the second string does not as it is a reference. We are able to pass an instance of the `String` type to the method which takes a reference `&str` because the compiler is able to coerce it using a _deref coercion_.

```rust
// + operator
let s1 = String::from("Tic, ");
let s2 = String::from("Tac");

let s = s1 + &s2; // note s1 has been moved here and can no longer be used
```

You can use `+` to join multiple strings but it gets a bit messy to read so you can use the `format!` macro instead which works like `println!`, but instead of printing the output to the screen, it returns a String with the contents.

```rust
let s3 = String::from("Toe");

// using + for multiple strings gets a bit messy
let s = s1 + "-" + &s2 + "-" + &s3;

// format! macro
let s = format!("{s1}-{s2}-{s3}");
```

#### Iterating over Strings

The safest approach for operating on strings is to be explicit about whether you want characters or bytes.

```rust
// use chars method if you want characters
for c in "Зд".chars() {    
    println!("{c}");
}   

// will print chars 
Зд

// if you need bytes use bytes method
for b in "Зд".bytes() {    
    println!("{b}");
}

// will print
208
151
208
180
```

### Vectors

The [vector](https://doc.rust-lang.org/std/vec/struct.Vec.html) is a generic collection with type `Vec` and has the following characteristics:-

- You can store multiple values of any type but only of the same type.
- All values will be stored next to each other in memory ie. contiguous.
- When a vector goes out of scope it is dropped and so are all its elements.
    

They are useful when you have lists of items such as lines of text in a file or items in a shopping cart.

```rust
// Creating vectors

// type <i32> annotated because it is empty so we tell Rust what we intend to store 
let v: Vec<i32> = Vec::new();

// Rust can infer the type if you create it with data
// the vec! macro is useful for creating vectors with initial values
// this will have type <i32> as that is default integer type
let v = vec![1, 2, 3];
```

They can be mutable and can be updated using the `push` method.

```rust
// Updating vectors

// we need to use mut if we want to update it
let mut v = Vec::new();

// update using the push method
v.push(5);
v.push(6);
```

There are two ways of referencing a value stored in a vector - by indexing or using the `get` method.

```rust
// Referening items in vectors

let v = vec![1, 2, 3, 4, 5];

// vector indexes start at zero
// & gives us a reference to the element
let third: &i32 = &v[2];

println!("The third element is {third}");

// get method gives us an option containing a reference
let third: Option<&i32> = v.get(2);

match third {    
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

Which method you use depends on how you want your program to behave if it references a non-existent element.

- Use an index if you want your program to crash when accessing an out of bounds index. 
- Use `get` if accessing an element beyond the range of the vector may happen occasionally under normal circumstances eg. the index is driven by manual input - in which case you will have logic to handle the `None` case.

```rust
// Panicking

let v = vec![1, 2, 3, 4, 5];

// this will panic
let does_not_exist = &v[100];

// this does not panic - it returns a None
let does_not_exist = v.get(100);
```

#### Vectors and Ownership

Note that the borrow checker enforces the ownership and borrowing rules to ensure that any references to the contents of the vector remain valid. Recall the rule that states you can’t have mutable and immutable references in the same scope.

The following will not compile.

```rust
// Ownership

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];  // & reference is an immutable borrow 

v.push(6);   // an update is a mutable borrow

println!("The first element is: {first}");  // reference to first borrow
```

Why should the reference to the `first` reference care about changes to the end of the vector caused by the `push`?

Since vector elements are stored in blocks of contiguous memory, an update might require the allocation of a new memory block somewhere else and the copying of the old elements to the new space. In such an event the reference to the first element would be pointing to deallocated memory. The borrowing rules guard against creating this situation.

When a vector gets dropped (when it goes out of scope) all its contents are also dropped. The borrow checker ensures that any references to the contents of a vector are only used while the vector itself is valid.

#### Iterating over vectors

The for loop gets immutable references to each element in a `Vec`.

```rust
// Iterating

let v = vec![100, 32, 57];

for i in &v {    
    println!("{i}");
}
```

If we want to make changes to elements in a mutable vector we need to get a mutable reference to each one and then use the `\\*` dereference operator to get to the value in `i` before we can use the `+=` operator.

```rust
// Iterating & updating

let mut v = vec![100, 32, 57];

for i in &mut v {    
    *i += 50;
}
```

Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules. If we attempted to insert or remove items in the for loops then this would not compile since the reference that the `for` loop holds prevents simultaneous modification of the whole vector.

#### Using Enum to store multiple values

Vectors can only hold values of the same type and this can be inconvenient in some situations. As a workaround you can store enum variants which can be of different types since as far as the vector is concerned it is just storing the same enum type.

[See the book for an example](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types)


### Hash Maps

The hash map type `HashMap` stores a mapping of keys of type `K` to values of type `V` using a hashing function, which determines how it distributes these keys and values in memory.

They are useful when you want to store and then look up a value based on its key that you control, rather than an index. There are many functions defined on [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) for use in varying situations.

- Like Strings and Vectors they are stored on the heap.
- Like vectors, they are homogeneous - so all the keys must be of the same type and all the values must be of the same type. The key and value can be different types however.
    
Since HashMap is not used as commonly as String and Vector it is not brought into scope automatically in the prelude and so we have to `use` is from the collections part of the standard library.

```rust
use std::collections::HashMap;

// create a hashmap using new and insert elements
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Green"), 20);
```

You retrieve values using the `get` method which takes a key.

The get method returns an `Option<&V>`. In the following example we use `copied` to get an `Option` rather than an `Option<&i32>` and then `unwrap_or` to set score to zero if the Option is a `None`.

```rust
// Get method

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

You can iterate over each key-value with a for loop.

```rust
// Iterating

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

#### HashMaps and ownership

When you add items to a hash map the ownership rules apply:-

- For anything that is stored on the stack (implements `Copy`) trait, like `i32`, the value gets copied into the hash map.
- For anything that is owned (stored on the heap), like `String`, the values will be moved and the hash map becomes the owner.

```rust
let field_name = String::from("Hello");
let field_value = String::from("World");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value can no longer be accessed
```

You can insert references to values into the hash map and the values shall not be moved. The compiler shall enforce that the values the references point to are valid for as long as the hash map is.

#### Updating a Hash Map

Each key is unique so you have to decide how to handle the case whereby a key already exists and refers to some value. You can replace the old value, only update if the key does not have a value, or combine the two values.

```rust
// Updating an existing key/value

use std::collections::HashMap;

let mut scores = HashMap::new();

// overwriting 
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

To update the key with a value only if it has no current value there is a special API called `entry` which returns an enum of type `Entry` which may or may not have a value. The `Enum.or_insert()` method returns a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the argument as the new value. 

Using `entry` is much neater than writing the logic ourselves.

```rust
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```

A common use case is to update a value based on the current value eg. we are keeping scores or keeping track of something. In the following example we keep count of how many times each word appears in a sentence. We check if each word already exists in the map or not, increment its count if its already there.

- `split_whitespace()` method returns an iterator over subslices, separated by whitespace, of the value in text.
- `or_insert` returns a mutable reference (`&mut V`) to the value for the specified key which is then stored in the `count` variable.
- In order to assign to that value, we must first dereference `count` using `*`.


```rust
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);  // if no word insert zero
    *count += 1;   // now increment by 1
}
```

Note that iteration of a hash map happens in an arbitrary order.

#### The hashing function

The default hashing function is called _SipHash_ which provides resistance to denial-of-service (DoS) attacks involving hash tables. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different _hasher_.


---


## Memory management (Ownership)

[What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)       
[Memory Allocations in Rust](https://dev.to/gritmax/memory-allocations-in-rust-3m7l)      

Rust does not use a garbage collector (Java, Python, Ruby, Javascript) nor does it depend on the programmer to explicitly allocate and release memory (C, C++, Pascal). It uses a set of rules that the compiler checks and if any are violated, the program won’t compile. These rules are the concept of **ownership** whereby every value in Rust is owned, and can only be owned, by a single scope. Once that scope ends the value is dropped from memory. 

- Any value whose memory requirements are unknown at compile time, must have memory allocated at **runtime** when we instantiate the data in our code. When a value goes out of scope, Rust calls a `drop` function which frees up the memory. Custom types can implement drop.
- Any type stored on the stack implements a `Copy` trait - these values do not move, but rather are trivially copied, making them still valid after assignment to another variable.
- Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.

### Variable Scope

Every variable is owned by a single scope at any one time and it can only be used by that scope. 

A variable can **move** to another scope - it still exists in memory but it can no longer be used by the current scope.    
   
```rust
fn main() {

    let s1 = String::from("hello");
    println!("s1 = {s1}");  // 1st print: Ok
    
    some_func(s1);  // ownership moved to some_func scope
    println!("s1 = {s1}");  // 2nd print: compiler error!!
}

fn some_func(s1: String) {
    println!("some_func() now owns s1: {s1}");
}
```

A variable, and its data, will be dropped from memory when its current scope ends.

### Reassignment and copying

For anything stored on the stack, when you bind a variable to another variable, the value is simply copied. In the following example, `y` will have it's own copy of 4.

```rust
// integers are stored on the stack
let x = 4;
let y = x;   // x's value is copied and assigned to y
 
println!("x = {x}, y = {y}");   // prints x = 4, y = 4
```

But for anything stored on the heap, like String, something different happens. In the following example, we could have `s1` and `s2` variables (on the stack) both pointing to the same memory address for the "hello" string on the heap.

```rust
let s1 = String::from("hello");
let s2 = s1;   // scope of s1 ends here and it is dropped from memory

println!("{s1}, world!");  // this would cause a compiler error
```

The term _shallow copy_ in programming languages refers to the copying of data on the stack which points to a shared memory address on the heap ie. nothing is copied on the heap. When Rust invalidates **s1** though, it is not really a shallow copy since s1 itself becomes invalid, so what we say instead is that s1 was **moved** to s2. This is how Rust solves the double free up (memory) problem since now it only needs to free the memory when **s2** goes out of scope.

A _deep copy_ is where data on the heap is copied, which could potentially be an expensive operation. Rust will never automatically create any deep copies.

If you want to create a deep copy you can use `clone()` but keep in mind that it can potentially be an expensive operation.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // s1 is still valid since it gets copied not moved

println!("{s1}, world!");  // this works because s1 is still valid
```

### Ownership & Functions

When you reassign a variable it gets moved or copied depending on where its stored. When you pass a variable to a function the same thing happens. 

- **Heap data** - If you pass a variable, whose data is stored on the heap, to a function, then the function takes ownership and it becomes invalid in its current scope. 
- **Stack data** - If you pass a variable stored on the stack, to a function, then a copy is passed, and it remains valid in its current scope. 

The following code does not compile because we try and use s1 in the current scope after we have passed ownerhip to a function.

```rust
fn main() {
    let s1 = String::from("Hello");
    let len = length(s1);  // ownership of s1 gets moved to the function's scope

    println!("s1 = {s1}");  // compile error because s1 is no longer owned here
}

fn length(s: String) -> (usize) {
    s.len()
}
```

### References & Borrowing

[References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)

Rust has a feature which allows another part of your program to "borrow" a value which means it does not own it but it can use it. To do this you create a **reference** to the variable by prefixing it with an ampersand `&` (immutable reference) or an `&mut` (mutable reference).

- You can have multiple immutable references to a value because they are readonly.
- You can only have a single mutable reference (to guard against race conditions).
- All these ownership rules are checked at compile time.

### Immutable References (`&`)

The following code compiles because instead of passing **s1** to the function (which would move ownership), we pass a reference **&s1** which allows the function to borrow the variable.

```rust
fn main() {
    let s1 = String::from("Hello");
    let len = length(&s1); // we pass a reference & to s1

    println!("s1 = {s1}"); // this works because the current scope still owns s1
}

fn length(s: &String) -> (usize) {  // the function must also take a &reference type
    s.len()
}
```

### Mutable References (`&mut`)

You can modify a reference if the variable it points to is mutable.

```rust
fn main() {
    let mut s1 = String::from("Hello");  // change s1 to be mut
    let len = change(&mut s1);  // the reference must be &mut

    println!("s1 = {s1}"); 
}

fn change(s: &mut String) { // the parameter type must be &mut
    s.push_str(" World");
}
```

[Mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references) have a restriction in that if you have a mutable reference to a value, you can have no other references to that value. The benefit of having this restriction is that (at compile time) Rust can prevent potential data race conditions. 

- You cannot have more than one mutable reference at the same time.
- You can have multiple immutable references at the same time because if you are only reading the data and not modifying it then there are no risks.


### Slice Type

[The Slice Type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html#the-slice-type) allows you to reference a partial (contiguous) section of a collection rather than the collection as a whole.

- It can be used with different types of collections but is commonly used with strings to refer to a substring (the string slice type `str`).
- A slice is a kind of reference so it never has ownership.
- A slice can refer to the whole string/collection - it does not have to refer to just a part.
- A slice stores a reference to its first element and a length.
    
#### The issue that slice solves

Say you want to find and refer to the first word in a string which contains one or more words. You could convert the string to an array of bytes and search through it until you find a byte which represents a space and then save the index to a variable `endIdx`. With the start and end indexes you now have a reference to this contiguous sequence of characters. The problem is, if the string is mutable and gets modified you cannot be certain that your end index is still accurate as it is just a number and is not actually tied in any way to the string.

A slice fixes that because it is an (immutable) reference to part of the string/collection, and therefore, if the compiler finds that we have a slice (a reference) to the collection in our code anywhere after another piece of code attempts to modify it then compilation fails. So the slice gives us memory safety.

``` rust
let s = String::from("hello world");

// create slices
let hello = &s[0..5];
let world = &s[6..11];

// if your slice range begins at 0 you can omit the first index
let slice= &s[..5];

// if your slice references the last element you can omit the trailing index
let slice= &s[6..];

// if the slice is the complete string then you can omit both start and end
let slice= &s[..];
```

#### String slice (`&str`)

They are an immutable reference to a sequence of UTF-8 characters. 

- Use when you want to pass a reference to an existing string without taking ownership.
- Use when you do not need to modify the data - they are immutable and have a fixed length. 
- String literals are inherently of type `&str`.

**String slices contain two components:-**

1. A pointer to some text in memory.
2. The length of the slice.

The slice struct itself (the pointer and length) is stored on the stack, but the actual string data that it refers to is stored elsewhere - typically in one of these locations:

- For string literals (like `let s = "hello"``), the actual text is stored in the program's read-only memory segment.
- For slices of `String`s, the text data is stored on the heap
- For slices of other string slices, it points to wherever the original data is stored

```rust
let text = "hello"; // The string slice struct is on the stack,
                    // but "hello" is in read-only memory of the binary

let owned = String::from("world"); // "world" is on the heap
let slice = &owned[..]; // slice struct on stack, still points to heap data
```

**They are immutable**

Once created, you cannot modify either the content or the length of a string slice. This is part of Rust's memory safety guarantees.

```rust
let s = String::from("hello world");
let slice = &s[0..5]; // Creates a slice containing "hello"

// You cannot do any of these:
// slice.push('a');     // Won't compile - no method to modify contents
// slice.extend("...");  // Won't compile - slice is immutable
// slice = &s[0..7];    // Won't compile - if slice is not declared as mut
```

If you need a modifiable string, you should use a `String` type instead. You can always create a new string slice with a different length by re-slicing either a `String` or another string slice, but the original slice itself cannot be changed.

#### String Slices as Parameters

If you have a function that takes a string reference `&String` then it is more useful to write the signature such that it takes a String slice (`&str`) rather than a reference (`&String`) because you can pass either a slice or a reference as an argument.

```rust 
fn some_func(s: &String) -> &str {  // must pass a reference &String only

fn some_func(s: &str) -> &str {   // pass either a slice &str or reference &String
```

#### General slice type

In addition to the string slice there is a more general form which you can use for all types of collections.

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the same way as string slices do, by storing a reference to the first element and a length.


---


## Organising Projects

As a project grows, you should organize code by splitting it into multiple modules and then multiple files. 

Rust has a number of features collectively referred to as the _module system_ that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs.

- **Packages**: A Cargo feature that lets you build, test, and share crates.
- **Crates**: A tree of modules that produces a library or executable.
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths.
- **Paths**: A way of naming an item, such as a struct, function, or module.

### Scope

The nested context in which code is written has a set of names of variables, constants, functions, structs, enums, modules that are deemed "in scope" ie. available for use. The module sytem provides a way to control which names are in scope.

### Encapsulation

The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. Other code can call your code via its public interface without having to know how the implementation works.

### Crates

A crate is the smallest amount of code that the Rust compiler considers at a time and can be as simple as a single file. Crates can contain modules which may be defined in separate files.

There are 2 types of crates:-

1. **Binary** - programs you can compile to an executable and then run. They must have a function called `main` which is the entrypoint to the program.

2. **Library** - these creates provide code that is intended to be used by other programs. They do not have a `main` function and cannot be run. Since Rust programs can be compiled for a large number of target architectures, library crates are not distributed in binary form but in source code so that they get compiled to the target architecture of each application they are used in. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library”.

The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate.

### Compilation starts at the crate root

When compiling a crate, the compiler first looks in the crate root file for code to compile.

Crate root is usually:-

- `src/lib.rs` for a library crate.
- `src/main.rs` for a binary crate.

### Packages

A package is a bundle of one or more crates and a **Cargo.toml** file that describes how to build those crates. 

- A package can contain as many binary crates as you like, but at most only one library crate. 
- A package must contain at least one crate, whether that’s a library or binary crate.

### Modules

Modules let us organize code within a crate for readability and easy reuse and allow us to control access.

- You define a module with `mod [module_name] { ... }` and its body is within the curly braces.
- Modules can contain - other modules, structs, enums, constrants, traits, functions.
- By grouping related code togethor and providing the context with a meaningful name, programmers can navigate the code more intuitively and will have a good idea where to place new code which helps keep the program organized.
- For details of how to declare modules and submodules see [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet).

#### Module Tree

When the compiler begins to compile your crate it begins with the crate root files _src/main.rs_ (binary crates) and `src/lib.rs` (library crates). They are called the crate root because their contents form a module called `crate` which sits at the root of the crate's module structure, what we call the _module tree_.

This is why module paths start with `crate::[some_module]::[some_sub_module]::...`

#### Privacy

- Modules are Rust's facility for making items private and controlling access to them. 
- Private items are implementation details that we have decided are internal and should not be available for outside use.
- If you want to make an item like a function or struct private, you hide it in a module.
- We can choose to make modules and the items within them public (using `pub mod`), which exposes them to allow external code to use and depend on them.
- Items in parent modules cannot use items inside child modules if they are private, but items in child modules can use items in the ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.

Note that just by making a module public does not expose its contents it only allows other code to reach the module itself. Since modules are just containers, there is not much point just making a module public. To make a function inside a module accessible (public) we have to do two things:-

- Make the **module** public (`pub mod`) - this allows other code to reach the module.
- Make the **function** public (`pub fn`) - this allows other coe to reach the function inside the public module.

Consider the following example of code defined in the crate root file. Although `routes` is not public, `run_checks()` can access it since it is defined in the same file so is a sibling. And since `health-route` and `run_checks()` are public they are reachable also.

```rust
// Library crate root file - src/lib.rs

mod routes {
    pub mod health_route {
        pub fn check_health() {}
    }
}

pub fn run_checks() {

    // Absolute path
    crate::routes::health_route::check_health();

    // Relative path
    routes::health_route::check_health();
}
```

#### Making Structs and Enums Public

You can use `pub` with structs and enums but there are a few more details to consider.

- You can mark a struct public but its fields remain private - you have to explicitly mark each field public as well. This is because structs can still be useful if fields are private because you may wish to hide some/all of the implementation details and only expose methods.
- Public fields can be accessed with dot (`.`) notation.
- If a struct has just 1 private field then you have to provide a public constructor function (in `impl`) which creates new instances.

```rust
// Library crate root file - src/lib.rs

mod routes {

    pub struct Route {
        pub path: String,
        protocol: String,

    }
    
    impl Route {
        pub fn new_route(name: &str) -> Route {
            Route {
                path: String::from(name),
                protocol: String::from("https://"),
            }
        }
    }
}

pub fn create_routes() {

    let mut route = routes::Route::new_route("home/")
    // change the path
    route.path = String::from("remote/")
}
```

**Enums** are simpler than structs because they are not very useful unless their variants are public - so just by marking the enum public, all of the variants are automatically public.

#### Sample module structure

The following is a simple example project structure.

- The compiler starts with the crate root files.
- You typically declare modules in the create root files.
- You typically declare submodules in the parent module source file eg. you would declare `mod my_sub_module` in `my_module.rs`

```rust
my_crate
    │
    ├── Cargo.lock
    ├── Cargo.toml
    │
    └── src
        │
        ├── lib.rs   // crate root file for library crate
        ├── main.rs  // crate root file for binary crate
        │
        ├── my_module.rs
        └── my_module
                │
                └── my_sub_module.rs
```

You can put code into the crate root files main.rs and lib.rs if you so wish.     
Consider that the following two examples have the same result.

```rust
// src/lib.rs
mod foo {
    fn test() {}
}

//---------------------------

// src/lib.rs
mod foo;

// src/foo.rs
fn test() {}
```

One approach is to configure all your modules in a single place ie. in either **main.rs** or **lib.rs**, for example.

```rust
// in either main.rs or lib.rs
mod config;
mod routes {
    mod health_route;
    mod user_route;
}
mod models {
    mod user_model;
}
```

#### Separating Modules into different files

[Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)

You could theoretically put all modules and code into the crate root file (main.rs or lib.rs) but this would get a bit large for anything but the trivial application.

Wherever you declare a module, its location in terms of the filesystem is relative to that. The following is an example of where code must be placed in files depending on where the modules are defined.

```rust
// src/main.rs or src/lib.rs 
mod routes;

pub use crate::routes::health_route; 

pub fn run_checks() {
    health_route::check_health();
}

// src/routes.rs
pub mod health_route;

// src/routes/health_route.rs
pub fn check_health() {}
```

There is an older but still supported style of file path whereby the compiler will check for a file at location `[some_module]/mod.rs`, as shown below. You can use a mix of styles but this is not encouraged and the older style (`mod.rs`) is not good as it leads to a proliferation of mod.rs files in large projects so don't use it.

```rust
src/routes.rs         // most common idiomatic style
src/routes/mod.rs     // older less preferred style
```


### Paths

Once a module is part of a crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, a `MyType` type in the `my_module my_sub_module` module would be found at `crate::my_module::my_sub_module::MyType`.

Paths have two forms:-

- _Absolute path_ - the full path starting from the crate root.
    - Paths in the same crate begin with the literal `crate`
    - Paths in external crates begin with the crate name.
- _Relative path_ - starts from current module and `self` or `super`, or an identifier.

Path Structure:-

- All paths are followed by some identifier (ie. module, struct, enum, function) separate by double colons (`::`).
- The `crate` keyword (for current crate) or crate name (external crates) are like saying start at the root of the filesystem (`/`).
- Choosing which path type to use (if both are valid) is a project choice. If you restructure modules then you may need to update absolute paths, and if you move code between modules then relative paths will need updating. The general preference is to use absolute paths because, assuming you have designed your module structure well and is pretty stable, this allows you to move code definitions around without too much updating (which you may have if you use relative paths).
- You can of course leverage the `use` keyword to create shortcuts to paths.

```rust
mod routes {
    pub mod health_route {
        pub fn check_health() {}
    }
}

pub fn run_checks() {

    // Relative path
    super::routes::health_route::check_health();
}
```

#### Relative paths using `super`

You can use the `super` keyword in paths which means the next level up the module tree from the current location, it is like using `..` in a directory path.


### Bringing Paths into scope (imports)

#### The Use Keyword

The `use` keyword enables you to write more concise code by creating shortcuts to what would otherwise be long paths that you would have to repreat in your code. You can think of them like symbolic links.

```rust
use crate::routes::health_route;

pub fn run_checks() {
    health_route::check_health();
}
```

Note that `use` statements only creates a shortcut in the current scope. If `run_checks()` was moved into another scope (inside a `mod host`) then you would either need to move the `use` statement inside the module or reference the `use` shortcut in the parent module with `use super::health_route`.

```rust
mod host {

    use crate::routes::health_route;

    pub fn run_checks() {
        health_route::check_health();
    }
}

// or

use crate::routes::health_route;

mod host {

    use super::health_route;

    pub fn run_checks() {
        health_route::check_health();
    }
}
```

#### Idiomatic Function paths

We could create a `use` path which creates a shortcut all the way to the function so that we would not need to reference the functions parent module when calling it.

```rust
use crate::routes::health_route::check_health;

pub fn run_checks() {

    // its not obvious whether this is local or not
    check_health();
}
```

However, it is useful to specify the parent module when invoking the function since this makes it clear the function isn't defined locally. So the idiomatic approach is to create a shortcut just to parent module of a function so that it makes it clear when calling it that it is not local `health_route::check_health()`.

#### Idiomatic Structs & Enum paths

In contrast to functions, when bringing structs, enums and other items into scope with `use` it is more conventional to specify the full path. There is no technical reason for this it has just become the idiomatic approach that has emerged over time.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

#### Resolving Path name clashes

If you are bringing two items with the same name but different parent modules into scope, you cannot use the above idiom and you need to either:-

- Reference the parent modules.
- Modify one or more of the names with the `as` keyword.

#### Path name aliases

One way to resolve name clashes, or simply provide a new name which you favour, is to rename an item with the `as` keyword.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;   // IoResult does not clash with Result

fn function1() -> Result { ... }

fn function2() -> IoResult<()> { ... }
```

#### Using external packages

The standard library `std` is an external crate but since it is shipped as part of the Rust language you do not need to explicitly declare a dependency on it in _Cargo.toml_. However, you do need to bring items from the standard library into scope.

```rust
use std::collections::HashMap;
```

For all other external creates, making them available for your package to use follows these steps:-

1. Declare the crate and version in _Cargo.toml_.
2. Bring the items you want to use into scope with `use` paths.
    
```rust
// Cargo.toml
// Importing version 0.8.5 of Random create
rand = "0.8.5"

// some file - bring `Rng` trait into scope.
use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=100);
}
```

#### Nested Paths for concise imports

Items from the same module can be grouped like this.

```rust
// instead of this
use std::io.Read;
use std::io.Write;
// do this
use std::io::{Read, Write];
```

If you want to bring the module into scope as well as some of its items then you can use `self`.

```rust
// instead of doing this
use std::io;
use std::io::Write;

// you can do this
use std::io::{self, Write};
```

Items from the same crate but with different paths can be brought into scope on a single line by using nested paths.

```rust
// instead of this
use std::cmp::Ordering;
use std::io;    

// you can do this
use std::{cmp::Ordering, io};
```

**Why bring a module like `std::io` into scope?**

Refer to the "Idiomatic Function paths" section for more details - we typically bring modules into scope because we might want to invoke functions declared there.

#### The Glob operator (`*`)

You can bring _all_ public items defined in a path into scope using the glob operator.

```rust
use std::collections::*;
```

Take care when using the glob operator as it makes it more difficult to tell what is in scope and where something was defined.

It is typically used when testing to bring everything under test into the `tests` module and also when using the [prelude pattern](https://doc.rust-lang.org/stable/std/prelude/index.html#other-preludes).

#### Re-exporting paths

Consider the following code. In its current form, any external code would not be able to reach the `check_health()` function because even though the function itself, and its parent module `health_route` are public the `routes` module which contains them is not public. 

```rust
// crate's root module - src/lib.rs

mod routes {
    pub mod health_route {
        pub fn check_health() {}
    }
}

use crate::routes::health_route;

pub fn run_checks() {
    health_route::check_health();
}
```

Through the _re-exporting_ feature of Rust we do have the option of exposing otherwise private items. We could expose (aka re-export) the `health_route` module by making the path public using `pub use`. If our crate was called **router** (for example) then the following line would allow any external code to call `router::health_route::check_health()`.

```rust
pub use crate::routes::health_route;
```

Re-exporting allows you to expose internal items through a module's public interface using the `pub use` syntax and is a useful feature for:-

1. Creating a more convenient public API.
2. Organizing your code internally while presenting a different structure externally.
3. Making deeply nested items available at a higher level

The following is an example of how you can expose a different public API. `Circle` is positioned within a private `shapes` which intuitively makes sense for the structure of the project. You can expose a simple path to Circle however.

```rust
// In lib.rs of "my_crate"
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

```rust
// your_library imports the Serde library and Reqwest
pub use serde::{Serialize, Deserialize};
pub use reqwest::Client as HttpClient;

// Now users of your library can do:
use your_library::Serialize;  // Instead of serde::Serialize
use your_library::HttpClient; // Instead of reqwest::Client
```

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

### Preludes (import groups)

The [std::prelude](https://doc.rust-lang.org/std/prelude/index.html) is the set of utilities in the standard library that are automatically imported into every program without you needing to manually import them. This set is kept small and focussed and just includes features that are commonly used across all Rust programs.

In addition to the standard library prelude, various other libraries have their own preludes eg. [std::io::prelude](https://doc.rust-lang.org/std/io/prelude/index.html) includes many common I/O traits so if you are writing some I/O heavy module you may want to just import the io::prelude rather than every part of the io library that you need. All these other preludes have to be manually imported into your modules.

### Workspaces

For very large projects comprising a set of interrelated packages that evolve together, Cargo provides [workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).


---


## Building Projects

### Cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the Rust package manager - it creates projects, downloads your project's dependencies compiles your packages, uploades your distributions to crates.io etc etc.

### Reproducible builds with Cargo.lock

See [Ensuring Reproducible Builds with the Cargo.lock File](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file)

The Cargo.lock file tracks the versions of all the dependencies your projects uses. It uses the lock file to determine which versions it needs each time your project is compiled, which provides two benefits:-

1. Cargo does not have to go and figure out the dependency graph each time it builds youe project (unless you have changed some of the dependency configuration in Cargo.toml between builds).
2. Each build will use the same version of dependencies (unless you update the configuration).

### Cargo `update`

[Updating a Crate to Get a New Version](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#updating-a-crate-to-get-a-new-version)

The `cargo update` command allows you to update versions of dependencies which then updates Cargo.lock.


---  


## Appendix A - Stack & Heap Refresher

[An interesting SO post](https://stackoverflow.com/a/47180043/3008323)

Rust is no different than many other programming languages when it comes to where data is stored - data of a fixed size (at compile time) is on the stack and data whose size cannot be determined (at compile time) will live on the heap. The data on the heap will actually use both the stack and the heap since the pointer address data (to the location on the heap etc) will be on the stack.

#### Stack

- Anything that has a fixed known size at compile time is stored on the stack - the scalar types (that represent a single value - Integer, Float, Boolean, Char), tuples and arrays (if their elements are of a type that reside on the stack), string literals.
- Data on the **stack** is fast to access since it is written/removed on a LIFO basis. A program pushes data on top of the stack as it works with it and pops it off when it is done with it.  
- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables, get pushed onto the stack. When the function is over, those values get popped off the stack.

Stack memory management is relatively straight-forward, data of a known size are pushed onto the stack and then popped off afterwards when they go out of scope. Since these data are also of a known size, they can be trivially copied (on the stack) if another part of code, in another scope, needs to work with it.

#### Heap

- Data whose size may change must live on the **heap**.
- When you put data on the heap the allocator finds an area big enough to store that data, marks it as being used, and returns a **pointer** (as address) to the location. This is called _allocating on the heap_ and or just _allocating_ and is more complex than managing data on the stack and therefore is slower.
- Since the pointer is a known fixed size it is stored on the stack. When you want the actual data you must follow the pointer to the address on the heap. 
- Contemporary processors are faster if they jump around less in memory ie. they are working with data that is in close proximity to each other on the heap.



## Appendix B - Tools

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


## Appendix C - Compilation

- Rust is an Ahead of Time (AOT) compiled language.
- A unit of compilation in Rust is a crate.
- When you compile a project, all of its dependencies are also compiled because any features (of each dependency) that you have not enabled shall be stripped out of the resulting binary. This is a great optimisation since it means that your resulting binary only contains what it needs and is therefore as small as it can be.
- This is why Rust dependencies use [source-based distribution](https://crates.io/) rather than pre-compiled binaries.

### Compilation Steps (MIR -> LLVM IL -> ASM)

The compilation process follows these stages with each step adding more low-level details before producing the final artefact which is the executable containing the binary instructions that can be directly loaded into memory and executed by the processor (_machine code_).

Rust Source Code → MIR → LLVM IR → ASM → Machine Code. 

#### MIR (Mid-level Intermediate Representation)

MIR is an intermediate representation in the Rust compiler (rustc) that comes after the initial parsing and type checking stages. It's a simplified, lower-level representation of the code that helps with:-

- Can be viewed using `rustc -Z print-mir`
- Performing borrow checking and ownership validation.
- Enabling compiler optimizations.

#### LLVM (Low Level Virtual Machine)

LLVM is a compiler infrastructure project which the Rust compiler leverages to transform the MIR to the LLVM Intermediate Representation which:-

- Leverages LLVM's powerful optimization passes.
- Generates machine code for different target architectures.
- It can be viewed with `rustc --emit=llvm-ir`

#### ASM (Assembly)

> ASM is just an abbreviation of ASseMbly - it is not an acronym
> Assembly language uses human-readable mnemonics to represent machine code instructions. Each assembly instruction typically corresponds directly to a single machine code instruction for a specific processor architecture.

- The LLVM now converts its IR into low-level machine code specific to the particular computer architecture you are compiling for ie. _native assembly code_.
- This is specific to the target architecture eg. x86_64, ARM etc.
- It is human readable but requires knowledge of the specific CPU architecture's instruction set to be able to understand it.
- It cannot be executed by a processor, it still requires a final step of being converted into machine language.
- You can view it with `rustc -s`.
- Think of assembly as a recipe written in words which the assembler then translates into the final dish (the executable binary) which can be eaten.

#### Machine Language (binary)

This is the set of binary instructions (for a specific processor architecture) that can be directly loaded into memory and executed.

### Rust compiles into platform-specific libraries

Unlike Java (for example), which compiles source code into platform-dependant JARs, Rust compilation generates platform-specific crates. 

- When you compile a Rust library for different platforms (targets), you will get an `.rlib` file for each target platform. This is handled by Rust's cross-compilation system - note that you need appropriate toolchains installed for cross-compilation.
- The Rust's compiler (rustc) will target your current platform's architecture and operating system unless specified otherwise. This default target is called the "host" target.

### Inlining (optimization)

[Inline In Rust](https://matklad.github.io/2021/07/09/inline-in-rust.html)

Inlining is an optimizing transformation that takes place at compile time and which replaces a call to a function with its body during compilation.

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

