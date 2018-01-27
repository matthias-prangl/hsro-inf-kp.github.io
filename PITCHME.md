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

# Rust Basics

---

# Cargo

<div class="twocolumn">
  <div>
    - Rust package manager </br>
    - Invokes rustc to compile </br>
    - Creates Packages
  </div>
  <div>
    <img src="https://raw.githubusercontent.com/matthias-prangl/rust/master/assets/cargo_logo.png" alt="cargo">
  </div>
</div>

Note:
Packages availiable at crates.io

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

Using the rand crate to print a random number

```rust
extern crate rand;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let random_number: u8 = rng.gen();
    println!("{}", random_number);
}
```
@[1](link the rand crate to the project)
@[2](bring used members in scope)
---

# Type Safety

- _Well defined_ programs don't exhibit unexpected behavior
- _Type safe_ languages only allow well defined programs

Note:
Neben offensichtlichen Dingen wie checken ob String methode tatsächlich auf String aufgerufen.
Type safety nicht nur bei statisch geschriebenen sprachen (z.B. python).

+++

## Undefined behavior in C

There is no way for the program/compiler to check whether or not x[3] is part of the array:
```c
int main(void) {
    unsigned long x[3];
    x[3] = 0x7ffff7aaaaaa;
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

Rust checks array bounds at run time:
```rust
fn main() {
    let mut x: [u64; 3] = [1, 2, 3];
    x[3] = 0x7ffff7aaaaaa;
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

Panics are for the kind of errors you can provide no solution for, like:
- division by zero
- assertion failures
- unwrapping an Option containing None
- out of bounds array access

How to panic! yourself:
```rust
panic!("You should never have seen this!");
```

Note: 
Panic macro änhnlich wie println mit argumenten.
Keine exceptions in rust. (Ressourcenaufwand siehe z.b. c++ in msp430)
+++

## In case a thread panics

- An error message is printed
- Temporary values, arguments and local variables are dropped. (Stack unwinding)
- The panicking thread exits

Note: 
Stack unwinding wie c++ exception handling. 
Mehr zu drop in Ownership.
Panic im main thread führt zu programmabsturz.

+++

## Option&lt;T&gt;

- For functions that may or may not return a value
- enum containing either:
    -  some value of type T (Some<T>)
    -  No value (None)

```rust
enum Option<T> {
    Some<T>,
    None,
}
```

Note: 
Genutzt bei z.B. 
Option wichtig weil keine nullpointer erlaubt sind, mehr dazu in references.

+++

## Using Option&lt;T&gt;

```rust
fn maybe_return_something(maybe: bool) -> Option<i32> {
    match maybe {
        true => Some(123),
        false => None,
    }
}

match maybe_return_something() {
    Some(val) => println!("I got: {}", val),
    None => println!("I got nothing!"),
}
```

@[8-11](Match on the returned Option and do something with the result)
+++

## Methods for Option<T>

The Option type provides some useful methods:

```rust
let mut some_option: Option<&str> = Some("Hello World 👽");
some_option.is_some();
some_option.is_none();
let another_option = some_option.take();
some_option.unwrap();
```

@[2-3](Check if the Option is some or none and return true/false)
@[4](Return Some from the Option and replace it with none)
@[5](Unwrap the Option, yielding Some<T>. Don't do this! Panics if Option was None)

Note: Take nützlich wichtig bei der großen Übung

+++

## Result&lt;T, E&gt;

- For functions that return a value or produce an error.
- Example: writing to a file

```rust 
fn write_to_file(path: &str, msg: &str) -> Result<(), Error> {
    let mut file =  File::create(path)?;
    file.write_all(msg.as_bytes())?;
    Ok(())
}

match write_to_file("file.txt", "TestText") {
    Err(e) => println!({}, e),
    _ => ()
}
```

@[1](We only care if an error occured)
@[2-3](? Operator propagates a possible error to the caller and exits the function)
@[7-10](Print a possible error)

Note: Mögliche Fehler: Datei nicht gefunden, keine schreibrechte,...
Error propagation nur bei Funktionen die Result liefern.

---

# Borrowing & References

Note: Nicht immer sinnvoll Ownership wegzugeben daher borrowing

+++

## References

- point to another value
- non owning
- creating a reference is borrowing
- references cannot outlive their value

Note: 
Referenzen 
+++

## References Examples

```rust 
//this will not compile
let x: &Vec<i32>;
{
    let v = vec![1, 2, 3];
    x = &v;
}
```
@[5-6](Vector v gets dropped while x still holds a reference to the vector)

```rust
//borrowing the result of a function
fn some_fun(x: i32) -> i32 { x + 1 }
let z = some_fun(10);
```

@[8-9](Rust allows you to borrow values of any expression)
Note: 
x könnte implizit typ ableiten, veranschaulichung.
References müssen immer initialisiert werden. (besipiel if let)
References zeigen immer auf einen wert.
Funktionsergebnisse können auch geliehen werden; bzw. alle Ausdrücke (z.B. ergebnis von match).
+++

## Mutable vs. Immutable References

- Any number of immutable references to a value
- Only a single mutable reference to a value at any time

```rust
let mut v = vec![1, 2, 3];
let x = &mut v;
```

Note: 
mutable borrow sehr ausführlich.
Nur mutable variablen können mutable geliehen werden.

+++

## References in Functions

```rust 
fn imm_ref(x: &Vec<i32>) {
    println!("{:?}", x);
}

fn mut_ref(y: &mut Vec<i32>) {
    y.push(2);
    println!("{:?}", y);
}

let v1 = vec![1];
imm_ref(&v1);
let mut v2 = vec![1];
mut_ref(&mut v2);
```

Note:
automatisch dereferenziert :)

---

# Exercise

---

# Lifetimes

Note:
Geben an wie lange ein wert lebt.
Durch fehlenden Garbage collector in Rust nötig.

+++

## Where are Lifetimes necessary?

- In functions that return references
- In structs with references as fields
- Lifetimes help the compiler decide if a reference outlives its value

```rust
fn get_frist(x: &Vec<usize>) -> Option<&usize> {
    if x.len() > 0 {
        Some(&x[0])
    } else {
        None
    }
}
```

Note:
einfaches beispiel; referenz auf ersten integer aus vektor.
Lifetimes klar, da element aus vektor kommt.
Lifetimes werden vom compiler abgeleitet.
Usize nur um array zu indexieren, konsistenz mit nächstem beispiel.
+++ 

## Lifetimes in functions

```rust
fn get_elem<'a, 'b>(x: &'a Vec<usize>, y: &'b usize) 
        -> Option<&'a usize> {
    if x.len() > *y {
        Some(&x[*y])
    } else {
        None
    }
}
get_elem(&Vec::new(), &1);
```

Note: 
Nicht mehr klar wo die rückgabereferenz herkommt.
explizite angabe von lifetimes mit hochkomma.
Dereferenzierung bei vergleich und index nötig, da nicht alle fälle für jeden typen implementiert.
Funktion wird immernoch normal aufgerufen

+++

## Lifetimes in structs

```rust
struct LifeStruct<'a> {
    life_val: &'a i32,
}

struct LifeLifeStruct<'a> {
    life_struct: LifeStruct<'a>,
}

let life_struct = LifeStruct{ life_val: &1 };
let life_life = LifeLifeStruct{ life_struct };
```

Note: 
Immer dann nötig, wenn struct refernz enthält.
Compiler kann prüfen, ob struct variable überlebt.
Würde zu dangling pointer führen.
Shorthand zum zuweisen in array.
Wichtig für zweiten teil der stack übung.
---

# Traits & Generics

---

# Closures

---

# Threads

---

# Exercise
