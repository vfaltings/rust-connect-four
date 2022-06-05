use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::io;
use std::cmp;
use colored::Colorize;
use colored::Color;

// Size of the board, must be <= 100
// or the column index display will be wonky
const BOARD_SIZE: usize = 20;
const NUM_PLAYERS: usize = 2;

#[derive(Debug, Copy, Clone)]
struct Faction {
    symbol: char,
    color: Color,
}

impl Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.symbol).color(self.color) )
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    faction: Faction
}

struct Game {
    board: [[Option<Faction>; BOARD_SIZE]; BOARD_SIZE],
    players: [Player; NUM_PLAYERS],
    over: bool,
    winner: Option<Player>
}

impl Game {
    fn with_players(players: [Player; NUM_PLAYERS]) -> Game {
        Game {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            players,
            over: false,
            winner: None,
        }
    }

    fn drop_piece(&mut self, col: usize, faction: Faction) {
        for (i, row) in self.board.iter().enumerate() {
            match row[i] {
                Some(_) => continue,
                None => {
                    self.board[i][col] = Some(faction);
                    break
                }
            }
        }
    }

    fn display(&self) {
        for (i, row) in self.board.iter().enumerate().rev() {
            for elem in row {
                match elem {
                    Some(f) => print!("{}  ", f),
                    None => print!("-  "),
                }
            }
            println!()
        }

        for i in 0..BOARD_SIZE {
            print!("{:<2} ", i);
        }
        println!();
    }
}

fn main() {
    let p1 = Player {
        name: String::from("Player1"),
        faction: Faction { symbol: 'X', color: Color::Red },
    };

    let p2 = Player {
        name: String::from("Player2"),
        faction: Faction { symbol: 'O', color: Color::Blue },
    };

    let mut game = Game::with_players([p1, p2]);

    game.display();
}

fn play(board: &mut [[i32; BOARD_SIZE]; BOARD_SIZE], faction: i32) -> bool {
    println!("Now playing: player {} ({})",
        faction,
        if faction == 1 {'X'} else {'O'});
    loop {
        let col = get_user_col();
        match put_piece(board, col, faction) {
            Ok(last_played) => {
                display_board(board);
                if check_win(board, last_played) {
                    println!("Congrats player {}", faction);
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                println!("That column is full, try again");
                continue
            }
        }
    }
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
        if row[col] == 0 {
            board[i][col] = val;
            return Ok((i, col));
        }
    }

    Err("Column full")
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