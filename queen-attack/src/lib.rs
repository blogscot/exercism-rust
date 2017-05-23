#[derive(Clone, Debug)]
pub struct ChessPosition {
    row: i8,
    col: i8,
}

pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(row: i8, col: i8) -> Result<Self, String> {
        if row < 0 || col < 0 || row >= 8 || col >= 8 {
            return Err("Invalid Parameters!".into());
        }
        Ok(ChessPosition { row: row, col: col })
    }
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Self {
        Queen { pos: pos }
    }

    pub fn can_attack(self, queen: &Queen) -> bool {
        self.pos.col == queen.pos.col || self.pos.row == queen.pos.row ||
        Self::on_diagonal(self.pos.clone(), queen.pos.clone())
    }

    fn on_diagonal(pos1: ChessPosition, pos2: ChessPosition) -> bool {
        let delta_x = (pos1.col - pos2.col).abs();
        let delta_y = (pos1.row - pos2.row).abs();
        delta_x == delta_y
    }
}
