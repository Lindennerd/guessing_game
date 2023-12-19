use rand::Rng;
use std::{cmp::Ordering, io};

use crate::player::Player;

pub struct Game {
    secret_number: u32,
    guesses_count: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            secret_number: rand::thread_rng().gen_range(1..=100),
            guesses_count: 0,
        }
    }

    pub fn start(mut self, player: &mut Player) {
        player.add_new_game();
        println!("Guess the number: ");

        loop {
            let guess = Game::ask_user_for_guess();

            self.guesses_count += 1;
            println!("You guessed: {guess}");

            match guess.cmp(&self.secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    player.add_new_win(guess);
                    player.add_new_score(self.guesses_count);
                    println!("You win with {} guesses!", self.guesses_count);
                    break;
                }
            }
        }
    }

    pub fn ask_user_for_guess() -> u32 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        guess.trim().parse().expect("Please type a number!")
    }

    pub fn ask_user_for_name() -> String {
        println!("Please input your name.");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("failed to read line");

        name
    }
}
