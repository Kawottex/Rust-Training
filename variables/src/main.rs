fn main() {
    let x = 5;
    let x: i32 = x + 1;
    let y: i32 = five();
    let z = if y < 5 { 4 } else { 6 };
    
    println!("Value of x: {x}");
    println!("Value of z: {z}");

    another_function(y);
    branch_function();
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