extern crate hyper;
extern crate rustc_serialize;

mod game;

use std::io::stdin;

fn main() {
    loop  {
        println!("RUST TRIVIA \n");
        println!("OPTIONS: ");
        println!("1- START ");
        println!("2- EXIT \n");
        let mut input = String::new();
        stdin().read_line(&mut input).ok().expect("failed to read stdin");
        match input.trim().parse::<u32>() {
            Err(_) => {
                println!("please insert a valid option");
                continue;
            }
            Ok(value) => match value {
                1 => {
                    let mut game = game::Game::new();
                    loop {
                        let question = game.new_question().ok().expect("could not parse question");;
                        println!("score: {}", &game.score);
                        println!("question: \n {}", &question.title);
                        print!("Answer: ");
                        let mut input = String::new();
                        stdin().read_line(&mut input).ok().expect("failed to read stdin");
                        if game.verify_question_answer(&input).unwrap() {
                            game.score += question.value;
                            continue;
                        } else {
                            println!("wrong answer, game over !");
                            break;
                        }
                    }
                },
                2 => break,
                _ => {
                    println!("please insert a valid option");
                    continue;
                }
            }
        };
    }
}
