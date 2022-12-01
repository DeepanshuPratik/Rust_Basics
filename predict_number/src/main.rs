use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess : i32 = guess.trim().parse().expect("Enter the number!"); 
    println!("The secret number is {secret_number}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    if guess == secret_number {
        println!("you got it right");
    } else { println!("you got it wrong! better luck next time!"); }

}
