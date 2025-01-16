fn main() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred

    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;
    
    println!("{}", s1); //There can be many immutable references at the same time
    println!("{}", s2);
    println!("{}", s3);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
} 