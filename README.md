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

Structures (`struct`) assemble multiple values of possibly different types into one value.
A `struct` in Rust can have on of three types: _named-field_, _tuple-like_ and _unit-like_.
The Rust convention for naming structs is _CamelCase_.

The most basic type is the unit-like struct.
This is a struct with no elements that does not occupy any memory.
Since those types of structs are only useful in specific use cases they are not further explained at this point.

A tuple-like struct resembles a tuple.
Those structs are useful for definig new types to achieve stricter type checking.
Instead of commenting what a tuple contains, the name of the tuple-like struct can be self explaining and also be checked by the compiler.
The following example shows how a tuple-like struct whit two elements (i32, u32) is defined:

```rust
struct TupleStruct(i32, u32);
```

Named-field structs assign a name to each element in the struct.
Those the named elements are called _fields_.
The following example shows how to define a struct `Student`:

```rust
struct Student {
    last_name: String,
    first_name: String,
    id: u32,
    major: String,
    semester: u8
}
```

Construct a value of the defined type like this:

```rust
    let last_name = "Doe".to_string();
    let first_name = "John".to_string();
    let s = Student { 
        first_name, 
        last_name, 
        id: 1234, 
        major: "INF-M".to_string(), 
        semester: 5
    };
```

Note the used shorthand to assign the `first_name` and `last_name` fields.

### Methods in Structs



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

## Ownership and Borrowing

Lifetime in Structs, Functions, ...

## Smart pointers

Box<T>, Rc<T>, Arc<T>

RefCell<T>?

## Threads

Closures, ownership in threads
