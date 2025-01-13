fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid

    println!("s2: {}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone(); // Use `.clone()` to create a new String from s3

    println!("s4: {}", s4); // s3 is invalid because it was moved to s4

    let s5 = String::from("Rust");
    let s6 = s5; // s5 is moved to s6, s5 is no longer valid

    println!("s6: {}", s6);

    let s7 = String::from("Programming");
    let s8 = s7.clone(); // Use `.clone()` to create a new String from s7

    println!("s7: {}, s8: {}", s7, s8);

    let s9 = String::from("Cloning");
    let s10 = s9.clone(); // s9 is cloned to s10, both are valid

    println!("s9: {}, s10: {}", s9, s10);

    let s11 = String::from("Ownership");
    takes_ownership(s11); // s11 is moved to the function, s11 is no longer valid

    // println!("s11: {}", s11); // This line is intentionally commented out as it would cause a compile error.

    let s12 = String::from("Borrowing");
    let len = calculate_length(&s12); // s12 is borrowed, not moved

    println!("The length of '{}' is {}.", s12, len);

    let mut s13 = String::from("Mutable");
    s13 = takes_and_returns_ownership(s13); // s13 is moved to the function and then returned
    println!("s13: {}", s13);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

fn takes_and_returns_ownership(some_string: String) -> String {
    println!("{}", some_string); // `some_string` now owns the data.
    some_string // Return ownership to the caller
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
