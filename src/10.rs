use std::io;

fn main() {
    let mut num = String::new();

    println!("Please enter a number: ");

    io::stdin().read_line(&mut num)
        .expect("Failed to read line");
    
    let num: i32 = num.trim().parse()
        .expect("Please enter a valid number");

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
