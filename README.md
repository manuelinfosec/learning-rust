## *"Hello World"* in Rust

Creating the source file, `main.rs`: 
```rust
fn main() {
	println!("Hello World!");
}
```
 - `main`: This is where the program begins execution.
 - `!` indicates this is a Rust macro instead of a function.

Compiling the source files to `exe`:
```bash
rustc main.rs
./main.exe
```


## Creating a Cargo project

```bash 
cargo new <project_name> --bin
```

Creates a `Cargo.toml` file in the project directory. File sample:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"
author = ["Manuel <manuelinfosec@gmail.com>"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- `[package]` is a section heading that indicates that the following statements are configuring a package. Next few details are the package configuration.
- `[dependencies]` is the start of a section for you to list any of your project's description. In Rust, packages of code are referred to as *crates*.

A sample "Hello World!" program is created *src/main.rs*. 

## Building Cargo project

From the *hello_world* directory, build your project:

```bash
cargo build
```

An executable is created at *./target/debug/hello_cargo.exe*. To run the executable,

```
.\target\debug\hello_cargo.exe
```

 or 
 
```
cargo run
```

You can check to make sure it compiles without producing an executable with:

```
cargo check
```

Checking is faster than building building it skips the process of creating an executable.

### Building Cargo project for release 

When a project is finally ready for release, it can be compiled with added optimizations with,

```bash
cargo build --release
```

This creates the executable in the *target/release* instead of *target/debug*. The optimizations make Rust code run faster.

## Conclusion
We've been able to create a sample Cargo project that prints "Hello World" to the screen. This introduced the idea of compiling a project for development, optimization for release and checking that the project works without producing binaries.

Next: [[#Programming a Guessing Game]]

# Programming a Guessing Game

```rust
fn main() {
    println!("Guess the number!");
    println!("Please input the guess: ");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed : {}", guess);
}
```

The `io` library comes from the standard library. By default Rust only brings in few types into the scope of the program in the *prelude*. `main` is the entry point to the program. `fn` declares a new function. `println!` is a macro that prints string to the screen. 

## Storing Values with Variables

```rust
let mut guess = String::new()
```

- `let` used to create a variable. Example, `let foo = bar`. This create a variable `foo` and binds it to the value bar. In Rust, variables are immutable by default.
- `mut` is used just before the variable name to make it mutable:

```rust
let foo = 5; // cannot be reassigned
let mut bar = 5; // this variable can take on a new value
```

**Note:**
	*// starts a comment that continues till the end of the line. Comments are ignored*

- `let mut guess` introduces a mutable variable named `guess`. On the other side of the equal sign (=) is the value is bound to, which the result of calling `String::new`, a function that returns a new instance of a `String`. `String` is a string type provided by the standard library.
- `::new` indicates that `new` is an associated function of the `String` type.  
- `read_line` calls the standard input handle to get input from the user. The argument needs to be mutable so the method can change the string's content by adding the user's input.
- `&` indicates that the argument is a *reference*. Good way to let multiple parts of code access one piece of data without needing to copy that data into memory multiple times.

## Handling Potential Failure with the Result Type

The `Result` types in `io::Result` are enumerations, also known as *enums*. An enumeration is a type that can have a fixed set of values, and those values are called the enum's variants.

For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

- `expect` is a method of `io::Result`.

## Using External Crates

Add to the `Cargo.toml` file:

```
[dependencies]

rand = "0.3.14"
```

This used the `rand` library version v0.3.14 during building. 

## Generating a Random Number

Updated code: 

```rust
extern crate rand;

use rand::Rng;
  
fn main() {
    println!("Guess the number!");
    let secret_numer = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_numer);
  
    println!("Please input the guess: ");
  
    let mut guess = String::new();


    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
  
    println!("You guessed : {}", guess);
}
```

- `extern crate rand` indicates to the compiler that we're using the `rand` crate as a an external dependency. This is equivalent to `use rand`. The code will still compile without this line.
- `use rand::Rng`: The `Rng` trait defines the methods that random number generators implement, and this trait must be in scope for us to use those methods.
- `rand::thread_rng` function gives the particular rand number generator that will be used. One local to the current thread of execution and seeded by the operating system. 
- `gen_range` method on the random number generator is defined by the `Rng` trait that was brought into scope. This method takes two numbers as arguments and generates a random number between them. The upper bound matters a lot, so we need to specify `1` and `101` to request a number between 1 and 100.

## Comparing Secret Value with Guessed Value

```rust
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input the guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed : {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- `std::cmp::Ordering` is brought into scope from the standard library. Like `Result`, `Ordering` is another enum, but the variants for `Ordering` are `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.
- The  `cmp` method from the `Ordering` enum compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it is comparing the guess to the secret number. Then it returns a variant of the `Ordering` enum we brought into scope. 
- `match` is used to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

However, the above code will not compile. Error message: 

```rust
error[E0308]: mismatched types
   --> src\main.rs:21:21
    |
21  |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: associated function defined here
   --> C:\Users\USER\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib/rustlib/src/rust\library\core\src\cmp.rs:780:8
    |
780 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```


The core of the error states that there are *mismatched types*.  We're trying to compare a string (*guess*) to an integer (*secret_number*). Because of the strict, static type system, Rust will not permit this operation. Hence, it returns an error during compilation.

To fix this, we'll convert the input (which defaults to string) to its integer equivalent, and if the conversion isn't possible, we return an error.

```rust
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input the guess: ");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
        
    let guess: i32 = guess.trim()
                        .parse::<i32>()
                        .expect("Please type a number!");
    
    println!("You guessed : {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

^f257ba


In this new block, the `guess` variable is overwritten with a version of guess that uses:
- `trim()`:  `String` method that removes all whitespaces at the beginning and end.
- `parse()`: `String` methods that is used to convert from one type to the specified type. This method returns a `Result` enum.
- `i32` (*unsigned 32-bit integer*): specifies the type we're parsing the string to. `u32` is good choice for small number.

## Allowing Multiple Guessing With Looping

```rust
// -- snip --

fn main() {
    println!("Guess the number!");

    // maximum number of guesses allowed
    const MAX_GUESSES: u32 = 5;

    // this counter tracks the number of guesses
    let mut counter: u32 = 1;

    // generate a random number with the thread_rng() trait
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // allow multiple retries
    let final_secret: i32 = loop {
        // update counter value
        counter += 1;

        // display for prompt
        println!("Please input the guess: ");
	
	// -- snip --
        
        // compare guess with secret number and return Enums from Ordering.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),

            // do some extras and return the secret number
            Ordering::Equal => {
                println!("You win!");
                break secret_number;
            }
        };

        // if the number of guesses hit the threshold
        if counter > MAX_GUESSES {
            println!("You've hit the maximum number of retries.");
            break secret_number;
        }
    };

    // reveal secret number
    println!("\nThe secret number was {final_secret}");
}

```

This idea behind this piece allows for multiple guesses using a loop, and tracks the number of guesses using a counter, it then breaks upon a successful guess or when the number of guesses hit the `MAX_GUESSES` threshold.

- `break` returns the `secret_number` to `final_secret` which is initialized at the start of the loop. 
- The `Ordering` enum allows for more statements within its scope. 

## Handling Invalid Input
Instead of crashing the entire program when there is invalid input, the game should ignore and request for another input, so the user can continue guessing.

```rust
// -- snip --

io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line!");

//parse string to specified data type
let guess: u32 = match guess.trim().parse::<u32>() {
	Ok(num) => num,
	Err(_) => continue,
};

// display guess result
println!("You guessed : {}", guess);

// -- snip --
```

With this, we've moved from crashing the programming to handling the error. If you recall, the `parse`  method returns a `Result` enum which consists for the `Ok` and `Err` variants. The `match` expression is used to check for the returning enum and handles them appropriately. If there is an invalid input, the `parse` method returns the `Err` variant. Instead of crashing, it uses the `continue` keyword to restart the loop. The `__`(underscore) is a catchalll value that specifies that it matches the error regardless of its containing value.

If `parse` can successfully return a number from the string, it returns the `Ok` variant and its containing number. This matches the first arm pattern and return its containing number, `num`.

### Drawdown (Bug)
The `if` condition is moved further up, so it can correctly run in the case of errors from the match case that restarts the loop. 

If this isn't done, the counter value is still updated when there has been no guesses, and upon a successful guess it breaks the loops without having to go through the `MAX_GUESS` threshold.

Updated code:

```rust
// rand was installed externally
extern crate rand;

// import Rng trait from rand
use rand::Rng;

// import Ordering enum & IO from standard library
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // maximum number of guesses allowed
    const MAX_GUESSES: u32 = 5;

    // this counter tracks the number of guesses
    let mut counter: u32 = 1;

    // generate a random number with the thread_rng() trait
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    // allow for multiple retries
    let final_secret: u32 = loop {
        // update counter value
        counter += 1;

        // if the number of guesses hit the threshold
        if counter > MAX_GUESSES {
            println!("You've hit the maximum number of retries.");
            break secret_number;
        }

        // display for prompt
        println!("Please input the guess: ");

        // create a mutable, empty string variable
        let mut guess: String = String::new();

        // collect input from standard input and save to the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        //parse string to specified data type
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // display guess result
        println!("You guessed : {}", guess);

        // compare guess with secret number and return Enums from Ordering.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),

            // do some extras and return the secret number
            Ordering::Equal => {
                println!("You win!");
                break secret_number;
            }
        };

    };

    // reveal secret number
    println!("\nThe secret number was {final_secret}");
}
```

## Conclusion
At this point, I've successfully built a guessing game with extra functionality. This project was a hands-on way to learn about `let`, `match`, `break`, enumerations, installing external crates and the random number generator library. Methods and associated functions are not excluded too.

# Common Programming Concepts

- By default, variables in Rust are immutable. 

For this exercise, a new project will be created named, `variables`. The command below does so:

```bash
cargo new variables --bin
```

To demonstrate that a variable in Rust is immutable by default, the code below returns an error:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}
```

