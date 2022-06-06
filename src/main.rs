extern crate ncurses;

use ncurses::*;

#[derive(Debug)]
enum Direction {
    Left, Right,
}

#[derive(Debug)]
enum Action {
    Quit,
    Move(Direction),
    Drop,
}

#[derive(Debug, Clone, Copy)]
enum Faction {
    Blue, Red
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
    Some(Faction), Empty
}

struct Game {
    board: [[Cell; Self::COLS]; Self::ROWS],
    cursor_col: i32,
    free_spots: [i32; Self::COLS],
    curr_turn: Faction,
}

impl Game {
    const ROWS: usize = 16;
    const COLS: usize = 20;

    fn play(&mut self) {
        initscr();
        cbreak();
        noecho();

        start_color();

        init_pair(1, COLOR_BLUE, COLOR_BLACK);
        init_pair(2, COLOR_RED, COLOR_BLACK);

        loop {
            clear();
            self.draw_board();
            self.draw_ui();
            refresh();

            let input = Self::get_input();

            match input {
                Action::Quit => break,
                Action::Move(dir) => self.move_cursor(dir),
                Action::Drop => self.drop_piece(),
            }
        }

        endwin();
    }

    fn has_space(&mut self, col: usize) -> bool {
        self.free_spots[col] > 0
    }

    fn drop_piece(&mut self) {
        let col: usize = self.cursor_col.try_into().unwrap();
        if !self.has_space(col) {
            return
        }

        let row = self.free_spots[col];
        let row: usize = row.try_into().unwrap();
        let row = Game::ROWS - row;

        self.board[row][col] = Cell::Some(self.curr_turn);
        self.free_spots[col] -= 1;
        self.curr_turn = self.curr_turn.swap();
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left if self.cursor_col > 0 => {
                self.cursor_col -= 1;
            }
            Direction::Right if self.cursor_col < (Game::COLS-1).try_into().unwrap() => {
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
        mvprintw(LINES() - 1, 0, "Press Q to exit");

        let col: usize = self.cursor_col.try_into().unwrap();
        mv(
            self.free_spots[col]*2, 
            self.cursor_col*2+1
        );
    }

    fn draw_board(&self) {
        mv(1, 0);

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
                    Cell::Some(f) => {
                        match f {
                            Faction::Blue => {
                                attron(COLOR_PAIR(1));
                                addch('O' as u32);
                                attroff(COLOR_PAIR(1));
                            }
                            Faction::Red => {
                                attron(COLOR_PAIR(2));
                                addch('O' as u32);
                                attroff(COLOR_PAIR(2));
                            }
                        }
                    }
                    Cell::Empty => {addch(' ' as u32);}
                }
                addch(ACS_VLINE());
            }
            addch('\n' as u32);

            if i == 0 {
                break;
            }

            addch(ACS_LTEE());
            for _i in 1..Game::COLS {
                addch(ACS_HLINE());
                addch(ACS_PLUS());
            }
            addch(ACS_HLINE());
            addch(ACS_RTEE());
            addch('\n' as u32);
        }

        addch(ACS_LLCORNER());
        for _i in 1..Game::COLS {
            addch(ACS_HLINE());
            addch(ACS_BTEE());
        }
        addch(ACS_HLINE());
        addch(ACS_LRCORNER());
    }
}

fn main() {
    let mut game = Game {
        board: [[Cell::Empty; Game::COLS]; Game::ROWS],
        cursor_col: 0,
        free_spots: [Game::ROWS.try_into().unwrap(); Game::COLS],
        curr_turn: Faction::Blue,
    };

    game.play();

    // attron(COLOR_PAIR(2));
    // mvaddch(0, 0, ACS_DIAMOND());
    // attroff(COLOR_PAIR(2));
}