pub struct Board {
    white: Side,
    black: Side,
    white_to_move: bool,
}

struct Side {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    king: Bitboard,
    passantable_mask: u8,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
}

struct Bitboard {
    bitboard: u64,
}

struct Move {
    start: u8,
    end: u8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: Side::new(),
            black: Side::new(),
            white_to_move: true,
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
                    'p' => board.black.pawns.add_piece(&rank, &file),
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
                        let passantable_file_ascii: u32 = character.into();
                        passantable_file = passantable_file_ascii - 97;
                    } else if character.is_digit(10) {
                        let digit: u32 = character.to_digit(10).unwrap();

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

    fn piece_mask(&self) -> u64 {
        self.white.piece_mask() & self.black.piece_mask()
    }
}

impl Side {
    fn new() -> Self {
        Self {
            pawns: Bitboard::new(),
            knights: Bitboard::new(),
            bishops: Bitboard::new(),
            rooks: Bitboard::new(),
            queens: Bitboard::new(),
            king: Bitboard::new(),
            passantable_mask: 0,
            can_castle_kingside: true,
            can_castle_queenside: true,
        }
    }

    fn piece_mask(&self) -> u64 {
        self.pawns.bitboard
            & self.knights.bitboard
            & self.bishops.bitboard
            & self.rooks.bitboard
            & self.queens.bitboard
            & self.king.bitboard
    }

    pub fn make_passantable(&mut self, file: &u32) {
        let passantable: u8 = 1 << file;
        self.passantable_mask &= passantable;
    }
}

impl Bitboard {
    pub fn new() -> Self {
        Self { bitboard: 0 }
    }

    pub fn add_piece(&mut self, rank: &u32, file: &u32) {
        let new_piece: u64 = 1 << (8 * rank + file);
        self.bitboard &= new_piece;
    }
}
