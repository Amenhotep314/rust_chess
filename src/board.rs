pub struct Board {
    white: Side,
    black: Side,
    white_to_move: bool,
}

struct Side {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    king: u64,
    passantable_mask: u8,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: Side::new(),
            black: Side::new(),
            white_to_move: true,
        }
    }

    fn piece_mask(&self) -> u64 {
        self.white.piece_mask() & self.black.piece_mask()
    }
}

impl Side {
    fn new() -> Self {
        Self {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            king: 0,
            passantable_mask: 0,
            can_castle_kingside: true,
            can_castle_queenside: true,
        }
    }

    fn piece_mask(&self) -> u64 {
        self.pawns & self.knights & self.bishops & self.rooks & self.queens & self.king
    }
}
