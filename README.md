# Rust

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
The following example shows how a tuple-like struct with two elements (i32, u32) is defined:

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

## Ownership

Every value in Rust has a single owner.
This owner determines the lifetime of the value.

Take for example this piece of code: 
```rust
struct Car { model: String, year: u16 }

fn main() {
    let mut cars = vec![ //car get allocated
        Car{model: "A4".to_string(), year: 2006}, 
        Car{model: "Clio".to_string(), year: 1998} ];
} //cars gets dropped
```
Since each value has a singe owner, in this example each `Car` owns its fields which in turn own their values.
`cars` owns a vector which owns its elements of type `Car`.
As soon as the vector leaves the scope every value associated with `cars` is _dropped_.
Dropping a value means the memory associated with this value is freed.


### Passing ownership with move
If you reassign a value to another variable the value doesn't get copied like for example in C++.
In Rust the ownership of that value is _moved_ to the new variable.
This, in turn means that the source of the value becomes uninitialized.
Using the now unintialized variable results in a compiler error with a hint that the value is used after it has been moved.
In the following example we try to reassign a Box twice, which results in an error:

```rust 
    let a = Box::new((123, 321));
    let b = a;  //compiler hint: value moved here
    let c = a;  //compiler error: value used here after move
```

Moving the ownership of values to other variables is much cheaper than doing a deep copy of the values.
It is also very clear to the program when it can clear the associated memory of a variable.
To truly copy a value you have to explicitly copy them. 
In the example above the compiler error could be solved by calling `a.clone()` instead of a simple assignment.

### Moving on: moves in functions
Moves don't only occur if you assign a value to a variable. 
Values are also moved if they are passed as a parameter to a function.
If you passed a variable to a function this variable is now uninitialized. 

```rust
fn do_sth(s: String) { }

let x = "I better get moving!".to_string();
do_sth(x);
println!("{}", x);  // compiler error: value used here after move

```

In the example `x` gives up its ownership of the String _I better get moving!_ and passes it to the function `do_sth`.
Trying to print the value of `x` after the move occured results in a compiler error.
You have to consider this especially if you try to call a function in a loop.
Unless you assign a new value to the variable after the function call this will certainly result in the same error.

A function can also return ownership of a value it owns.
For example a function `fn count_words(s: String) -> (String, u32) {...}` counts the words in a string and returns the ownership to a tuple containing the input string and the wordcount. 

### Types that don't move: Copy Types

The prevous examples have shown how ownership gets implicitly moved to a new owner.
Maybe you have noticed that the data types in the examples were rather complex.
A `Copy` type can be every type that doesn't need any special handling in case of the associated values being dropped.
Assigning a `Copy` type to several variables creates copies of the values.
The only types that can be of `Copy` are those which can be copied bit by bit.
This includes compound types like arrays an tuples but only if the contained values are `Copy`.

```rust
let x: i32 = 5;
let y = x;  //value of variable x is copied to y
let z = x;  //value of variable x is copied to z
```

The example shows the reassignment of the value stored in `x` to `y` and `z`.
This does not result in an error even though there is no explicit call to `.clone()`.

## References and Borrowing

Giving the ownership of a value away at a function call is often not what you want.
You may have a function to print the content of a structure in a formatted way.
The value would not be usable after the function call unless you pass the ownership back in a rather clunk way like `x = print_x(x);`.
This would also require x to be mutable since it gets assigned a new value.

To keep ownership of a value at the variable you can _borrow_ the value to the function.
To borrow something you need to pass the _reference_ to the variable you'd like to borrow.

```rust 
let b = Box::new(123);
let c = &b;
let d = &b; //no error since the value is borrowed
```

In the example a box containing an integer is created, the ownership of the box is passed to the variable `b`.
Other variables can borrow this box by referencing it with `&`.
This a an example for an _immutable reference_. 
Rust lets you create any number of immutable references to a value as long as you guarantee not to modify the source.
For this reason _mutable references_ are rather verbose in the code:

```rust 
fn mut_fun(b: &mut Box<i32>) {}

let mut b = Box::new(123);
let c = &b;
mut_fun(&mut b);
```

Apart from the obvious requirement of being a mutable variable you also have to explicitly state that you pass a mutable reference `&mut` to a function.
The example will produce a compiler error, since the code tries to pass a mutable reference while a immutable reference is active on the same value.

## Smart pointers

Box<T>, Rc<T>, Arc<T>

RefCell<T>?

## Threads

Closures, ownership in threads
