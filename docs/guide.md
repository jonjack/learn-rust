

# Rust Guide


A summary of my Rust learnings largely guided by reading the offical Rust guides:-

[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)        
[The Rust Language Reference](https://doc.rust-lang.org/reference/introduction.html)         
[Rust by Practise](https://practice.course.rs/)       



```rust
// key for code sample comments

✅     code compiles
❌     compiler error: code does not compile
⚠️     compiler warning: compiles but does not follow convention
```


---


<!--TOC-->


## TL;DR

This section which summarises the most useful aspects of what I think I shall need to keep in mind when writing code. It should evolve with further understanding.

- [Keyword](https://doc.rust-lang.org/book/appendix-01-keywords.html) reference.
- [Operators & symbols](https://doc.rust-lang.org/book/appendix-02-operators.html) reference.
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

### Pattern Matching on Option<T>

Option has two variants - Some or None - and the common approach to handling an Option is with a match block.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```


## Structs

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

### Tuple Structs

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

### Unit Structs

It is possible to declare structs that do not contain any fields called _unit-like structs_ because they behave similarly to `()`.` They can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;
```

### Struct data ownership and Lifetimes

You can define structs with fields which are owned types, eg. the String type (which is owned by its scope) rather than the string slice type `&str` which is a reference. When using the String type, each instance of the struct will own all of its data and that data will be valid for as long as the entire struct is valid.

It is also possible to define structs with references such as the string slice, in which case the actual String may be owned by something other than the struct instance. In order to do this you need to use Lifetimes.



## Functions & Methods

- Methods are basically functions defined within a struct, enum or trait. They have a first parameter called `self` which represents the instance of the struct they are being called on. They typically work on the data of the struct or enum instance.
- Functions are similar to methods except they stand alone and are not tied to a specific instance of a struct or enum.

### Functions

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

### Associated functions (`::`)

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

### Methods

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



## Enums

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

The `call()`` method uses `&self` to get the value that it was invoked on, in this example a String.

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



## Notable Types

An adhoc list of common types I have come across.

### Unit

Unit is a tuple without any values. 

- This value and its corresponding type are both written `()` and represent an empty value or an empty return type. 
- Expressions implicitly return the unit value if they don’t return any other value.
- One practical use of Unit is when we don't care about a generic type, and () makes this explicit. For example, a `Result<(), String>`` can be used as return type for a function that either completes successfully or fails for a variety of reasons.

### Tuple

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

### Array

Arrays are another way to group multiple elements.

> The Vector type, unlike Array, is a collection that is allowed to grow or shrink in size. Unless you need to ensure your collection resides on the stack and has a fixed size, you should use Vector over Array.

- Unlike tuples every element of an array must have the same type.
- Arrays in Rust have a fixed length.
- You create them using square brackets `let arr = [1, 2, 3, 4, 5]`

Arrays are useful:-

- When you want your data allocated on the stack, rather than the heap.
- When you want to ensure you always have a fixed number of elements ie. you know the number of elements will not need to change eg. you are storing reference data like the months of the year.

### String

[parse()](https://doc.rust-lang.org/std/primitive.str.html#method.parse) converts a string slice into any type that implements the [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait. It will return `Err` if its not possible to parse into the desired type.

See [String vs &str](https://www.reddit.com/r/rust/comments/1695k03/string_vs_str/).     
String [ASCII](https://gist.github.com/jonjack/76ae94ad83c07ddb1cd2ee286f69e564).     

### Result

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

### Ordering

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


## Memory management (Ownership)

[What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

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

[The Slice Type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html#the-slice-type) allows you to reference to a contiguous sequence of elements in a collection rather than the collection as a whole.

- It can be used with different types of collections but is commonly used with strings to refer to a substring ie. the string slice.
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

String literals (because their memory requirements are know at compile time) are stored within the binary. The type of string literals are a string slice `&str` which are a reference to that particular part of the binary. `&str` is an immutable reference so string literals must be immutable.

```rust
let s = "Hello, world!";    // string literals are implicitly the &str slice type
```

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



