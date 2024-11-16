use std::io;

fn main() {
    println!("Enter the first number:");
    let mut num1 = String::new();//allocating memory for a new string that is empty
    io::stdin().read_line(&mut num1).unwrap(); //unwrap is for when no value is entered
    let num1: f64 = num1.trim().parse().unwrap(); //trim is for removing leading and trailing whitespace. Parse is for converting
                                                  //to a different type

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();


    println!("Choose an operation: +, -, /, *");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();


    let result = match operation.trim() { //match is used for pattern matching aka switch statement in Rust
      "+" => num1 + num2,
      "-" => num1 - num2,
      "/" => num1 / num2,
      "*" => num1 * num2,
      _ => {
        println!("Invalid operation!");
        return;
      }
    };

    println!("Result: {}", result);
}
