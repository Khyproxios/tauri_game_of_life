use std::sync::Mutex;

pub mod commands;

pub struct GameManager {
    mutex: Mutex<Game>
}

pub struct Game {
    value: usize
}

impl GameManager {
    pub fn new(game: Game) -> GameManager {
        GameManager {
            mutex: Mutex::new(game)
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            value: 0
        }
    }

    pub fn increase(&mut self) {
        self.value = if self.value == 100 {
            0
        } else {
            self.value + 1
        }
    }

    pub fn decrease(&mut self) {
        self.value = if self.value == 0 {
            100
        } else {
            self.value - 1
        }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }
}
