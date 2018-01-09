# Rust Basics

Rust is a statically typed system programming language developed by Mozilla with a focus on safety while still maintaining high performance.
The main goals of Rust are to prevent segmentation faults and guaranteeing thread safety.

This blog will give you an overview over the language and addresses some of the type and memory safety features Rust provides.

## Basic Syntax

Variables, Tuples, Functions, Blocks, Keywords (let, mut...), Macros

Variables in Rust are by default immutable. To decalre a mutable variable the `mut` keyword must be used:

```rust
let var = 1;            //<= an immutable variable
let mut var = 2;        //<= a mutable variable
let var_u32: u32 = 1;   //<= a 32-bit unsigned integer
```

Since Rust is statically typed, the information about the data types must be availiable at compile time.
In the example above the data types are implicitly inferred as `i32` - a 32-bit integer.
The example also shows _shadowing_. _Shadowing_ allows you to reuse a variable name for a new value. 
`var_u32` shows a variable with explicit type declaration to `u32`.

Unlike variables **all** types for a function must be provided.
Since all the types are named you almost never need to provide a type anywhere else.

```rust
fn sum_fun(x: i32, y: i32) -> i32 {
    x + y
}
```

The function `sum_fun` takes two arguments of type `i32` and returns the sum of those two arguments as a `i32`.
Note that there is no trailing semicolon at the end of the function body. 

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
