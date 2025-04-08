use std::io;

fn main() {
 println!("Guess a number!");

 println!("Kindly input a number:");

 let mut input = String::new();

 io::stdin()
 .read_line(&mut input)
 .expect("Failed to read line");

 println!("Your guess is: {}", input);
}
