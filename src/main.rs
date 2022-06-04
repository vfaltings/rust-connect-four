const BOARD_SIZE: usize = 10;

fn main() {
    let mut gameboard = [[0; BOARD_SIZE]; BOARD_SIZE];
    display_board(&gameboard);
}

fn display_board(board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
    for row in board {
        for elem in row {
            match elem {
                1 => print!("X "),
                2 => print!("O "),
                _ => print!("- "),
            }
        }
        println!();
    }
}