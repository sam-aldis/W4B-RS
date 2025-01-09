use {

	anyhow::{self},
	crokey::{key, KeyCombination},
	std::io::Write,
	std::slice::Iter,
	termimad::*,
	termimad::crossterm::{
		event::{Event, MouseEvent},
		queue,
		terminal::{
			Clear,
			ClearType,
		},
	},
};

pub struct View {
	area: Area,
	drawable: bool,
	label_skin: MadSkin,
	text: MadView,
	input_label_area: Area,
	input: InputField,
}

impl Default for View {
	fn default() -> Self {
		let mut label_skin = MadSkin::default();
		label_skin.paragraph.align = Alignment::Center;
		label_skin.headers[1].align = Alignment::Center;
		let mut view = Self {
			area: Area::uninitialized(),
			drawable: false,
			label_skin,
			text: MadView::from(
				String::from(""),
				Area::uninitialized(),
				MadSkin::default()
			),
			input_label_area: Area::uninitialized(),
			input: InputField::default()
		};
		view.input.set_normal_style(CompoundStyle::with_fgbg(gray(22), gray(2)));
		view.set_focus();
		view
	}
}

impl View {
	pub fn new(area: Area) -> Self {
		let mut view = Self::default();
		view.resize(area);
		view
	}
	fn set_focus(&mut self) {
		self.input.set_focus(true);
	}
	// pub fn write(&self) -> Result<()> {
	// 	self.write_on(&mut std::io::stdout())
	// }
	// pub fn write_on<W: Write>(&self, w: &mut W) -> Result<()> {
	// 	let text = self.skin.area_text(&self.markdown, &self.area);
	// 	let mut text_view = TextView::from(&self.area, &text);
	// 	text_view.scroll = self.scroll;
	// 	text_view.write_on(w)?;
	// 	Ok(())
	// }
	pub fn resize(&mut self,area: Area) -> bool {
		if self.area == area {
			return false;
		}
		self.drawable = area.width >= 20 && area.height >= 15;
		if self.drawable {
			let full_width = area.width - 3;
			let half_width = (area.width - 6) /2;
			let h = area.height - 6;
			let txt_area = Area::new(3, 1, area.width - 3, h);
			self.text.resize(&txt_area);
			let y = txt_area.bottom() + 2;
			self.input_label_area = Area::new(3, y, full_width, 1);
			let y = y + 2;
			self.input.change_area(3, y, full_width);
		}
		self.area = area;
		true
	}
	pub fn apply_timed_event(&mut self, timed_event: TimedEvent) -> bool {
		match timed_event.event {
			Event::Resize(w, h) => self.resize(Area::new(0,0,w,h)),
			_ => false,
		}
	}
	pub fn queue_on<W: Write>(&mut self, w: &mut W) -> anyhow::Result<()> {
		queue!(w, Clear(ClearType::All))?;
		if self.drawable {
			let skin = &self.label_skin;
			self.text.write_on(w)?;
			skin.write_in_area_on(w, "## Input:", &self.input_label_area)?;
			self.input.display_on(w)?;
		} else {
			self.text.skin.write_in_area_on(
					w,
					"*Sorry* terminal is waaayy tooo small!",
					&self.area
				)?;
		}
		Ok(())
	}
	pub fn update_text<W: Write>(&mut self, txt: String,w: &mut W) -> bool {
		self.text = MadView::from(
				txt,
				Area::uninitialized(),
				MadSkin::default()
			);
		self.resize(self.area.clone());
		self.queue_on(w);
		true
	}
}