use std::io;

fn main(){
    println!("---Guessing the number Game---");
    println!("Please input of your guess.");

    let mut guess: String  = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);


}
