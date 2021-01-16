# alloc_unsafe

This is a different implementation of the example shown in the allocate project except
that this version doesn't use any of Rust's nice features for heap allocation. This is
similar to what you would write if you were doing this in C. While there are cases where
you may need to write Rust that handles raw pointers, most of the time you will be fine
with things provided by the standard library or other crates (Rust packages/libraries).
An example of where code like this might be found is in the implementation of something
like the `Box<T>` or `Vec<T>` found in Rust's standard library.
