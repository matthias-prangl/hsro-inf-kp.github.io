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
- Exercise

+++

## Agenda

- Lifetimes
- Traits and Generics
- Smart Pointers
- Closures
- Threads
- Exercise

---

# Rust Basics

---

# Cargo

<div class="twocolumn">
  <div>
    - Rust package manager </br>
    - Invokes rustc to compile </br>
    - Creates packages
  </div>
  <div>
    <img src="https://raw.githubusercontent.com/matthias-prangl/rust/master/assets/cargo_logo.png" alt="cargo">
  </div>
</div>

Note:
Packages availiable at crates.io.
Rust eher bare bones, daher viele crates verfügbar
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
    -  Some value of type T (Some&lt;T&gt;)
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

match maybe_return_something(true) {
    Some(val) => println!("I got: {}", val),
    None => println!("I got nothing!"),
}
```

@[8-11](Match on the returned Option and do something with the result)
+++

## Methods for Option&lt;T&gt;

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
() ist leerer typ, wenn result nicht interessant.
Hinweis auf expect() liefert ok, panic bei E.

---

# Memory Safety

---

# Ownership

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
let z = &some_fun(10);
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

@[1-3, 10-11](Immutable reference as parameter)
@[5-8, 12-13](Mutable reference as parameter, very verbose)

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
fn get_frist<'a>(x: &'a Vec<usize>) -> Option<&'a usize> {
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
Lifetimes werden vom compiler abgeleitet, können weggelassen werden.
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
Dereferenzierung bei vergleich und index nötig, da nicht alle fälle für jeden typen implementiert (&i32 == i32, &mut i32 == i32, ....) einfacher dereferenzierung zu verlangen.
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
Auch dann nötig, wenn struct mit lifetime feld ist.
Compiler kann prüfen, ob struct variable überlebt.
Würde zu dangling pointer führen.
Shorthand zum zuweisen in array.
Wichtig für zweiten teil der stack übung.
---

# Traits & Generics

+++

## Traits

- Zero cost abstraction
- _Interfaces_ in Rust
- Statically dispached
- Dynamically dispached
- Used for generics
- Can be added to **any** type

Note: 
ZCA: geschwindigkeit, speicher, gleich wenn abstrahiert.
Interfaces aus Java bekannt.
Static: Generics werden wegkompiliert; laufzeit verbessert.
Dynamic: für dinge wie button callbacks (u.U. variable größe) wirds abstrakt behalten.
Trait bounds geben an welche Traits erwartet werden.
Kann auch zu z.B. i32 hinzugefügt werden.

+++

## Trait Example

```rust 
struct Fib { next: i32, curr: i32, }

impl Iterator for Fib {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
```
@[1](Definition of a new struct)
@[3](Implementation block for the Iterator trait)
@[4-5](Required items to implement the iterator)

Note:
Fib kann nun alle Iterator methoden ausführen.
Darunter take, umwandlung in peekable, ...

+++

## Using Traits

```rust
trait AwesomeTrait {
    fn default_method(&self) { println!("Wow!"); }
    fn required_method(&self);
}

impl AwesomeTrait for i32 {
    fn required_method(&self) {
        println!("I am special!");
    }
}

fn generic_fun<T: AwesomeTrait>(y: T) {
    y.default_method();
    y.required_method();
}
```

Note:
Traits mit default methoden bereits implementiert.
Andere methoden müssen von implementierenden typen implementiert werden.
Einfach für bestehende Datentypen implementierbar.
Trait objects auch möglich, meist geboxed, wg. variabler größe.
+++

## Trait bounds

- Define which traits a parameter has to implement
- Multiple traits possible

```rust
use std::fmt::Debug;
fn trait_fun<'a, 'b, T: Debug + Iterator, U: Iterator, V: std::ops::Add>(w: T, x: &'a U, y: &'b U, z: V) -> &'b U
{ y }

fn trait_fun<'a, 'b, T, U, V>(w: T, x: &'a U, y: &'b U, z: V) -> &'b U
    where T: Debug + Iterator, U: Iterator, V: std::ops::Add 
{ y }
```

@[5](Define the trait bounds after the return value to make things more clear)

Note: Verschiedene Notationen, damit parameter und reutrn nicht zu weit weg rücken.

Bei simplen implementierungen nicht nötig.
Unübersichtlich bei vielen generics.
---

# Smart Pointers

+++

## Why though?

- Explicitly allocate a value on the heap
- Shared ownership (across threads)
- Added indirection

Note:
Referenzen sind quasi schon Smarte pointer wie in C++.
Durch Ownership gestärkt.

+++

## Box&lt;T&gt;

```rust
struct RecStruct {
    rec: RecStruct,
}

struct ActualRecStruct{
    rec: Box<ActualRecStruct>,
}

let r = ActualRecStruct{ rec: Box::new(ActualRecStruct { rec: ... })};

fn do_something(v: Vec<i32>) { }
let v = Box::new(Vec[123, 234]);
do_something(*v)
```

@[1-3](Doesn't work, RecStruct has infinite size on the stack)
@[5-7](Added indirection through Box, size on stack is known)
@[5-9](Actually still infinite size, put the Box in an Option!)
@[11-13](Dereference a box using *)

Note: Dereferenzierung mit stern, normalerweise nicht benötigt aber z.b. bei zuweisung

+++

## Rc&lt;T&gt;

- Reference counted smart pointer
- Share immutable data
- Value dropped if no more reference to value

```rust
use std::rc::Rc;

let x; 
{
    let y = Rc::new("Share me");
    x = Rc::clone(&y);
    println!("{}", Rc::strong_count(&x));
}
println!("{}", Rc::strong_count(&x));
println!("{}", x);
```

@[1](Bring Rc in scope)
@[7-9](Get owning references count)
@[3-10](Would not be possible without Rc. y is dropped after inner scope)

Note: definiere y als rc.
Explizites aufrufen von rc::clone.
Weak count wäre auch möglich; non owning, wird None wenn dropped.
Normale Referenzen: borrowed value does not live long enough.
Mutable Rc auch möglich durch cell, geht hier zu weit.
Arc auch verfügbar (atomic rc), für multithreading, rc mit overhead (langsamer)

---

# Closures

+++

## Closures

- Anonymous functions
- A little different in Rust
- Can move, borrow or mutate values

Note: sollte man schon aus java, c++, python etc. kennen

+++

## Closure examples

- Three different kinds of closures
- Can be used as parameters in functions

```rust 
let mut v = vec!["Hello", "closure!"];
let closure = || println!("{}", v[0]);
let closure_once = move || {
        for i in v { println!("{}", i) }
    };
let mut closure_mut = || v.push("Hi!");

fn closure_fun<F>(c: F) where F: FnOnce()->() { c(); c(); }
```

@[2](Fn()->() can be called any number of times)
@[3-5](FnOnce()->() can only be called once)
@[6](FnMut()->() mutates the value in the closure)
@[8](This will not work, since FnOnce can only be called once)

Note: Fn, FnOnce, FnMut sind automatisch implementierte traits
Beispiel kompiliert NICHT.
move könnte weggelassen werden, explizite übertragung des ownership.
v muss zwingend mut sein wenn closure_mut funktionieren soll.
Auch als Funktionsparameter möglich.
Jede Fn ist FnMut ist FnOnce.

---

# Threads

Note: gibts auch.
Komfortable möglichkeiten für mutexe, channels, atomic reference counted immutable variablen

---

# Exercise
