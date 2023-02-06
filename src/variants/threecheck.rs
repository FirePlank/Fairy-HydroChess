// Hand crafted evaluation

use crate::board::*;
use crate::evaluation::*;

pub static mut MASKS: Masks = Masks::new();

pub struct Masks {
    pub file_masks: [u64; 64],
    pub rank_masks: [u64; 64],
    pub isolated_masks: [u64; 64],
    pub white_passed_masks: [u64; 64],
    pub black_passed_masks: [u64; 64],
}

impl Masks {
    pub const fn new() -> Masks {
        Masks {
            file_masks: [0; 64],
            rank_masks: [0; 64],
            isolated_masks: [0; 64],
            white_passed_masks: [0; 64],
            black_passed_masks: [0; 64],
        }
    }
}

// evaluation function
pub fn evaluate(position: &Position) -> i16 {
    let mut score = 0;

    // the only difference between normal eval and 3check eval is the fact that we punish for getting checked and reward for giving checks
    score += 400 * position.checks[1] as i16 - 400 * position.checks[0] as i16;

    let phase = position.phase() <= 7;
    if phase {
        // add material score
        score += position.material_scores[0][1] - position.material_scores[1][1]; 
        // add piece square table score
        score += position.pst_scores[0][1] - position.pst_scores[1][1];
        // add double pawn score
        score += (position.bitboards[0].0 & position.bitboards[0].0 << 8).count_ones() as i16 * DOUBLED_PAWN_ENDING - 
        (position.bitboards[Piece::BlackPawn as usize].0 & position.bitboards[Piece::BlackPawn as usize].0 << 8).count_ones() as i16 * DOUBLED_PAWN_ENDING;
        // add mobility score
        score += position.mobility[2] * BISHOP - position.mobility[8] * BISHOP;
        score += position.mobility[3] * ROOK_EG - position.mobility[9] * ROOK_EG;
        score += position.mobility[4] * QUEEN_EG - position.mobility[10] * QUEEN_EG;
        // add score to get king closer to the other for mate
        score += force_king_corner(&position);
    } else {
        // add material score
        score += position.material_scores[0][0] - position.material_scores[1][0]; 
        // add piece square table score
        score += position.pst_scores[0][0] - position.pst_scores[1][0];
        // add double pawn score
        score += (position.bitboards[0].0 & position.bitboards[0].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING - 
        (position.bitboards[Piece::BlackPawn as usize].0 & position.bitboards[Piece::BlackPawn as usize].0 << 8).count_ones() as i16 * DOUBLED_PAWN_OPENING;

        score += position.mobility[2] * BISHOP - position.mobility[8] * BISHOP;
        score += position.mobility[3] * ROOK - position.mobility[9] * ROOK;
        score += position.mobility[4] * QUEEN - position.mobility[10] * QUEEN;
    }
    score += calculate_all(&position, phase);

    // count bishop pair
    if position.bitboards[Piece::WhiteBishop as usize].count() >= 2 {
        score += BISHOP_PAIR;
    } if position.bitboards[Piece::BlackBishop as usize].count() >= 2 {
        score -= BISHOP_PAIR;
    }

    // return final evaluation based on side
    return if position.side == 0 { score } else { -score };
}

pub fn force_king_corner(position: &Position) -> i16 {
    let mut eval = 0.0;

    // favour positions where the opponent king has been forced into the edge of the board
    // this makes the bot be able to checkmate easier in the endgame
    let opponent_square = if position.side == 0 { position.bitboards[Piece::BlackKing as usize].ls1b() } else {
        position.bitboards[Piece::WhiteKing as usize].ls1b()
    };
    let opponent_rank = GET_RANK[opponent_square as usize];
    let opponent_file = opponent_square % 8;

    eval += (3 - opponent_file).max(opponent_file - 4) as f32 + (3 - opponent_rank).max(opponent_rank - 4) as f32;

    // Incentivize moving king closer to opponent king
    let king_square = if position.side == 0 { position.bitboards[Piece::WhiteKing as usize].ls1b() } else {
        position.bitboards[Piece::BlackKing as usize].ls1b()
    };
    let king_rank = GET_RANK[king_square as usize];
    let king_file = king_square % 8;

    eval += 14.0 - ((king_file - opponent_file).abs() as f32 + (king_rank - opponent_rank).abs() as f32);

    return if position.side == 0 { (eval * 3.0 * (1.25-(position.phase()as f32/24.0))) as i16 } else { -(eval * 3.0 * (1.25-(position.phase()as f32/24.0))) as i16 };
}

// calculates all the different evaluation scores for the given position
pub fn calculate_all(position: &Position, phase: bool) -> i16 {
    let mut score = 0;
    if phase {
        for piece_index in 0..Piece::BlackPawn as usize {
            let mut bitboard = position.bitboards[piece_index];
            while bitboard.0 != 0 {
                let square = bitboard.ls1b();
                
                unsafe {
                    if piece_index == 0 {
                        // isolated pawns and passed pawns
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.isolated_masks[square as usize]) == 0 {
                            score += ISOLATED_PAWN_ENDING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.isolated_masks[square as usize]) == 0 {
                                score -= ISOLATED_PAWN_ENDING;
                            }
                        }
    
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                            score += PASSED_PAWN_ENDING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                                score -= PASSED_PAWN_ENDING;
                            }
                        }
                    }
                }
                bitboard.pop(square as usize);
            }
        }
        return score;
    }
    for piece_index in 0..Piece::BlackPawn as usize {
        let mut bitboard = position.bitboards[piece_index];
        while bitboard.0 != 0 {
            let square = bitboard.ls1b();
            
            unsafe {
                match piece_index {
                    0 => {
                        // isolated pawns and passed pawns
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.isolated_masks[square as usize]) == 0 {
                            score += ISOLATED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.isolated_masks[square as usize]) == 0 {
                                score -= ISOLATED_PAWN_OPENING;
                            }
                        }

                        if position.side == 0 && (position.bitboards[0].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                            score += PASSED_PAWN_OPENING;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.white_passed_masks[square as usize]) == 0 {
                                score -= PASSED_PAWN_OPENING;
                            }
                        }
                    },
                    3 | 9 => {
                        // open files
                        if position.side == 0 && (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                score += OPEN_FILE;
                            } else {
                                score += SEMI_OPEN_FILE;
                            }
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                                    score -= OPEN_FILE;
                                } else {
                                    score -= SEMI_OPEN_FILE;
                                }
                            }
                        }
                    },
                    5 | 11 => {
                        // open file penalties and king safety bonus
                        if position.side == 0 {
                            if (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                                if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                    // let mut a = (score, 0, 0);
                                    score -= OPEN_FILE_PENALTY;
                                    // a.1 = score;
                                    // a.2 = OPEN_FILE_PENALTY;
                                    // println!("{:?}", a);
                                } else {
                                    score -= SEMI_OPEN_FILE_PENALTY;
                                }
                            } else if square as usize % 8 != 0 && (position.bitboards[0].0 & MASKS.file_masks[square as usize - 1]) == 0 {
                                if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize - 1]) == 0 {
                                    if (position.bitboards[Piece::BlackRook as usize].0 & MASKS.file_masks[square as usize - 1]) != 0 {
                                        score -= 50;
                                    }
                                    score -= SIDE_OPEN;
                                } else {
                                    score -= SIDE_SEMI_OPEN;
                                }
                            } else if square as usize % 8 != 7 && (position.bitboards[0].0 & MASKS.file_masks[square as usize + 1]) == 0 {
                                if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize + 1]) == 0 {
                                    if (position.bitboards[Piece::BlackRook as usize].0 & MASKS.file_masks[square as usize + 1]) != 0 {
                                        score -= 50;
                                    }
                                    score -= SIDE_OPEN;
                                } else {
                                    score -= SIDE_SEMI_OPEN;
                                }
                            }
                            score += (KING_ATTACKS[square as usize] & position.occupancies[0].0).count_ones() as i16 * KING_SHIELD;
                        } else {
                            if (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize]) == 0 {
                                    score += OPEN_FILE_PENALTY;
                                } else {
                                    score += SEMI_OPEN_FILE_PENALTY;
                                }
                            } else if square as usize % 8 != 0 && (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize - 1]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize - 1]) == 0 {
                                    if (position.bitboards[Piece::WhiteRook as usize].0 & MASKS.file_masks[square as usize - 1]) != 0 {
                                        score += 50;
                                    }
                                    score += SIDE_OPEN;
                                } else {
                                    score += SIDE_SEMI_OPEN;
                                }
                            } else if square as usize % 8 != 7 && (position.bitboards[Piece::BlackPawn as usize].0 & MASKS.file_masks[square as usize + 1]) == 0 {
                                if (position.bitboards[0].0 & MASKS.file_masks[square as usize + 1]) == 0 {
                                    if (position.bitboards[Piece::WhiteRook as usize].0 & MASKS.file_masks[square as usize + 1]) != 0 {
                                        score += 50;
                                    }
                                    score += SIDE_OPEN;
                                } else {
                                    score += SIDE_SEMI_OPEN;
                                }
                            }
                            score -= (KING_ATTACKS[square as usize] & position.occupancies[0].0).count_ones() as i16 * KING_SHIELD;
                        }
                    }
                    _ => ()
                }
            }
            bitboard.pop(square as usize);
        }
    }
    return score;
}