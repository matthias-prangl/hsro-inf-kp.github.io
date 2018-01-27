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

---

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

+++

### Creating Projects

Easily create new projects:

```bash
cargo new new_project
cargo new new_bin_project --bin
```

Generates completet project structure

```bash
├── Cargo.toml
└── src
    └── lib.rs #or main.rs if created with --bin
```

+++

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

+++

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

---

# Type Safety

- _Well defined_ programs don't exhibit unexpected behavior
- _Type safe_ languages only allow well defined programs

Note:
Neben offensichtlichen Dingen wie checken ob String methode tatsächlich auf String aufgerufen.

+++

## C example

```c
int main(void) {
    unsigned long x[3];
    x[3] = 0x0x7ffff7aaaaaa;
}
```

@[2](Declare unsigned long array with length 3)
@[3](Write to array index 3)
@[1-4](Leads to undefined behavior, the value may or may not have been written)

Note:
Compiler Warnung, aber kein Fehler.
Je Nach Ausführungsrechten an Speicher geschrieben

+++

## Rust equivalent

```rust
fn main() {
    let mut x: [u64; 3] = [1, 2, 3];
    x[3] = 0x0x7ffff7aaaaaa;
}
```

@[2](Declare u64 array with length 3)
@[3](Write to array index 3)
@[1-4](Thread main panics. Index out of bounds)

Note:
Uninitialisiertes array nicht erlaubt.
Länge immer explizit angegeben.
Panic bei nicht behebbaren Fehlern.
Panic ist definiertes Verhalten -> Type safe.

+++

## Panic!

