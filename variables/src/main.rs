fn main() {
    let x = 5;
    let x: i32 = x + 1;
    let y: i32 = five();
    let z = if y < 5 { 4 } else { 6 };
    
    println!("Value of x: {x}");
    println!("Value of z: {z}");

    another_function(y);
    branch_function();

    let s = String::from("hello world");

    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {word}"); // doesn't compile if we use s.clear() -> no reference anymore for word
}

fn another_function(x: i32) {
    println!("Value received: {x}");
}

fn five() -> i32 {
    5
}

fn branch_function() {
    let number = five();

    if number < 5 {
        println!("< 5");
    } else {
        println!(">= 5");
    }
}

// loop, while, for

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}