fn if_else_statement() {
    let number =7;
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is 5 or greater");
    }
}

fn loop_statement() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("Loop stopped at count: {}", count);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn match_statement() {
    let number = 6;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}

fn match_with_enum() {
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    for direction in &directions {
        match direction {
            Direction::North => println!("Heading North!"),
            Direction::South => println!("Heading South!"),
            Direction::East => println!("Heading East!"),
            Direction::West => println!("Heading West!"),
        }
    }
}

fn if_let_statement() {
    let some_option = Some(7);
    if let Some(value) = some_option {
        println!("The value is: {}", value);
    } else {
        println!("No value");
    }
}

fn while_let_statement() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Top of the stack: {}", top);
    }
}

fn break_continue_statement() {
    for i in 1..10 {
        if i == 5 {
            continue; // Skip the rest of this iteration
        }
        if i == 8 {
            break; // Exit the loop
        }
        println!("i: {}", i);
    }
}

fn nested_loops() {
    for i in 1..=3 {
        for j in 1..=3 {
            println!("i: {}, j: {}", i, j);
        }
    }
}

fn main() {
    if_else_statement();
    loop_statement();
    while_loop();
    for_loop();
    match_statement();
    match_with_enum();
    if_let_statement();
    while_let_statement();
    break_continue_statement();
    nested_loops();
}


