extern crate ncurses;

use ncurses::*;

#[derive(Debug)]
enum Direction {
    Up, Down, Left, Right,
}

#[derive(Debug)]
enum Action {
    Quit,
    Move(Direction),
}

struct Game {
    board: [[i32; Self::COLS]; Self::ROWS],
    cursor: (i32, i32),
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
        let (mut x, mut y) = self.cursor;

        match direction {
            Direction::Up => y = if y > 0 {y-1} else {y},
            Direction::Down => y = y+1,
            Direction::Left => x = if x > 0 {x-1} else {x},
            Direction::Right => x = x+1,
        }

        self.cursor = (x, y);
    }

    fn get_input() -> Action {
        loop {
            let c = getch();

            match c as u8 as char {
                'q' => return Action::Quit,
                'w' => return Action::Move(Direction::Up),
                'a' => return Action::Move(Direction::Left),
                's' => return Action::Move(Direction::Down),
                'd' => return Action::Move(Direction::Right),
                _ => (),
            }
        }
    }

    fn draw_ui(&self) {
        let (x, y) = self.cursor;
        mv(y, x);
    }

    fn draw_board(&self) {
        
    }
}

fn main() {
    let mut game = Game {
        board: [[0; Game::COLS]; Game::ROWS],
        cursor: (0, 0),
    };

    game.play();

    // start_color();

    // init_pair(1, COLOR_BLUE, COLOR_BLACK);
    // init_pair(2, COLOR_RED, COLOR_BLACK);

    // attron(COLOR_PAIR(2));
    // mvaddch(0, 0, ACS_DIAMOND());
    // attroff(COLOR_PAIR(2));
}