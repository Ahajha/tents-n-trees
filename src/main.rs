use crate::board::TentsAndTreesBoard;

mod board;

fn main() {
    /*
      1 0 2 0
    1 _ T ^ _
    0 T _ _ _
    1 ^ _ T _
    1 _ _ ^ _
    */
    let mut board = TentsAndTreesBoard::new(
        vec![1, 0, 1, 1],
        vec![1, 0, 2, 0],
        &[(0, 1), (1, 0), (2, 2)],
    );

    board.solve();

    println!("{}", board);
}
