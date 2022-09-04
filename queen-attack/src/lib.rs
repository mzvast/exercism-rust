#[derive(Debug)]
pub struct ChessPosition(i32, i32); // (row,col)

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // unimplemented!(
        //     "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
        //     rank, // row
        //     file // col
        // );
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            None
        } else {
            Some(ChessPosition(rank, file))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // unimplemented!(
        //     "Given the chess position {:?}, construct a Queen struct.",
        //     position
        // );
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // unimplemented!(
        //     "Determine if this Queen can attack the other Queen {:?}",
        //     other
        // );

        let is_same_row_or_col =
            self.position.0 == other.position.0 || self.position.1 == other.position.1;

        let cross_up_1 = self.position.0 + self.position.1;
        let cross_up_2 = other.position.0 + other.position.1;

        let cross_down_1 = self.position.0 - self.position.1;
        let cross_down_2 = other.position.0 - other.position.1;
        // diag
        let is_same_cross = cross_up_1 == cross_up_2 || cross_down_1 == cross_down_2;

        is_same_row_or_col || is_same_cross
    }
}
