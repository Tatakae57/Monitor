use ncurses::{
    attroff, attron, curs_set, init_pair, initscr, keypad, mvaddch, mvhline, mvprintw, noecho,
    start_color, stdscr, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_PAIR, COLOR_WHITE,
};

// Starting
fn set_colors() {
    start_color();
    // Default
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    // First change
    init_pair(2, COLOR_WHITE, COLOR_BLUE);
    // Second change
    init_pair(3, COLOR_BLUE, COLOR_WHITE);
    // Help
    init_pair(4, COLOR_BLACK, COLOR_WHITE);
    // Info
    init_pair(5, COLOR_BLACK, COLOR_GREEN);
}

pub fn start_screen() {
    initscr(); // Starting screen
    noecho(); // Dont show keys
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Dont show cursor
    keypad(stdscr(), true); // Activate keypad
    set_colors();
}

// Draw functions
//  Information
pub fn clear_screen(x: i32, y: i32) {
    attron(COLOR_PAIR(1)); // Activate default color
    for yspace in 0..y {
        for xspace in 0..x {
            mvaddch(yspace, xspace, ' ' as u32);
        }
    }
    attroff(COLOR_PAIR(1)); // Deactivate color
}

pub fn draw_help(x: i32, y: i32) {
    // Positions
    let posx: (i32, i32, i32, i32) = ((x / 5), (x / 5) * 2, (x / 5) * 3, (x / 5) * 4);

    // Help
    attron(COLOR_PAIR(4));
    mvhline(y - 1, 0, ' ' as u32, x);
    mvprintw(y - 1, 0, "F1: Add").ok();
    mvprintw(y - 1, posx.0, "F2: Next").ok();
    mvprintw(y - 1, posx.1, "F3: Prev").ok();
    mvprintw(y - 1, posx.2, "F4: Close").ok();
    mvprintw(y - 1, posx.3, "F10: Exit").ok();
    attroff(COLOR_PAIR(4));

    // Info
    attron(COLOR_PAIR(5));
    mvhline(0, 0, ' ' as u32, x);
    mvprintw(0, 0, "State").ok();
    mvprintw(0, posx.0, "VmSize").ok();
    mvprintw(0, posx.1, "VmRSS").ok();
    mvprintw(0, posx.2, "VmData").ok();
    mvprintw(0, posx.3, "VmExe").ok();
    attroff(COLOR_PAIR(5));
}

//  Process
pub fn draw_proc() {}
