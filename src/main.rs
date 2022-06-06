extern crate ncurses;

use ncurses::*;
use std::cmp;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Action {
    Quit,
    Move(Direction),
    Drop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Faction {
    Blue,
    Red,
}

impl Faction {
    fn swap(self) -> Faction {
        match self {
            Faction::Blue => Faction::Red,
            Faction::Red => Faction::Blue,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Some(Faction),
    Empty,
}

struct Game {
    board: [[Cell; Self::COLS]; Self::ROWS],
    cursor_col: i32,
    free_spots_in_col: [i32; Self::COLS],
    free_spots: usize,
    curr_turn: Faction,
    winner: Option<Faction>,
}

impl Game {
    const ROWS: usize = 16;
    const COLS: usize = 20;

    fn play(&mut self) {
        self.init();

        loop {
            clear();
            self.draw_board();
            self.draw_ui();
            refresh();

            let input = Self::get_input();

            match input {
                Action::Quit => break,
                Action::Move(dir) => self.move_cursor(dir),
                Action::Drop => match self.drop_piece() {
                    Some((row, col)) => {
                        if self.free_spots == 0 || self.check_win(row, col, self.curr_turn) {
                            break;
                        }
                        self.curr_turn = self.curr_turn.swap();
                    }
                    None => (),
                },
            }
        }

        clear();
        self.draw_board();
        match self.winner {
            Some(f) => {
                mv(0, 0);
                match f {
                    Faction::Blue => addstr_color("Blue Team", 1),
                    Faction::Red => addstr_color("Red Team", 2),
                }
                addstr(" wins!");
            }
            None => {
                mvprintw(0, 0, "Game over!");
            }
        }
        mvprintw(1, 0, "Press any key to exit");

        getch();

        endwin();
    }

    fn init(&self) {
        initscr();
        cbreak();
        noecho();

        start_color();

        init_pair(1, COLOR_BLUE, COLOR_BLACK);
        init_pair(2, COLOR_RED, COLOR_BLACK);
    }

    fn check_win(&mut self, row: usize, col: usize, faction: Faction) -> bool {
        // Check horizontal
        let from = if col > 3 { col - 3 } else { 0 };
        let to = if col < Self::COLS - 1 - 3 {
            col + 3
        } else {
            Self::COLS - 1
        };
        let mut count = 0;
        for i in from..=to {
            match self.board[row][i] {
                Cell::Some(f) if f == faction => {
                    count += 1;
                    if count >= 4 {
                        self.winner = Some(faction);
                        return true;
                    }
                }
                _ => count = 0,
            }
        }

        // Check vertical
        let from = if row > 3 { row - 3 } else { 0 };
        let to = row;
        let mut count = 0;
        for i in from..=to {
            match self.board[i][col] {
                Cell::Some(f) if f == faction => {
                    count += 1;
                    if count >= 4 {
                        self.winner = Some(faction);
                        return true;
                    };
                }
                _ => count = 0,
            }
        }

        // Check first diagonal
        let start_diff = cmp::min(cmp::min(row, col), 3);
        let nb = start_diff + 1 + cmp::min(cmp::min(Self::COLS - col, Self::ROWS - row), 3);
        let (start_row, start_col) = (row - start_diff, col - start_diff);
        let mut count = 0;
        for i in 0..nb {
            match self.board[start_row + i][start_col + i] {
                Cell::Some(f) if f == faction => {
                    count += 1;
                    if count >= 4 {
                        self.winner = Some(faction);
                        return true;
                    };
                }
                _ => count = 0,
            }
        }

        // Check second diagonal
        let start_diff = cmp::min(cmp::min(col, Self::ROWS - row), 3);
        let nb = start_diff + 1 + cmp::min(cmp::min(row, Self::COLS - col), 3);
        let (start_row, start_col) = (row + start_diff, col - start_diff);
        let mut count = 0;
        for i in 0..nb {
            match self.board[start_row - i][start_col + i] {
                Cell::Some(f) if f == faction => {
                    count += 1;
                    if count >= 4 {
                        self.winner = Some(faction);
                        return true;
                    };
                }
                _ => count = 0,
            }
        }

        false
    }

    fn drop_piece(&mut self) -> Option<(usize, usize)> {
        let col: usize = self.cursor_col.try_into().unwrap();
        if self.free_spots_in_col[col] <= 0 {
            return None;
        }

        let row = self.free_spots_in_col[col];
        let row: usize = row.try_into().unwrap();
        let row = Game::ROWS - row;

        self.board[row][col] = Cell::Some(self.curr_turn);
        self.free_spots_in_col[col] -= 1;
        self.free_spots -= 1;

        Some((row, col))
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left if self.cursor_col > 0 => {
                self.cursor_col -= 1;
            }
            Direction::Right if self.cursor_col < (Game::COLS - 1).try_into().unwrap() => {
                self.cursor_col += 1;
            }
            _ => (),
        }
    }

    fn get_input() -> Action {
        loop {
            let c = getch();

            match c as u8 as char {
                'q' => return Action::Quit,
                'a' => return Action::Move(Direction::Left),
                'd' => return Action::Move(Direction::Right),
                ' ' => return Action::Drop,
                _ => (),
            }
        }
    }

    fn draw_ui(&self) {
        mvprintw(0, 0, "Currently playing: ");
        match self.curr_turn {
            Faction::Blue => addstr_color("Blue", 1),
            Faction::Red => addstr_color("Red", 2),
        }

        mvprintw(
            LINES() - 2,
            0,
            "Use A and D to move the cursor, SPACE to place a piece",
        );
        mvprintw(LINES() - 1, 0, "Press Q to exit");

        let col: usize = self.cursor_col.try_into().unwrap();
        mv(self.free_spots_in_col[col] * 2 + 1, self.cursor_col * 2 + 1);
    }

    fn draw_board(&self) {
        mv(2, 0);

        // Top border
        addch(ACS_ULCORNER());
        for _i in 1..Game::COLS {
            addch(ACS_HLINE());
            addch(ACS_TTEE());
        }
        addch(ACS_HLINE());
        addch(ACS_URCORNER());
        addch('\n' as u32);

        for (i, row) in self.board.iter().enumerate().rev() {
            addch(ACS_VLINE());
            for elem in row {
                match elem {
                    Cell::Some(f) => match f {
                        Faction::Blue => addstr_color("O", 1),
                        Faction::Red => addstr_color("O", 2),
                    },
                    Cell::Empty => {
                        addch(' ' as u32);
                    }
                }
                addch(ACS_VLINE());
            }
            addch('\n' as u32);

            if i == 0 {
                break;
            }

            // Middle border
            addch(ACS_LTEE());
            for _i in 1..Game::COLS {
                addch(ACS_HLINE());
                addch(ACS_PLUS());
            }
            addch(ACS_HLINE());
            addch(ACS_RTEE());
            addch('\n' as u32);
        }

        // Bottom border
        addch(ACS_LLCORNER());
        for _i in 1..Game::COLS {
            addch(ACS_HLINE());
            addch(ACS_BTEE());
        }
        addch(ACS_HLINE());
        addch(ACS_LRCORNER());
    }
}

fn addstr_color(str: &str, color: i16) {
    attron(COLOR_PAIR(color));
    addstr(str);
    attroff(COLOR_PAIR(color));
}

fn main() {
    let mut game = Game {
        board: [[Cell::Empty; Game::COLS]; Game::ROWS],
        cursor_col: 0,
        free_spots_in_col: [Game::ROWS.try_into().unwrap(); Game::COLS],
        free_spots: Game::COLS * Game::ROWS,
        curr_turn: Faction::Blue,
        winner: None,
    };

    game.play();
}
