#![allow(warnings)]
mod lib;
use termimad::*;
use lib::libtui::{
	View
};
use std::io::{stdout, Write};
use termimad::crossterm::{
	event::{Event, MouseEvent},
	queue,
	terminal::{
		ClearType,
		Clear
	}
};
fn view_area() -> Area {
	let mut area = Area::full_screen();
	area.pad_for_max_width(120);
	area
}
fn output_markdown(skin: &MadSkin) -> bool {
	let mut q = Question::new("Output as Formatted Markdown?");
	q.add_answer('y',"**y**es");
	q.add_answer('n',"**n**o");
	q.set_default('y');
	let a = q.ask(skin).expect("y");
	match &a[..] {
		"y" => true,
		"n" => false,
		&_ => true
	}

}

fn show(skin: &MadSkin,txt: &str) {
	println!("{}", skin.inline(txt));
}

fn run_app(skin: MadSkin) -> Result<(),Error> {
	let mdout = output_markdown(&skin);
	if mdout {
		show(&skin,"### Using Formatted Markdown for Output");
	} else {
		show(&skin,"### Will Output as Plaintext");
	}
	let view = View::default();
	loop {		
		view.queue_on(&mut w)?;
		w.flush()?;
		match Event::read() {
			Ok(Event::Key(KeyEvent{code, ..})) => {
				match code {
					Esc => break,
					_ => {},
				}
			}
			Ok(Event::Resize(..)) => {
				queue!(w, Clear(ClearType::All)).expect("Error");
				view.resize(view_area());
			},
			_ => {}
		}
	}
	terminal::disable_raw_mode()?;
	queue!(w,Show)?;
	queue!(w, LeaveAlternateScreen)?;
	w.flush()?;
	Ok(())
}

fn make_skin() -> MadSkin {
	let mut skin = MadSkin::default();
	skin.table.align = Alignment::Center;
	skin.set_headers_fg(Green);
	skin.bold.set_fg(Red);
	skin.code_block.align = Alignment::Center;
	skin
}

fn main() -> Result<(),Error>{
	let skin = make_skin();
	run_app(skin)
}