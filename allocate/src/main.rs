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

    // Create a boxed Point, essentially a heap-allocated Point.
    // The memory used for the point is "owned" by the Box<Point> stored in
    // heap_point. Unless the ownership of that Box is changed, it will be
    // dropped when heap_point goes out of scope.
    let heap_point = Box::new(Point::new(3.0, 4.0));

    println!("{}", stack_point.distance(heap_point.as_ref()));

    // At this time, heap_point is going out of scope so Rust will statically
    // call the drop method of the Box which will handle freeing of the memory.
}

fn example_alloc_multi_points() {
    // Create an array of items allocated on the stack.
    let stack_points = [Point::origin(), Point::new(3.0, 4.0), Point::new(6.0, 8.0)];

    let number_of_elements = stack_points.len();

    // In Rust, Vec is the standard growable array type that is provided by the standard library.
    // Think of it like Java's ArrayList. This is just a convenient way of iterating over a range,
    // mapping those values into Point structs, and then collecting those Points into a Vec<Point>
    let heap_points: Vec<Point> = (0..number_of_elements)
        .map(|index| {
            let index_plus_1 = (index + 1) as f64;
            Point::new(3.0 * index_plus_1, 4.0 * index_plus_1)
        })
        .collect();

    for index in 0..number_of_elements {
        let distance = stack_points[index].distance(&heap_points[index]);

        println!(
            "Distance betweens points at index {} is {}",
            index, distance
        );
    }

    // Just like for the Box in the `example_alloc_single_point` function,
    // the Vec owns the heap memory used by its contents so when the Vec is
    // dropped, it will take care of deallocating its memory.
}

fn main() {
    example_alloc_single_point();

    example_alloc_multi_points();
}
