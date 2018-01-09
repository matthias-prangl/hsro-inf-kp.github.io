# Rust Basics

Rust is a systems programming language developed by Mozilla.
Rust aims to be a safe concurrent an practical programming language.

## Basic Syntax

Variables, Tuples, Functions, Blocks, Keywords (let, mut...), Macros

## Structs

Struct syntax, Tuple structs, unit-like structs

## Methods

Methods of structs

## Traits

traits are like interfaces (C++, Java), how do they add to safety?

# Type Safety 

A _type safe_ language can check if a written program is _well defined_.
_Well defined_ programs cannot exhibit undefined behavior. 

While programs in C or C++ can be well definde the compiler does not assure this. 
Consider a C program which tries to access an element in an array that is out of the bound of the array:

```c
int main (void) {
    unsigned long x[1];
    x[3] =  0x7ffff7aaaaaa;
}
```
This program does compile. 
But since `x[3]` tries to access a element out of the bounds of the array the behavior is not defined.

In a type safe language like rust a similar program as shown below would not try to access an element outside of the arrays bounds.

```rust
fn main() {
    let mux x: [u64; 1] = [123];
    x[4] = 0x7ffff7aaaaaa;
}
```

While this program also compiles it `panics` at runtime.
Since a panic is the expected behavior in this particular case this program is well defined.
The resulting Error:

```bash
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 4', src/main.rs:3:5
stack backtrace:
```

## Error Handling

# Memory Safety

What is it and how does rust guarantee it? (borrowing, ownership), comparison to other languages

## Borrowing and Ownership

Lifetime in Structs, Functions, ...

## Smart pointers

Box<T>, Rc<T>, Arc<T>

RefCell<T>?

## Threads

Closures, ownership in threads
