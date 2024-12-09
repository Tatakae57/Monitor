mod keyboard;
mod reader;
mod screen;

use keyboard::get_keys;
use ncurses::{endwin, getmaxyx, refresh, stdscr};
use screen::{clear_screen, draw_help, draw_proc, start_screen};

fn main() {
    // "Global" variables
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut exit_program: bool = false;
    let mut current_program: u8 = 0;

    // Starting program
    start_screen();

    // Loop
    while !exit_program {
        getmaxyx(stdscr(), &mut y, &mut x);
        clear_screen(x, y);
        draw_help(x, y);
        draw_proc();
        refresh();
        get_keys(&mut exit_program, &mut current_program);
    }
    endwin();
}
