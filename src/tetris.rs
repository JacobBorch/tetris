enum PlayerInput {
    Left,
    Right,
    Rotate,
    Swap,
    Place
}


struct Piece(Vec<(i32, i32)>);

impl Piece {
    fn rotated(&self) -> Self {
        Self(self.0.iter().map(|&(i, j)| (j, -i)).collect())
    }

    fn random() -> Self {
        todo!()
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
        todo!()
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
}

impl TetrisGame for Game {
    fn player_input(&mut self, input: PlayerInput) {
        match input {
            PlayerInput::Left => {
                
            },
            PlayerInput::Right => todo!(),
            PlayerInput::Rotate => {
                self.current_piece = self.current_piece.rotated()
            },
            PlayerInput::Swap => {
                if let Some(piece) = &self.saved_piece {

                }
            },
            PlayerInput::Place => todo!(),
        }
    }

    fn tick(&mut self) {
        todo!()
    }
}