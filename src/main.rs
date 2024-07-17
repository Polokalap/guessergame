use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

        loop {

        let a = rng.gen_range(1..11);

        println!("Guess between 1 and 10");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let iinput: i32 = input.trim().parse().expect("owo");

        if a == iinput {
            println!("You won owo the number was {a}.");
        }

        else {
            println!("You lost. The number was {a}.");
        }
    }
}
