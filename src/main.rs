#[macro_use]
extern crate lazy_static;

mod board;
use board::*;

mod r#move;
use r#move::*;

mod uci;
use uci::*;

mod search;
use search::*;

mod evaluation;
use evaluation::*;

mod cache;
use cache::*;

mod variants;
use variants::*;

use std::mem::MaybeUninit;

// use std::thread;

// FEN debug positions
// empty_board "8/8/8/8/8/8/8/8 w - -"
// start_position "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
// tricky_position "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
// killer_position "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
// cmk_position "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
// fireplank_special "r3k2r/4nq1P/1n2N1b1/1b6/4N3/5B2/1pRQPPPP/2BK4 w kq - 0 1"

// use std::io;
// use std::io::Write;

fn main() {
    init_all();

    // debug mode variable
    let debug = false;
    if debug {
        // let mut position = Position::from_fen("rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 3+3 0 1");
        // let mut position = Position::new();
        // unsafe {
        //     OPTIONS.variant = Variant::ThreeCheck;
        // }

        // parse_position("position fen rnbq1bnr/ppp2kpp/8/4p3/3pP3/BPP5/P2P1PPP/RN1QK1NR w KQ - 0 7 +2+0");

        // let king_square = position.bitboards[Piece::WhiteKing as usize].ls1b();
        // // left rook
        // let left_rook_square = position.bitboards[Piece::WhiteRook as usize].ls1b();
        // // right rook
        // let right_rook_square = position.bitboards[Piece::WhiteRook as usize].ms1b();

        // // print everything
        // println!("King square: {}", king_square);
        // println!("Left rook square: {}", left_rook_square);
        // println!("Right rook square: {}", right_rook_square);

        // get position of rook
        // let string: char = 'D';
        // println!("{} {}", string as isize - 65, position.bitboards[Piece::WhiteKing as usize].ls1b() % 8);
        // perft_test(&mut position, 2);

        // let mut move_list = MoveList::new()
        // chess960::generate_moves(&mut position, &mut move_list);
        // move_list.show();

        // print bitboard of rook
        // position.bitboards[Piece::WhiteRook as usize].show();
        // generate legal antichess moves
        // let mut moves = antichess::generate_moves(&mut position);
        // moves.show();

        // print moves
        // move_list.show();

    } else {
        // start the main UCI loop to handle commands
        main_loop();
    }
}