//input output library
use std::io;
use rand::Rng;
//declaring main function
fn main() {
    //bro u should know
    println!("Guess the number!");
    // gen random - loca to current thread, seeded by OS
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //again, u know
    loop {
        println!("Please input your guess");
        //making mutable variable
        //calling static new string method
        let mut guess = String::new();
        //using standard input, read the next line of input and write the input to the guess variable (& denotes a reference)
        io::stdin().read_line(&mut guess)
        //same method lowkey
        //expect runs if Result is of type Err
            .expect("failed to read line");
        //shadowing + trim removes newline + parse turns string into number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
    
        };
            

        //curly braces format string
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}