use super::TetrisMap;
use rand::distributions::{Standard, Distribution};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl TetrominoType {
    fn color_code(&self) -> u8 {
        match self {
            TetrominoType::I => 2,
            TetrominoType::J => 3,
            TetrominoType::L => 4,
            TetrominoType::O => 5,
            TetrominoType::S => 6,
            TetrominoType::T => 7,
            TetrominoType::Z => 8,
        }
    }

    fn map(&self) -> [[u8; 4];2] {
        match self {
            TetrominoType::I => [
                [0, 0, 0, 0],
                [1, 1, 1, 1]
            ],
            TetrominoType::J => [
                [1, 1, 1, 0],
                [0, 0, 1, 0]
            ],
            TetrominoType::L => [
                [1, 1, 1, 0],
                [1, 0, 0, 0]
            ],
            TetrominoType::O => [
                [0 , 1, 1, 0],
                [0 , 1, 1, 0]
            ],
            TetrominoType::S => [
                [0, 1, 1, 0],
                [1, 1, 0, 0]
            ],
            TetrominoType::T => [
                [1, 1, 1, 0],
                [0, 1, 0, 0]
            ],
            TetrominoType::Z => [
                [1, 1, 0, 0],
                [0, 1, 1, 0]
            ],
        }
    }
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0, 6) {
            0 => TetrominoType::I,
            1 => TetrominoType::J,
            2 => TetrominoType::L,
            3 => TetrominoType::O,
            4 => TetrominoType::S,
            5 => TetrominoType::T,
            _ => TetrominoType::Z
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tetromino {
    variant: TetrominoType,
    x: usize,
    pub y: usize,
    rot: usize, // by 90 degree increments
}

impl Tetromino {

    pub fn new() -> Self{
        Self {
            variant: rand::random(),
            x: 4,
            y: 0,
            rot: 0
        }
    }

    pub fn check(&self, tetris_map: &TetrisMap) -> bool {
        let tetromino_map = self.variant.map();

        for row in 0..2 {
            for col in 0..4 {
                match self.rot {
                    0 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+row][self.x+col] != 0 {
                            return false;
                        }
                    },
                    1 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+col][self.x+1-row] != 0 {
                            return false;
                        }
                    },
                    2 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+1-row][self.x+2-col] != 0 {
                            return false;
                        }
                    },
                    3 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+3-col][self.x+row] != 0 {
                            return false;
                        }
                    },
                    _ => {}
                }
            }
        }

        true
    }

    pub fn add_to_map(&self, tetris_map: &mut TetrisMap) {
        let color_code = self.variant.color_code();

        let tetromino_map = self.variant.map();

        for row in 0..2 {
            for col in 0..4 {
                match self.rot {
                    0 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+row][self.x+col] == 0 {
                            tetris_map[self.y+row][self.x+col] = color_code;
                        }
                    },
                    1 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+col][self.x+1-row] == 0 {
                            tetris_map[self.y+col][self.x+1-row] = color_code;
                        }
                    },
                    2 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+1-row][self.x+2-col] == 0 {
                            tetris_map[self.y+1-row][self.x+2-col] = color_code;
                        }
                    },
                    3 => {
                        if tetromino_map[row][col] == 1 && tetris_map[self.y+3-col][self.x+row] == 0 {
                            tetris_map[self.y+3-col][self.x+row] = color_code;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn rotate(&mut self, tetris_map: &TetrisMap) {
        self.rot = (self.rot+1) % 4;

        if !self.check(tetris_map) {
            self.rot = (self.rot+1) % 4;
        }
    }

    pub fn left(&mut self, tetris_map: &TetrisMap) {
        if self.x > 0 {
            self.x -= 1;

            if !self.check(tetris_map) {
                self.right(tetris_map);
            }
        }
    }

    pub fn right(&mut self, tetris_map: &TetrisMap) {
        if self.x < 12 {
            self.x += 1;

            if !self.check(tetris_map){
                self.left(tetris_map);
            }
        }
    }

    pub fn down(&mut self, tetris_map: &TetrisMap) -> bool {
        self.y += 1;

        if !self.check(tetris_map) {
            self.y -= 1;
            return false
        }

        true
    }
}
