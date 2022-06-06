extern crate ncurses;
use ncurses::*;

pub fn addstr_color(str: &str, color: i16) {
    attron(COLOR_PAIR(color));
    addstr(str);
    attroff(COLOR_PAIR(color));
}
