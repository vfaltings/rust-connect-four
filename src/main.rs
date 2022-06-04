use std::io;

const BOARD_SIZE: usize = 10;

fn main() {
    let mut gameboard = [[0; BOARD_SIZE]; BOARD_SIZE];

    let col = get_user_col();
    if put_piece(&mut gameboard, col, 1) {
        display_board(&gameboard);
    }
}

fn get_user_col() -> usize {
    println!("Enter a column index (between 0 and {})", BOARD_SIZE-1);

    let mut buf = String::new();
    let mut col: usize;

    loop {
        buf.clear();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading line, try again: ");
                continue
            }
        }
        
        col = match buf.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a number!");
                continue
            }
        };

        if col < BOARD_SIZE {
            break
        } else {
            println!("Please respect the bounds (0 to {})", BOARD_SIZE-1);
        }
    }

    col
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
        print!("{}  ", i);
    }
    println!();
}