fn main() {
    fn chessboard_grains(chessboard_squares: u8) -> u128 {
        let mut grains: u128 = 0;
        let mut square = 1;
        while square <= chessboard_squares {
            if grains == 0 {
                grains = 1;
                continue;
            }

            grains = grains * 2;
            square += 1;
        }

        return grains;
    }

    fn consult_square(mut square: u8) -> u128 {
        let mut grains: u128 = 0;
        while square > 0 {
            if grains == 0 {
                grains = 1;
                continue;
            }

            grains = grains * 2;
            square = square - 1;
        }

        return grains;
    }
}
