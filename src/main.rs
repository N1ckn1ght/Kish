use board::Board;

mod util;
mod magic;
mod board;

fn main() {
    println!("Hello, world!");
    let _ = &*magic::MAGIC_MAPS;
    println!("Force init completed.");

    let mut board = Board::default();

    board.bb = 0b110000111010000001000000010011010011;

    // let mut cnt = 0;
    // let moves = board.get_moves();
    // for mov in moves.iter() {
    //     println!("{}", mov);
    //     board.make_move(*mov);
    //     let moves2 = board.get_moves();
    //     for mov2 in moves2.iter() {
    //         println!("\t{}", mov2);
    //         board.make_move(*mov2);
    //         board.make_move(*mov2);
    //         cnt += 1;
    //     }
    //     board.make_move(*mov);
    //     cnt += 1;
    // }
    // println!("total = {}", cnt);

    // println!();
    // util::print_boards(&[board.bb], Some(1));

    let mut cnt = 0;
    let mut it = board.iter_moves();
    while let Some(mov) = it.next() {
        // println!("{}", mov);
        board.make_move(mov);
        let mut it2 = board.iter_moves();
        while let Some(mov2) = it2.next() {
            board.make_move(mov2);
            board.make_move(mov2);
            cnt += 1;
        }
        board.make_move(mov);
        cnt += 1;
    }
    println!("total = {}", cnt);

    
}
