fn main() {
    // println!("Hello, world!");
    use::std::io;
    use::rand::Rng;

    println!("Guess a number");
    let mut my_guess = String::new();
    // let generated_number = rand::random::<u32>() % 100;
    let generated_number = rand::thread_rng().gen_range(1..=100);
    println!("Input your guess:");
    io::stdin().read_line(&mut my_guess).expect("Could not read the input");
    let my_guess: u32 = my_guess.trim().parse().expect("Please type a number!");
    println!("Generated number is: {}", generated_number);
    println!("Your guess is: {}", my_guess);
}
