use ncurses::getch;

pub fn get_keys(exit_program: &mut bool, current_program: &mut u8) {
    let key: i32 = getch();
    match key {
        265 => *current_program += 1, // F1 pressed (add program)
        274 => *exit_program = true,  // F10 pressed (exit program)
        _ => {}
    }
}
