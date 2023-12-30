use std::sync::Mutex;

pub mod commands;
mod game;

pub struct GameManager {
    mutex: Mutex<Game>
}

pub struct Game {
    pub(crate) board: game::Board
}

impl GameManager {
    pub fn new(game: Game) -> GameManager {
        println!("Creating GameManager");
        GameManager {
            mutex: Mutex::new(game)
        }
    }
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            board: game::Board::new(width, height)
        }
    }

    pub(crate) fn update(&mut self) {
        self.board.update();
    }

    pub(crate) fn reset(&mut self) {
        self.board.reset();
    }

    pub(crate) fn get_board(&self) -> game::Board {
        self.board.clone()
    }
}
