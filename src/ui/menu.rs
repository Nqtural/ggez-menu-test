use ggez::{
	Context, GameResult,
	glam::Vec2,
	graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment},
	winit::keyboard::{Key, NamedKey},
};
use strum::Display;

use crate::game::view::{View, ViewAction};

pub struct MenuView {
	items: Vec<MenuItem>,
	selected: usize,
}

#[derive(Display)]
pub enum MenuItem {
	#[strum(to_string = "Resume")]
	Resume,

	#[strum(to_string = "Quit")]
	Quit,
}

impl MenuView {
	pub fn new() -> Self {
		Self {
			items: vec![MenuItem::Resume, MenuItem::Quit],
			selected: 0,
		}
	}

	fn select(&self) -> ViewAction {
		match self.items[self.selected] {
			MenuItem::Resume => ViewAction::Pop,
			MenuItem::Quit => ViewAction::Quit,
		}
	}

	fn move_up(&mut self) {
		self.selected = self.selected.checked_sub(1).unwrap_or(self.items.len() - 1);
	}

	fn move_down(&mut self) {
		self.selected = (self.selected + 1) % self.items.len();
	}
}

impl View for MenuView {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<ViewAction> {
		Ok(ViewAction::None)
	}

	fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		const FONT_SIZE: f32 = 32.0;
		const LINE_GAP: f32 = 32.0;

		let background = Mesh::new_rectangle(
			ctx,
			DrawMode::fill(),
			Rect::new(0.0, 0.0, 800.0, 600.0),
			Color::from_rgba(0, 0, 0, 200),
		)?;
		canvas.draw(&background, Vec2::ZERO);

		let center = 300.0;
		let len = self.items.len() as f32;
		let height = (len * FONT_SIZE) + ((len - 1.0) * LINE_GAP);
		let start = center - (height / 2.0);
		for (index, item) in self.items.iter().enumerate() {
			canvas.draw(
				&Text::new(TextFragment::new(item.to_string()).scale(FONT_SIZE).color(
					if self.selected == index {
						Color::BLUE
					} else {
						Color::WHITE
					},
				)),
				DrawParam::default()
					.dest([400.0, start + (index as f32 * LINE_GAP)])
					.offset([0.5, 0.0]),
			);
		}

		Ok(())
	}

	fn key_down(&mut self, _ctx: &mut Context, key: &Key) -> ViewAction {
		if let Key::Named(named_key) = key {
			return match named_key {
				NamedKey::Enter => self.select(),
				NamedKey::Escape => ViewAction::Pop,
				NamedKey::ArrowDown => {
					self.move_down();
					return ViewAction::None;
				}
				NamedKey::ArrowUp => {
					self.move_up();
					return ViewAction::None;
				}
				_ => ViewAction::None,
			};
		}

		ViewAction::None
	}

	fn key_up(&mut self, _ctx: &mut Context, _key: &Key) -> ViewAction {
		ViewAction::None
	}

	fn transparent(&self) -> bool {
		true
	}
}
