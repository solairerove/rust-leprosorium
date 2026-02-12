# Rust Notes

A short reference for core Rust concepts used in this project.

## 1. Immutability

Variables are immutable by default.

```rust
let x = 10;
// x = 20; // compile-time error

let mut y = 10;
y = 20;
```

Java analogy:
- `let x = ...` is similar to `final`
- `let mut y = ...` is mutable

## 2. Explicit Error Handling (`Result`)

`Result<T, E>` means:
- `Ok(T)` successful value
- `Err(E)` error value

```rust
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    let p: u16 = s.parse()?;
    Ok(p)
}

fn main() {
    match parse_port("3000") {
        Ok(port) => println!("port={port}"),
        Err(err) => eprintln!("invalid port: {err}"),
    }
}
```

Practice:
- Do not suppress errors.
- At application boundaries (HTTP handlers, CLI entrypoints), map errors to clear responses/logs.

## 3. Ownership, Move, and Borrowing

### 3.1 Ownership

A value has one owner. When the owner goes out of scope, the value is dropped.

```rust
fn main() {
    let a = String::from("hello");
    let b = a; // move: ownership transferred to b

    // println!("{a}"); // error: a no longer owns the value
    println!("{b}");
}
```

### 3.2 Borrowing (read-only)

```rust
fn print_len(s: &String) {
    println!("len = {}", s.len());
}

fn main() {
    let a = String::from("hello");
    print_len(&a); // pass a reference, not ownership
    println!("{a}"); // a is still valid
}
```

### 3.3 Mutable Borrowing

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut a = String::from("hello");
    append_world(&mut a);
    println!("{a}");
}
```

Rule:
- either many `&T` (read-only),
- or one `&mut T` (write),
- not both at the same time.

## 4. `Option` Without Syntactic Sugar

`Option<T>` is an enum:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example without `if let`, `unwrap_or`, etc.:

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Option::Some(String::from("Alice"))
    } else {
        Option::None
    }
}

fn main() {
    let result = find_user(2);

    match result {
        Option::Some(name) => {
            println!("found: {}", name);
        }
        Option::None => {
            println!("not found");
        }
    }
}
```

Why this matters: instead of `null`, the compiler forces you to handle both branches.

## 5. Axum/Tokio Practices

For the current notes service:
- Keep handlers thin: parse request + call logic layer + return response.
- Keep business logic in separate modules (`store`, `service`).
- Avoid `unwrap/panic` in runtime paths; return `Result` and map to HTTP status codes.
- Keep `AppState` in `Arc`; use synchronization (`Mutex/RwLock`) minimally.
- Validate input at boundaries (`title/body` not empty).
- Log errors and important events.

## 6. Mini Pre-Commit Checklist

- No unnecessary `unwrap/panic` in user-facing flows.
- All `Option/Result` branches are handled explicitly.
- Function interfaces are simple; prefer references where possible (`&T`, `&str`).
- `cargo fmt`
- `cargo check`
- `cargo test`
