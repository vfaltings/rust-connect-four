const BOARD_SIZE: usize = 10;

fn main() {
    let mut gameboard = [[0; BOARD_SIZE]; BOARD_SIZE];

    if put_piece(&mut gameboard, 0, 1) {
        display_board(&gameboard);
    }
}

fn put_piece(board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE], col: usize, val: i32) -> bool {
    for (i, row) in board.iter().enumerate() {
        for (j, elem) in row.iter().rev().enumerate() {
            if j != col {
                continue;
            }

            if elem == &0 {
                board[i][j] = val;
                return true;
            }
        }
    }

    false
}

fn display_board(board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
    for row in board.iter().rev() {
        for elem in row {
            match elem {
                1 => print!("X  "),
                2 => print!("O  "),
                _ => print!("-  "),
            }
        }
        println!();
    }

    for i in 0..BOARD_SIZE {
        print!("{}  ", i + 1);
    }
    println!();
}