use std::fmt;

pub struct Board {
    white: Side,
    black: Side,
    board: [usize; 64],
    white_to_move: bool,
    moves: usize,
}

struct Side {
    bitboards: [u64; 6],
    passantable_mask: u8,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: Side::new(),
            black: Side::new(),
            board: [0; 64],
            white_to_move: true,
            moves: 0,
        }
    }

    pub fn default() -> Self {
        let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        Board::read_fen(starting_position)
    }

    pub fn read_fen(fen: &str) -> Self {
        let mut board = Self::new();
        let mut rank: u32 = 7;
        let mut file: u32 = 0;
        let mut read_board = 0;
        let mut passantable_file: u32 = 0;

        // Loop over all characters in the FEN string
        for character in fen.chars() {
            match read_board {
                // Read the piece locations
                0 => match character {
                    'p' => board.add_piece(&rank, &file),
                    'n' => board.black.knights.add_piece(&rank, &file),
                    'b' => board.black.bishops.add_piece(&rank, &file),
                    'r' => board.black.rooks.add_piece(&rank, &file),
                    'q' => board.black.queens.add_piece(&rank, &file),
                    'k' => board.black.king.add_piece(&rank, &file),

                    'P' => board.white.pawns.add_piece(&rank, &file),
                    'N' => board.white.knights.add_piece(&rank, &file),
                    'B' => board.white.bishops.add_piece(&rank, &file),
                    'R' => board.white.rooks.add_piece(&rank, &file),
                    'Q' => board.white.queens.add_piece(&rank, &file),
                    'K' => board.white.king.add_piece(&rank, &file),

                    '/' => {
                        file = 0;
                        rank -= 1;
                    }
                    ' ' => read_board += 1,
                    _ => {
                        let step: u32 = character.to_digit(10).unwrap();
                        file += step;
                    }
                },

                // Read white or black to move
                1 => match character {
                    'w' => board.white_to_move = true,
                    'b' => board.white_to_move = false,
                    _ => read_board += 1,
                },

                // Read castling rights
                2 => match character {
                    '-' => {
                        board.white.can_castle_kingside = false;
                        board.white.can_castle_queenside = false;
                        board.black.can_castle_kingside = false;
                        board.black.can_castle_queenside = false;
                    }
                    'K' => board.white.can_castle_kingside = true,
                    'Q' => board.white.can_castle_queenside = true,
                    'k' => board.black.can_castle_kingside = true,
                    'q' => board.black.can_castle_queenside = true,
                    _ => read_board += 1,
                },

                3 => {
                    if character.is_lowercase() {
                        let passantable_file_ascii: usize = character.into();
                        passantable_file = passantable_file_ascii - 97;
                    } else if character.is_ascii_digit() {
                        let digit: usize = character.to_digit(10).unwrap();

                        if digit == 3 {
                            board.white.make_passantable(&passantable_file);
                        } else {
                            board.black.make_passantable(&passantable_file);
                        }
                    }
                }

                _ => break,
            }
        }

        board
    }

    fn add_piece(&mut self, is_white: bool, kind: usize, rank: usize, file: usize) {
        if is_white {
            self.white.bitboards[kind] = 1 << (8 * rank + file);
            self.board[rank * 8 + file] = kind;
        } else {
            self.black.bitboards[kind] = 1 << (8 * rank + file);
            self.board[rank * 8 + file] = kind + 6;
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Side {
    fn new() -> Self {
        Self {
            bitboards: [0; 6],
            passantable_mask: 0,
            can_castle_kingside: true,
            can_castle_queenside: true,
        }
    }

    pub fn make_passantable(&mut self, file: &u32) {
        let passantable: u8 = 1 << file;
        self.passantable_mask &= passantable;
    }
}

pub fn print_piece(piece_number: &usize) -> &str {
    "k"
}
