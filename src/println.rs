fn print_integer() {
    let x = 42;
    println!("The value of x is: {}", x);
}

fn print_float() {
    let y = 3.14;
    println!("The value of y is: {}", y);
}

fn print_string() {
    let s = "Rust";
    println!("The value of s is: {}", s);
}

fn print_multiple_values() {
    let a = 10;
    let b = 20;
    println!("The values of a and b are: {} and {}", a, b);
}

fn print_debug() {
    let arr = [1, 2, 3];
    println!("The array is: {:?}", arr);
}
fn print_named_parameters() {
    let name = "Alice";
    let age = 30;
    println!("Name: {name}, Age: {age}", name = name, age = age);
}

fn print_with_formatting() {
    let number = 42.6789;
    println!("Formatted number: {:.2}", number); // prints 42.68
}

fn print_with_padding() {
    let number = 42;
    println!("Padded number: {:05}", number); // prints 00042
}

fn print_binary_hex() {
    let number = 255;
    println!("Binary: {:b}, Hex: {:x}", number, number); // prints Binary: 11111111, Hex: ff
}

fn print_escaped_characters() {
    println!("This is a newline character: \\n");
    println!("This is a tab character: \\t");
}

fn print_raw_string() {
    let raw_str = r"This is a raw string with no escape: \n \t";
    println!("{}", raw_str);
}
fn main() {
    println!("Hello, Jaya!");
    print_integer();
    print_float();
    print_string();
    print_multiple_values();
    print_debug();
    print_named_parameters();
    print_with_formatting();
    print_with_padding();
    print_binary_hex();
    print_escaped_characters();
    print_raw_string();
}
