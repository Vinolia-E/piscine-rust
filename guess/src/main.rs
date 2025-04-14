fn main() {
    // println!("Hello, world!");
    use::std::io;
    // use::rand;

    println!("Guess a number");
    let mut my_guess = String::new();
    println!("Input your guess:");
    io::stdin().read_line(&mut my_guess).expect("Could not read the input");
    let my_guess: u32 = my_guess.trim().parse().expect("Please type a number!");
    println!("Your guess is: {}", my_guess);
}
