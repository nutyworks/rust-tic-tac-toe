mod game;

use std::{io::Write, str::FromStr};

use game::Point;

use self::game::PlaceError;

pub fn start_game() {
    let mut game = game::Game::new();

    let winner = loop {
        match make_turn(&mut game) {
            Ok(_) => match game.get_result() {
                game::GameResult::Win(player) => break Some(player),
                game::GameResult::Draw => break None,
                game::GameResult::Undetermined => game.proceed(),
            },
            Err(PlaceError::Taken) => {
                println!("That place is already taken.");
            }
        }
    };

    println!("{}", game.to_string());
    if let Some(winner) = winner {
        println!("{} won the game!", winner);
    } else {
        println!("It was draw.");
    }
}

fn make_turn(game: &mut game::Game) -> Result<(), PlaceError> {
    println!("{}", game.to_string());
    println!("{}'s turn.", game.get_turn());
    let point = get_point();

    game.place(&point)
}

fn get_point() -> Point {
    loop {
        let mut buf = String::new();

        print!("> ");
        std::io::stdout().flush().expect("Cannot flush stdout");
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Cannot read from stdin");

        if let Ok(point) = Point::from_str(&buf.trim()) {
            break point;
        } else {
            println!("Cannot parse to the point. Please try again.");
            continue;
        }
    }
}
