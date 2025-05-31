use console_lib::Console;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;
use crate::game::Game;

pub mod game;
pub mod collections;

fn main() -> ExitCode {
    let console = Console::new().unwrap();

    let game = Game::new(&console);
    let mut game = match game {
        Ok(game) => game,
        Err(err) => {
            drop(console);

            eprintln!("{err}");

            return ExitCode::FAILURE;
        },
    };

    loop {
        if game.update() {
            return ExitCode::SUCCESS;
        }

        sleep(Duration::from_millis(40));
    }
}
