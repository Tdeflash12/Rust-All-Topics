use rand::Rng;

use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number:");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input ");
        let guess: u32 = match guess.trim().parse(){
Ok(num)=>num,
Err(_)=>{
    print!("Please Enter a valid Input");
    continue;
},
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal =>{
                print!("You Won");
                break;  
            },
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("To Less"),
        }
    }
}
