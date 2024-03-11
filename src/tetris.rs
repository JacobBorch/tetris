enum PlayerInput {
    Left,
    Right,
    Rotate,
    Swap,
    Place,
}

#[derive(Clone)]
struct Piece(Vec<(i32, i32)>);

lazy_static::lazy_static! {
    static ref O: Piece = Piece(vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
    static ref I: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
    static ref S: Piece = Piece(vec![(0, 0), (-1, -1), (1, 0), (0, 1)]);
    static ref Z: Piece = Piece(vec![(0, 0), (1, 1), (1, 0), (0, -1) ]);
    static ref L: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (2, 1)]);
    static ref J: Piece = Piece(vec![(0, 0), (1, 0), (2, 0), (2, -1)]);
    static ref T: Piece = Piece(vec![(0, 0), (0, 1), (0, -1), (1, 0)]);

    static ref PIECES: Vec<Piece> =  vec![O.clone(), I.clone(), S.clone(), Z.clone(), L.clone(), J.clone(), T.clone()];
}

impl Piece {
    fn rotated(&self) -> Self {
        Self(self.0.iter().map(|&(i, j)| (j, -i)).collect())
    }

    fn random() -> Self {
        let i = fastrand::usize(0..7);
        PIECES[i].clone()
    }
}

trait TetrisGame {
    fn player_input(&mut self, input: PlayerInput);

    fn tick(&mut self);
}

struct Game {
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

    fn can_move_right(&self) -> bool {
        todo!()
    }

    fn can_move_left(&self) -> bool {
        todo!()
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
                todo!()
            }
        }
    }

    fn tick(&mut self) {
        todo!()
    }
}
