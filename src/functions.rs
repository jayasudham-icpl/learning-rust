// Function to greet
fn greet() {
    println!("Hello, world!");
}

// Function with parameters
fn add (a:i32,b:i32)->i32{
    a+b
}

// Function with a return value
fn square(x: i32) -> i32 {
    x * x
}

// Recursive function
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Function with multiple parameters
fn multiply(a: i32, b: i32, c: i32) -> i32 {
    a * b * c
}

// Function with a string parameter
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with a tuple parameter
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Function with an array parameter
fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    greet();

    // Using function with parameters
    let sum = add(5, 3);
    println!("Sum: {}", sum);

    // Using function with return value
    let sq = square(4);
    println!("Square: {}", sq);

    // Using recursive function
    let fact = factorial(5);
    println!("Factorial: {}", fact);


    // Using function with multiple parameters
    let product = multiply(2, 3, 4);
    println!("Product: {}", product);

    // Using function with a string parameter
    greet_person("Alice");

    // Using function with a tuple parameter
    let point = (3, 5);
    print_coordinates(&point);

    // Using function with an array parameter
    let numbers = [1, 2, 3, 4, 5];
    let total = sum_array(&numbers);
    println!("Sum of array: {}", total);   
}