Error summary: 

```
cannot assign twice to immutable variable `x`
```
The error indicates that you cannot assign twice to an immutable variable, this means, after declaring the variable *x*, you cannot re-assign another value to the declared variable. This also indicates how helpful compilers can be in finding errors.

Never the less, variables can be made mutable by including the `mut` keyword just before the variable name as show below.

```rust
let mut x = 5;
```

When we run the program again with the above line replaced in the previous variable declaration, we get results. Updated code should look like this:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}
```

Results:

```
   Compiling variables v0.1.0 (C:\Users\USER\Manuel Files\Code Playground\Rust\Learn\variables)
    Finished dev [unoptimized + debuginfo] target(s) in 3.39s
     Running `target\debug\variables.exe`
The value of x is: 5
The value of x is: 6
```

Now, we're allowed to rewrite the value in the `x` variable safely without being thrown compilation errors. 

## Considerable trade-offs between immutable and mutable variables

- Using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to thing through, so lower performance might be a worthwhile penalty for gaining that clarity.

## Differences Between Variables and Constants

- `mut` is not allowed for use with constants. Constants are always immutable.
- Constants are declared using the `const` keyword as opposed to variables that use the `let` keyword.
- In constants, the variable type *must* be annotated. 
- Constants can be declared in any scope, including the global scope. 
- Constants are only set to a constant expression, not to the result of a function call or values that could be computed an runtime.

Example of a constant declaration where the constant's name is `MAX_POINTS` and its value is set to 100,000. The naming convention for constants is to use all uppercase with underscores between words:

```rust
const MAX_POINTS: u32 = 100,000;
```


## Data types in Rust

- In cases when may types are possible, such as when we converted a `String` to a numeric type using `parse` in [[Rust Study Notes#^f257ba]]

### Scalar Types
A *scalar* type represents a single value. Rust has for primary scalar types: 
- integers, 
- floating-point numbers, 
- Booleans and, 
- characters.

### Integer Types
An *integer* is a number without a fractional component. The type declaration indicates that the value it's associated with should be an unsigned integer (signed integer types start with `i` instead of `u`) that takes up 32 bits of space.

It can range from `i8/u8` to `i64/u64`.

<table>
		<th>Length</th>
		<th>Range of values</th>
	<tr>
		<td>8-bit</td>
		<td>-128 to 127</td>
	</tr>
	<tr>
		<td>16-bit</td>
		<td>-32,768 ‬to 32,767 ‬</td>
	</tr>
		<tr>
		<td>32-bit</td>
		<td>-2,147,483,648 ‬to 2,147,483,647</td>
	</tr>
</table>

- Unsigned means the number can never have a sign (negative) and will forever be positive, signed means the number can take on the negative sign therefore it can be a negative number.
- Each signed variant can store value from $-(2^{n-1})$ to $2^{n-1} - 1$, where *n* is the number of bits the variant uses. So an `i8` can store value from $-(2^{7})$ to $2^{7}-1$.  Its unsigned variant can store numbers from $0$ to $2^{7}$. $2^{7}$ equals 128.
- Integers can be written as *57u8* or *1_000*, using \_(underscore) as a visual separator.

<table>
		<th>Number Literal</th>
		<th>Examples</th>
	<tr>
		<td>Decimal</td>
		<td>98_768</td>
	</tr>
	<tr>
		<td>Hex</td>
		<td>0xff</td>
	</tr>
		<tr>
		<td>Octal</td>
		<td>0o88</td>
	</tr>
		<tr>
		<td>Binary</td>
		<td>0b1111_0000</td>
	</tr>
	<tr>
		<td>Byte (u8 only)</td>
		<td>b'A'</td>
	</tr>
</table>


If unsure on the integer type to use, Rust defaults to `i32`, this type is generally the fastest, even on 64-bit systems.

### Floating-Point Types
Rust's floating points types are `f32` and `f64`, which are 32 bits and 64 bits in size respectively. The default type is `f64`, this is considerably as fast as `f32` and has got more precision.

Floating numbers in action:
```rust
fn main() {
    let x = 2.0; // f64 

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the [**IEEE-754**](https://wikipedia.org/wiki/IEEE_754) standard. The `f32` type is a single-precision float, and `f64` has double precision.

### Numeric Operations
Like every other programming language, Rust supports basic mathematics operations for all of the number types: addition, subtraction, multiplication, division and remainder.

```rust
fn main() {
    // addition
    let _sum: u8 = 5 + 10;

    // subtraction
    let _difference: f32 = 95.5 - 4.3;

    // multiplication
    let _product: u8 = 4 * 30;

    // division
    let _quotient: f32 = 56.7 / 32.2;

    // remainder 
    let _remainder: u8 = 43 % 5;
}
```
