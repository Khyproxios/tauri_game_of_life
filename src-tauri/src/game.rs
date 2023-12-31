#[derive(Clone, serde::Serialize)]
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

#[derive(serde::Deserialize)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells: Vec<bool> = vec![false; width * height];

        Board {
            width,
            height,
            cells
        }
    }

    pub fn clone(&self) -> Board {
        Board {
            width: self.width.clone(),
            height: self.height.clone(),
            cells: self.cells.clone()
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn next_state(&self, x: usize, y: usize, is_alive: bool) -> bool {
        let mut count = 0;

        let valid_left_x = x > 0;
        let valid_right_x = self.width - 1 > x;
        let valid_top_y = y > 0;
        let valid_bottom_y = self.height - 1 > y;

        if valid_left_x && valid_top_y {
            let index = self.get_index(x - 1, y - 1);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_right_x && valid_top_y {
            let index = self.get_index(x + 1, y - 1);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_left_x && valid_bottom_y {
            let index = self.get_index(x - 1, y + 1);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_right_x && valid_bottom_y {
            let index = self.get_index(x + 1, y + 1);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_left_x {
            let index = self.get_index(x - 1, y);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_right_x {
            let index = self.get_index(x + 1, y);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_top_y {
            let index = self.get_index(x, y - 1);

            if self.cells[index] {
                count += 1;
            }
        }

        if valid_bottom_y {
            let index = self.get_index(x, y + 1);

            if self.cells[index] {
                count += 1;
            }
        }

        // Any live cell with two or three live neighbours lives
        // Any dead cell with exactly three live neighbours
        // Any live cell with more than three live neighbours dies
        // Any live cell with fewer than two live neighbours dies
        (is_alive && 1 < count && count < 4) || count == 3
    }

    pub fn update(&mut self) {
        let mut next_states = vec![false; self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);

                next_states[index] = self.next_state(x, y, self.cells[index]);
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);

                self.cells[index] = next_states[index];
            }
        }
    }

    pub fn reset(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;

                self.cells[index] = false;
            }
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = vec![false; width * height];
    }

    pub fn get_alive_count(&self) -> usize {
        self.cells.iter()
            .filter(|&state| *state == true)
            .count()
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;

        self.cells[index] = !self.cells[index];
    }
}