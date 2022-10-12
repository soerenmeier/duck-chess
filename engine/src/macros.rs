
macro_rules! iter_board {
	($board:expr) => ({
		let board: &[_; 64] = &$board;
		board.iter().enumerate().map(|(i, p)| (
			unsafe { Square::from_u8_unchecked(i as u8) },
			p
		))
	})
}