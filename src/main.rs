use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::io;
use std::cmp;
use colored::Colorize;
use colored::Color;

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
    board: [[Option<Faction>; Game::COLUMNS]; Game::ROWS],
    players: Vec<Player>,
    over: bool,
    winner: Option<usize>,  // The index of the winning player, if there is one
    next_index: usize,      // The index of the player whose turn it is
    free_spots: usize,      // The number of free spots in the board
}

impl Game {
    const ROWS: usize = 3;
    const COLUMNS: usize = 3;
    
    fn with_players(players: Vec<Player>) -> Game {
        Game {
            board: [[None; Game::COLUMNS]; Game::ROWS],
            players,
            over: false,
            winner: None,
            next_index: 0,
            free_spots: Game::COLUMNS * Game::ROWS,
        }
    }

    fn play(&mut self) {
        while !self.over {
            self.display();
            let col = Game::ask_col(&self.players[self.next_index]);

            self.drop_piece(col);
        }

        self.display();

        match self.winner {
            Some(winner) => println!("Congrats {}!", self.players[winner].name),
            None => println!("Game over!"),
        }
    }

    fn ask_col(player: &Player) -> usize {
        println!(
            "It is {}'s turn, please pick a column (between 0 and {})", 
            player.name,
            Game::COLUMNS-1
        );

        loop {
            let mut col = String::new();
            
            io::stdin()
                .read_line(&mut col)
                .expect("Error reading line");

            let col: usize = match col.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Enter a number!");
                    continue
                }
            };

            if col >= Game::COLUMNS {
                println!("Too high!");
                continue
            }

            return col
        }
    }

    fn drop_piece(&mut self, col: usize) {
        let mut found = false;
        let (mut i, mut j) = (0, 0);
        for (_i, row) in self.board.iter().enumerate() {
            if let None = row[col] {
                found = true;
                (i, j) = (_i, col);
            }
        }

        if found {
            self.board[i][j] = Some(
                self.players[self.next_index].faction
            );

            self.free_spots -= 1;

            if self.check_won(i, j) {
                self.over = true;
                self.winner = Some(self.next_index);
            } else if self.free_spots == 0 {
                self.over = true;
            }
            
            self.next_index = (self.next_index + 1) % self.players.len();
        }
    }

    fn check_won(&self, row: usize, col: usize) -> bool {
        todo!()
    }

    fn display(&self) {
        for row in self.board {
            for elem in row {
                match elem {
                    Some(f) => print!("{}  ", f),
                    None => print!("-  "),
                }
            }
            println!()
        }

        for i in 0..Game::COLUMNS {
            print!("{:<2} ", i);
        }
        println!();
    }
}

fn main() {
    let p1 = Player {
        name: String::from("Player 1"),
        faction: Faction { symbol: 'X', color: Color::Red },
    };

    let p2 = Player {
        name: String::from("Player 2"),
        faction: Faction { symbol: 'O', color: Color::Blue },
    };

    let mut game = Game::with_players(Vec::from([p1, p2]));

    game.play();
}
