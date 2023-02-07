use crate::{board::{Position, Piece}, evaluation::*};

// evaluation function
pub fn evaluate(position: &Position) -> i16 {
    let mut score = 0;

    // give score based on what rank king is on
    let white_rank = 8 - ((position.bitboards[Piece::WhiteKing as usize].ls1b() as usize) + 1) / 8;
    let black_rank = 8 - ((position.bitboards[Piece::BlackKing as usize].ls1b() as usize) + 1) / 8;

    score += 350 * white_rank as i16 - 350 * black_rank as i16;

    // add material score
    score += position.material_scores[0][0] - position.material_scores[1][0];

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    }

    return if position.side == 0 { score } else { -score };
}
