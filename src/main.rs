use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("---Guessing the number Game---");
    
    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    loop {
        println!("Please input of your guess.");
    
        let mut guess: String  = String::new();

        io::stdin()
        //just to get the user input as mutable variable
            .read_line(&mut guess)
            .expect("Failed to read line");
        //println!("the secret number is: {secret_number}.");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}" );

        // generating a secret number
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
        }
    
    }
    }
    


}
