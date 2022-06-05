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
}

struct Game {
    board: [[i32; Self::COLS]; Self::ROWS],
    cursor_col: i32,
}

impl Game {
    const ROWS: usize = 10;
    const COLS: usize = 10;

    fn play(&mut self) {
        initscr();
        cbreak();
        noecho();

        loop {
            clear();
            self.draw_board();
            self.draw_ui();
            refresh();

            let input = Self::get_input();

            match input {
                Action::Quit => break,
                Action::Move(dir) => self.move_cursor(dir),
            }
        }

        endwin();
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left if self.cursor_col > 1 => {
                self.cursor_col -= 2;
            }
            Direction::Right if self.cursor_col < (2*Game::COLS-1).try_into().unwrap() => {
                self.cursor_col += 2;
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
                _ => (),
            }
        }
    }

    fn draw_ui(&self) {
        mv(0, self.cursor_col);
    }

    fn draw_board(&self) {
        mv(1, 0);
        for row in self.board {
            addch(ACS_VLINE());
            for elem in row {
                addstr(&format!("{}", elem));
                addch(ACS_VLINE());
            }
            addch('\n' as u32);
        }
    }
}

fn main() {
    let mut game = Game {
        board: [[0; Game::COLS]; Game::ROWS],
        cursor_col: 1,
    };

    game.play();

    // start_color();

    // init_pair(1, COLOR_BLUE, COLOR_BLACK);
    // init_pair(2, COLOR_RED, COLOR_BLACK);

    // attron(COLOR_PAIR(2));
    // mvaddch(0, 0, ACS_DIAMOND());
    // attroff(COLOR_PAIR(2));
}