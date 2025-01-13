// Function demonstrating stack memory usage
fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

// Function demonstrating heap memory usage
fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

// Function demonstrating dynamic memory management (modifying a heap-allocated string)
fn update_string() {
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    // Appending more text to a heap-allocated string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
}

fn main() {
    // Demonstrating stack allocation
    let x = 42;
    println!("Stack allocated value: {}", x);

    // Demonstrating heap allocation
    let y = Box::new(42);
    println!("Heap allocated value: {}", y);

    // Demonstrating ownership and borrowing with heap allocation
    let z = Box::new(100);
    let z_ref = &z;
    println!("Heap allocated value with reference: {}", z_ref);

    // Demonstrating ownership transfer (moving data)
    let z_moved = z;
    println!("Heap allocated value after move: {}", z_moved);
    // println!("Original value after move: {}", z); // Uncommenting this line will cause a compile-time error
    println!();

    // Call functions that demonstrate stack, heap, and dynamic memory manipulation
    stack_fn();   // Function demonstrating stack memory usage
    heap_fn();    // Function demonstrating heap memory usage
    update_string(); // Function demonstrating a dynamic change in size
}

