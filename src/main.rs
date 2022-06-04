use std::io;
use std::cmp;

const BOARD_SIZE: usize = 10;

fn main() {
    let mut gameboard = [[0; BOARD_SIZE]; BOARD_SIZE];
    display_board(&gameboard);

    let col = get_user_col();
}

fn check_win(board: &[[i32; BOARD_SIZE]; BOARD_SIZE], last_played: (usize, usize)) -> bool {
    let (i, j) = last_played;
    let faction = board[i][j];

    // Check horizontal
    let from = cmp::max(0, j-3);
    let to = cmp::min(BOARD_SIZE-1, j+3);
    let mut count = 0;
    for j in from..=to {
        if board[i][j] == faction {
            count += 1;
        } else {
            if count >= 4 {
                break;
            } else {
                count = 0;
            }
        }
    }

    if count >= 4 {
        return true;
    }

    // Check verical
    let from = cmp::max(0, i-3);
    let to = cmp::min(BOARD_SIZE-1, i+3);
    let mut count = 0;
    for i in from..=to {
        if board[i][j] == faction {
            count += 1;
        } else {
            if count >= 4 {
                break;
            } else {
                count = 0;
            }
        }
    }

    if count >= 4 {
        return true;
    }

    // Check diagonal
    // TODO
    return false;
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

fn put_piece(board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE], col: usize, val: i32) -> Result<(usize, usize), &str> {
    for (i, row) in board.iter().enumerate() {
        for (j, elem) in row.iter().rev().enumerate() {
            if j != col {
                continue;
            }

            if elem == &0 {
                board[i][j] = val;
                return Ok((i, j));
            }
        }
    }

    Err("Could not play")
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