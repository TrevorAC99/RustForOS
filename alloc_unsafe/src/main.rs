use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn example_alloc_single_point() {
    // Create a Point on the stack
    let stack_point = Point::origin();

    unsafe {
        // Create a pointer to a Point on the heap.
        let heap_point: *mut Point = my_alloc(1);
    
        *heap_point = Point::new(3.0, 4.0);
    
        println!("{}", stack_point.distance(&*heap_point));
    
        my_dealloc(heap_point as *mut u8, 1);
    }
}

/// Custom alloc function that makes the current allocation API
/// a bit nicer.
unsafe fn my_alloc<T>(num: usize) -> *mut T {
    let layout =
        Layout::from_size_align(size_of::<T>() * num, align_of::<T>()).expect("Alignment error");
    alloc(layout) as *mut T
}

/// Custom dealloc function that makes the current allocation API
/// a bit nicer.
unsafe fn my_dealloc<T>(ptr: *mut T, num: usize) {
    let layout =
        Layout::from_size_align(size_of::<T>() * num, align_of::<T>()).expect("Alignment error");
    dealloc(ptr as *mut u8, layout)
}

fn example_alloc_multi_points() {
    // Create an array of items allocated on the stack.
    let stack_points = [Point::origin(), Point::new(3.0, 4.0), Point::new(6.0, 8.0)];

    let number_of_elements = stack_points.len();

    // We can limit unsafe code to a specific block and only include
    // the code that depends on unsafe operations.
    unsafe {
        let heap_points_raw: *mut Point = my_alloc(number_of_elements);

        // We can treat the pointer that we explicitly allocated as if it were a slice.
        // Now we just have to make sure that we don't mutate any data held by the raw
        // pointer until we are done using the slice. Otherwise, strange and terrible
        // things could happen.
        let heap_points_slice = std::slice::from_raw_parts_mut(heap_points_raw, number_of_elements);

        for (index, point) in heap_points_slice.iter_mut().enumerate() {
            let index_plus_1 = (index + 1) as f64;
            // Here `*point` is dereferencing a normal Rust mutable reference,
            // not a raw pointer so this line is actually safe code.
            *point = Point::new(3.0 * index_plus_1, 4.0 * index_plus_1);
        }

        for index in 0..number_of_elements {
            let distance = stack_points[index].distance(&heap_points_slice[index]);

            println!(
                "Distance betweens points at index {} is {}",
                index, distance
            );
        }

        my_dealloc(heap_points_raw, number_of_elements);
    }
}

fn main() {
    example_alloc_single_point();

    example_alloc_multi_points();
}
