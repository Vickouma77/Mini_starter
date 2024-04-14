# rust

rust is a statically typed, compiled language that is designed for speed, safety, and concurrency. It is a systems programming language that is focused on safety and performance. It is a great language for building high-performance applications that require low-level control over the hardware.

## Getting Started

:one:  To get started with rust, you will need to install the rust compiler. Check out the [official rust website](https://doc.rust-lang.org/book/ch01-01-installation.html) for instructions on how to install rust on your system.

:two:  Once you have installed the rust compiler, you can start writing rust code. You can use any text editor or IDE to write rust code.

## Cargo

Cargo is the package manager and build tool for rust. It is used to create, build, and manage rust projects. Cargo makes it easy to create new projects, add dependencies, build the project, run tests, and generate documentation.

<h2 align="center">Cargo Commands</h2>

| Command | Description |
| ------- | ----------- |
| `cargo new <project-name>` | Create a new rust project |
| `cargo build` | Build the project |
| `cargo run` | Build and run the project |
| `cargo check` | Check the project for errors |
| `cargo test` | Run the tests in the project |
| `cargo doc` | Generate the documentation for the project |
| `cargo doc --open` | Generate and open the documentation for the project |
| `cargo add` | Add a dependency to the project |
| `catgo fetch` | Fetch the dependencies in the project |
| `cargo update` | Update the dependencies in the project |
| `cargo clean` | Clean the project |
| `cargo remove` | Remove a dependency from the project |

## Common Concepts

- **Variables**: Rust has a strong static type system that requires you to declare the type of a variable when you define it.
- **Functions**: Rust functions are defined using the `fn` keyword and can take arguments and return values.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- **Control Flow**: Rust has if expressions, loops, and pattern matching to control the flow of execution.

```rust
fn main() {
    let x = 10;

    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than or equal to 5");
    }
}
```

- **Structs**: Rust has a struct type that allows you to define custom data types with named fields.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

- **Enums**: Rust has an enum type that allows you to define custom data types with multiple variants.

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

- **Traits**: Rust has a trait system that allows you to define shared behavior for types.

```rust
trait Drawable {
    fn draw(&self);
}
```

- **Modules**: Rust has a module system that allows you to organize your code into separate files and namespaces.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

- **Error Handling**: Rust has a powerful error handling system that allows you to handle errors using the `Result` type.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## Advanced Concepts

- **Ownership**: Rust has a unique ownership system that allows you to manage memory safely without garbage collection.
- **Borrowing**: Rust allows you to borrow references to data without taking ownership of it.
- **Lifetimes**: Rust has a lifetime system that ensures that references are valid for as long as they are needed.
- **Concurrency**: Rust has built-in support for concurrency with threads and async/await.

## Examples

Here are some examples of rust code:

```rust
fn main() {
    println!("Hello, world!");
}
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", p.x, p.y);
}
```

## Resources

- [Official Rust Website](https://www.rust-lang.org/)
