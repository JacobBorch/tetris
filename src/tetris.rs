use bevy::{ecs::system::Resource, render::color::Color};

pub enum PlayerInput {
    Left,
    Right,
    Rotate,
    Swap,
    Place,
}

#[derive(Clone)]
//Offsets and color
pub struct Piece(Vec<(i32, i32)>, usize);

lazy_static::lazy_static! {
    static ref O: Piece = Piece(vec![(0, 0), (1, 0), (0, 1), (1, 1)], 1);
    static ref I: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (3, 0)], 2);
    static ref S: Piece = Piece(vec![(0, 0), (-1, -1), (1, 0), (0, 1)], 3);
    static ref Z: Piece = Piece(vec![(0, 0), (1, 1), (1, 0), (0, -1) ], 4);
    static ref L: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (2, 1)], 5);
    static ref J: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (2, -1)], 6);
    static ref T: Piece = Piece(vec![(0, 0), (0, 1), (0, -1), (1, 0)], 7);

    static ref PIECES: Vec<Piece> =  vec![O.clone(), I.clone(), S.clone(), Z.clone(), L.clone(), J.clone(), T.clone()];
}


impl Piece {
    fn rotated(&self) -> Self {
        Self(self.0.iter().map(|&(i, j)| (j, -i)).collect(), self.1)
    }

    fn random() -> Self {
        let i = fastrand::usize(0..PIECES.len());
        PIECES[i].clone()
    }

    pub fn color(&self) -> Color {
        match self.1 {
            1 => Color::YELLOW,
            2 => Color::TURQUOISE,
            3 => Color::RED,
            4 => Color::GREEN,
            5 => Color::ORANGE,
            6 => Color::BLUE,
            7 => Color::PURPLE,
            _ => unreachable!()
        }
    }
}

pub trait TetrisGame {
    fn player_input(&mut self, input: PlayerInput);

    fn tick(&mut self);

    fn get_board(&self) -> &Vec<Vec<usize>>;

    fn get_current_piece(&self) -> ((usize, usize), Piece);
}

#[derive(Resource)]
pub struct Game {
    board: Vec<Vec<usize>>,
    current_piece: Piece,
    piece_pos: (usize, usize),
    saved_piece: Option<Piece>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; 10]; 20],
            current_piece: Piece::random(),
            piece_pos: (0, 10 / 2),
            saved_piece: None,
        }
    }

    fn spawn_new_piece(&mut self, opt_piece: Option<Piece>) {
        if let Some(piece) = opt_piece {
            self.current_piece = piece;
        } else {
            self.current_piece = Piece::random();
        }
        self.reset_piece()
    }

    fn reset_piece(&mut self) {
        self.piece_pos = (0, self.board[0].len() / 2)
    }

    fn can_move(&self, dx: i32, dy: i32) -> bool {
        self.current_piece.0.iter().all(|&(x, y)| {
            let new_x = x + dx + self.piece_pos.0 as i32;
            let new_y = y + dy + self.piece_pos.1 as i32;

            new_x >= 0 && new_x < self.board.len() as i32 && // Check vertical boundaries
            new_y >= 0 && new_y < self.board[0].len() as i32 && // Check horizontal boundaries
            self.board[new_x as usize][new_y as usize] == 0 // Check for no blockage
        })
    }

    fn can_move_left(&self) -> bool {
        self.can_move(0, -1)
    }
    
    fn can_move_right(&self) -> bool {
        self.can_move(0, 1)
    }
    
    fn can_move_down(&self) -> bool {
        self.can_move(1, 0)
    }
    
    fn lock_piece(&mut self) {
        // Place the piece on the board
        for &(x, y) in &self.current_piece.0 {
            let board_x = (self.piece_pos.0 as i32 + x) as usize;
            let board_y = (self.piece_pos.1 as i32 + y) as usize;
            if board_x < self.board.len() && board_y < self.board[0].len() {
                self.board[board_x][board_y] = 1;
            }
        }

        // Check for and clear any full lines
        self.remove_full_lines();

        // Spawn a new piece
        self.spawn_new_piece(None);
    }

    fn remove_full_lines(&mut self) {
        self.board.retain(|row| row.iter().any(|&cell| cell == 0));
        while self.board.len() < 20 { // Assuming 20 rows in the standard Tetris game
            self.board.insert(0, vec![0; 10]); // Re-add empty rows at the top
        }
    }
}

impl TetrisGame for Game {
    fn player_input(&mut self, input: PlayerInput) {
        match input {
            PlayerInput::Left => {
                if self.can_move_left() {
                    self.piece_pos.1 -= 1
                }
            }
            PlayerInput::Right => {
                if self.can_move_right() {
                    self.piece_pos.1 += 1
                }
            }
            PlayerInput::Rotate => self.current_piece = self.current_piece.rotated(),
            PlayerInput::Swap => {
                let prev = &self.saved_piece;
                if prev.is_some() {
                    let tmp = self.current_piece.clone();
                    self.current_piece = prev.clone().unwrap();
                    self.saved_piece = Some(tmp);
                    self.reset_piece()
                } else {
                    self.saved_piece = Some(self.current_piece.clone());
                    self.spawn_new_piece(None);
                }
            }
            PlayerInput::Place => {
                self.lock_piece()
            }
        }
    }

    fn tick(&mut self) {
        if !self.can_move_down() {
            self.lock_piece()
        } else {
            self.piece_pos.0 += 1
        }
    }

    fn get_board(&self) -> &Vec<Vec<usize>> {
        &self.board
    }

    fn get_current_piece(&self) -> ((usize, usize), Piece) {
        (self.piece_pos, self.current_piece.clone())
    }
}
