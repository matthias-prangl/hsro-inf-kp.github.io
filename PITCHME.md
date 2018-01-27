# Rust 

memory and type safety

--- 

## Agenda

- History
- Basics
- Cargo
- Type Safety
- Memory Safety
- Ownership
- Borrowing & References

+++

## Agenda

- Lifetimes
- Traits and Generics
- Smart Pointers
- Closures
- Threads

---

## Rust Basics

+++

## Cargo

<div class="twocolumn">
  <div>
    - Rust package manager </br>
    - Compiles the project </br>
    - Creates Packages
  </div>
  <div>
    <img src="https://raw.githubusercontent.com/matthias-prangl/rust/master/assets/cargo_logo.png" alt="cargo">
  </div>
</div>

---

### Creating Projects

Easily create new projects:

```sh
cargo new new_project
cargo new new_bin_project --bin
```

Generates completet project structure

```sh
├── Cargo.toml
└── src
    └── lib.rs $or main.rs if created with --bin
```

---

### Adding Depencies

- Cargo.toml describes your dependencies.
- Dependencies automatically downloaded and compiled

```toml
[package]
name = "new_project"
version = "0.1.0"
authors = ["Matthias Prangl <matthias.prangl@gmail.com>"]

[dependencies]
rand = "0.4.2"
```

---

### Using Dependencies

```rust
extern crate rand; //link the rand crate
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let random_number: u8 = rng.gen();
    println!("{}", random_number);
}
```

+++

# Type Safety
