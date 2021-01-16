use std::alloc::{alloc, dealloc, Layout};

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

unsafe fn example_alloc_single_point() {
    // Create a Point on the stack
    let stack_point = Point::origin();

    // Create a pointer to a Point on the heap.
    let heap_point: *mut Point = alloc(Layout::new::<Point>()) as *mut Point;

    *heap_point = Point::new(3.0, 4.0);

    println!("{}", stack_point.distance(&*heap_point));

    dealloc(heap_point as *mut u8, Layout::new::<Point>());
}

unsafe fn example_alloc_multi_points() {
    // Create an array of items allocated on the stack.
    let stack_points = [Point::origin(), Point::new(3.0, 4.0), Point::new(6.0, 8.0)];

    let number_of_elements = stack_points.len();

    let layout = Layout::from_size_align_unchecked(
        std::mem::size_of::<Point>() * number_of_elements,
        std::mem::align_of::<Point>(),
    );

    let heap_points_raw: *mut Point = alloc(layout) as *mut Point;

    let heap_points_slice = std::slice::from_raw_parts_mut(heap_points_raw, number_of_elements);

    for (index, point) in heap_points_slice.iter_mut().enumerate() {
        let index_plus_1 = (index + 1) as f64;
        // Here `*point` is dereferencing a normal Rust mutable reference, not a raw pointer so this line is actually safe code
        *point = Point::new(3.0 * index_plus_1, 4.0 * index_plus_1);
    }

    for index in 0..number_of_elements {
        let distance = stack_points[index].distance(&heap_points_slice[index]);

        println!(
            "Distance betweens points at index {} is {}",
            index, distance
        );
    }

    drop(heap_points_slice);

    dealloc(heap_points_raw as *mut u8, layout);
}

fn main() {
    unsafe {
        example_alloc_single_point();

        example_alloc_multi_points();
    }
}
