use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("--------- Guess the number ---------");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut try_take: u32 = 0;

    loop {
        println!("--------- Input your guess");
        try_take = try_take + 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("--------- Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("--------- You Guess is correct");
                break;
            }
            _ => {
                println!("--------- Retry");

                let wrong =
                    ((secret_number as f64 - guess as f64).abs() / secret_number as f64) * 100.0;

                let correct = 100.0 - wrong;

                println!("Wrong {wrong} %");
                println!("Correct {correct} %");
            }
        }
    }

    println!("--------- Taken try {try_take}");
}
