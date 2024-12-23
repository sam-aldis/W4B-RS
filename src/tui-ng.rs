use std::char;
use ncurses::*;

fn main() {
	initscr();
	raw();
	keypad(stdscr(), true);
	noecho();
	addstr("Char:").unwrap();
	let ch = getch();
	if ch == KEY_F(1)
	{
		attron(A_BOLD | A_BLINK);
		addstr("\nF1 Pressed").unwrap();
		attroff(A_BOLD | A_BLINK);
		addstr("...").unwrap();
	} else {
		addstr("\nKey Pressed:").unwrap();
		attron(A_BOLD | A_BLINK);
		addstr(format!("{}\n"), char::from_u32(ch as u32).expect("Invalid Char").as_ref()).unwrap();
		attroff(A_BOLD | A_BLINK);
	}
	refresh();
	endwin();
}