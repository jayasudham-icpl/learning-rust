// Function to demonstrate mutability with function parameters
fn modify_value(mut z: i32) -> i32 {
    z += 5;
    z
}

// Function to demonstrate immutability with function parameters
fn display_value(a: i32) {
    println!("The value of a is: {}", a);
}

// Function to demonstrate mutability with references
fn modify_reference_value(z: &mut i32) {
    *z += 10;
}

fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The initial value of y is: {}", y);

    // Changing the value of the mutable variable
    y = 20;
    println!("The new value of y is: {}", y);

    // Calling functions
    let modified_value = modify_value(y);
    println!("The modified value of y is: {}", modified_value);

    // Calling function to display value of x
    display_value(x);

    // Calling function to modify value by reference
    modify_reference_value(&mut y);
    println!("The value of y after modify_reference_value is: {}", y);

}
