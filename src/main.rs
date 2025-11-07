use board::Board;

use std::time::Instant;
use std::hint::black_box;

mod util;
mod magic;
mod board;

fn main() {
    println!("Hello, world!");
    let _ = &*magic::MAGIC_MAPS;
    println!("Force init completed.");

    let depth = 8;

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_iter_8", dfs_iter, &mut board, depth, 40);

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_get_8", dfs_get, &mut board, depth, 40);

    let depth = 9;

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_get_9", dfs_get, &mut board, depth, 20);

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_iter_9", dfs_iter, &mut board, depth, 20);

    let depth = 10;

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_iter_10", dfs_iter, &mut board, depth, 4);

    let mut board = Board::default();
    board.bb = 0b111111111111110011111111111111111111;
    bench("dfs_get_10", dfs_get, &mut board, depth, 4);
}

fn bench<F>(name: &str, mut f: F, board: &mut Board, depth: u8, runs: usize)
where
    F: FnMut(&mut Board, u8) -> u64,
{
    let warm = f(board, depth);
    black_box(warm);

    let mut times = Vec::with_capacity(runs);
    let mut res = 0;
    for _ in 0..runs {
        let start = Instant::now();
        res = f(board, depth);
        let dur = start.elapsed();
        black_box(res);
        times.push(dur);
    }

    let total_nanos: u128 = times.iter().map(|d| d.as_nanos()).sum();
    let avg = total_nanos as f64 / runs as f64;
    let min = times.iter().map(|d| d.as_nanos()).min().unwrap_or(0);
    let max = times.iter().map(|d| d.as_nanos()).max().unwrap_or(0);
    let mut xs: Vec<u128> = times.iter().map(|d| d.as_nanos()).collect();
    xs.sort_unstable();
    let med = xs[xs.len() / 2];

    println!(
        "{:12} runs={} // cnt {} // nanos // avg {:.0} // med {:.0} // min {} // max {}",
        name, runs, res, avg, med, min, max
    );
    println!(
        "{:12} ms // avg {} // // {} operations per sec.",
        "", (avg as f64) / 1000000.0, 1000000000.0 * res as f64 / (avg as f64)
    );
}

fn dfs_iter(board: &mut Board, mut depth: u8) -> u64 {
    depth -= 1;
    let mut cnt = 0;
    let mut it = board.iter_moves();
    while let Some(mov) = it.next() {
        board.make_move(mov);
        if depth != 0 {
            cnt += dfs_iter(board, depth);
        }
        board.make_move(mov);
        cnt += 1;
    }
    cnt
}

fn dfs_get(board: &mut Board, mut depth: u8) -> u64 {
    depth -= 1;
    let mut cnt = 0;
    let moves = board.get_moves();
    for mov in moves.iter() {
        board.make_move(*mov);
        if depth != 0 {
            cnt += dfs_get(board, depth);
        }
        board.make_move(*mov);
        cnt += 1;
    }
    cnt
}