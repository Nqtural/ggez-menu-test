use ggez::event;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::KeyInput;
use ggez::winit::keyboard::Key;
use ggez::{Context, GameResult};

pub enum ViewAction {
	None,
	Push(Box<dyn View>),
	Pop,
	Quit,
}

pub trait View {
	fn update(&mut self, ctx: &mut Context) -> GameResult<ViewAction>;
	fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
	fn key_down(&mut self, ctx: &mut Context, key: &Key) -> ViewAction;
	fn key_up(&mut self, ctx: &mut Context, key: &Key) -> ViewAction;
	fn transparent(&self) -> bool;
}

pub struct ViewState {
	views: Vec<Box<dyn View>>,
}

impl ViewState {
	pub fn new<V: View + 'static>(view: V) -> Self {
		Self {
			views: vec![Box::new(view)],
		}
	}

	fn current(&mut self) -> &mut dyn View {
		self.views.last_mut().unwrap().as_mut()
	}

	fn handle_action(&mut self, ctx: &mut Context, action: ViewAction) {
		match action {
			ViewAction::None => {}

			ViewAction::Push(view) => {
				self.views.push(view);
			}

			ViewAction::Pop => {
				if self.views.len() > 1 {
					self.views.pop();
				}
			}

			ViewAction::Quit => {
				ctx.request_quit();
			}
		}
	}
}

impl event::EventHandler for ViewState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let action = self.current().update(ctx)?;
		self.handle_action(ctx, action);

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let start = self
			.views
			.iter()
			.rposition(|view| !view.transparent())
			.unwrap_or(0);

		let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

		for view in &self.views[start..] {
			view.draw(ctx, &mut canvas)?;
		}

		canvas.finish(ctx)?;

		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
		let action = self.current().key_down(ctx, &input.event.logical_key);
		self.handle_action(ctx, action);

		Ok(())
	}

	fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
		let action = self.current().key_up(ctx, &input.event.logical_key);
		self.handle_action(ctx, action);

		Ok(())
	}
}
