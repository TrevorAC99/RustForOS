# RustForOS

These are some examples of Rust code showcasing features that might be useful if Rust were to
replace C in the Operating Systems class at the University of Wisconsin Oshkosh. Currently,
`allocate`, `alloc_unsafe`, and `concurrency` have code examples, the other projects are hello world placeholders for now.

## `allocate`

This example uses a stack allocated Point struct and a heap allocated Point struct (a `Box<Point>`) and then finds
the distance between those two points on a 2D place. It then uses an stack allocated array of Points
along with a heap allocated array of Points (a `Vec<Point>`) of the same length and finds the distances between Points
at the same index in two collections.

This example utilizes Rust's nice allocation features such a `Box<T>` for allocating a single item on the heap and `Vec<T>`
for allocating a variable number of items on the heap. These work by internally storing a pointer to allocated memory
which is deallocated when the `Box` or `Vec` is dropped. Data in Rust is dropped when the variable that owns that data
goes out of scope. This allows the compiler to statically determine when memory needs to be allocated and freed while also
allowing the programmer to not have to worry about explicitly calling `malloc()` or `free()` like one would in C.

## `alloc_unsafe`

This example does the same thing as the `allocate` example but works with raw pointers rather
than using Rust's `Box<T>` and `Vec<T>` like you would in normal use.

## `concurrency`

This example is an implementation of project 3 from the OS class. I implemented this project to show Rust's way of using mutexes, condition variables, and threads since I already had the requirements for it and it nicely demonstrates how these things are used. Implementing this project in Rust is a great demonstration of Rust's concurrency benefits because the lack of unsafe Rust means that if it compiles, it is guaranteed to be free of race conditions.
