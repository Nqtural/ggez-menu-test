use ggez::GameError;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::input::keyboard::KeyInput;
use ggez::winit::keyboard::Key;
use ggez::winit::keyboard::NamedKey;
use ggez::{Context, GameResult};
use strum::Display;

const ACCELERATION_RATIO: f32 = 5.0;
const MAX_BASE_VELOCITY: f32 = 100.0;

enum ViewAction {
	None,
	Push(Box<dyn View>),
	Pop,
	Quit,
}

trait View {
	fn update(&mut self, ctx: &mut Context) -> GameResult<ViewAction>;
	fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
	fn key_down(&mut self, ctx: &mut Context, key: &Key) -> ViewAction;
	fn key_up(&mut self, ctx: &mut Context, key: &Key) -> ViewAction;
	fn transparent(&self) -> bool;
}

struct ViewState {
	views: Vec<Box<dyn View>>,
}

impl ViewState {
	fn new<V: View + 'static>(view: V) -> Self {
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

struct MenuView {
	items: Vec<MenuItem>,
	selected: usize,
}

#[derive(Display)]
enum MenuItem {
	#[strum(to_string = "Resume")]
	Resume,

	#[strum(to_string = "Quit")]
	Quit,
}

impl MenuView {
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

#[derive(Default)]
struct GameState {
	player: Player,
}

#[derive(Default)]
struct Player {
	position: Vec2,
	velocity: Vec2,
	input: PlayerInput,
	velocity_modifier: f32,
}

#[derive(Default)]
struct PlayerInput {
	up: bool,
	left: bool,
	down: bool,
	right: bool,
}

impl View for GameState {
	fn update(&mut self, ctx: &mut Context) -> GameResult<ViewAction> {
		let dt = ctx.time.delta().as_secs_f32();

		let max_velocity = MAX_BASE_VELOCITY * (1.0 + self.player.velocity_modifier);

		let mut acceleration = Vec2::ZERO;

		if self.player.input.up {
			acceleration.y -= ACCELERATION_RATIO * max_velocity;
		}
		if self.player.input.down {
			acceleration.y += ACCELERATION_RATIO * max_velocity;
		}
		if self.player.input.left {
			acceleration.x -= ACCELERATION_RATIO * max_velocity;
		}
		if self.player.input.right {
			acceleration.x += ACCELERATION_RATIO * max_velocity;
		}

		self.player.velocity += acceleration * dt;

		if acceleration.x == 0.0 {
			self.player.velocity.x = approach_zero(
				self.player.velocity.x,
				ACCELERATION_RATIO * max_velocity * dt,
			);
		}

		if acceleration.y == 0.0 {
			self.player.velocity.y = approach_zero(
				self.player.velocity.y,
				ACCELERATION_RATIO * max_velocity * dt,
			);
		}

		if self.player.velocity.length() > max_velocity {
			self.player.velocity = self.player.velocity.normalize() * max_velocity;
		}

		self.player.position += self.player.velocity * dt;

		Ok(ViewAction::None)
	}

	fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		let background = Mesh::new_rectangle(
			ctx,
			DrawMode::fill(),
			Rect::new(0.0, 0.0, 800.0, 600.0),
			Color::from_rgba(0, 200, 0, 128),
		)?;
		canvas.draw(&background, Vec2::ZERO);

		canvas.draw(&player_sprite(ctx)?, self.player.position);
		Ok(())
	}

	fn key_down(&mut self, _ctx: &mut Context, key: &Key) -> ViewAction {
		match key {
			Key::Named(NamedKey::Escape) => {
				return ViewAction::Push(Box::new(MenuView {
					selected: 0,
					items: vec![MenuItem::Resume, MenuItem::Quit],
				}));
			}
			Key::Character(character) => match character.to_string().as_str() {
				"w" => self.player.input.up = true,
				"a" => self.player.input.left = true,
				"s" => self.player.input.down = true,
				"d" => self.player.input.right = true,
				_ => (),
			},
			_ => (),
		};

		ViewAction::None
	}

	fn key_up(&mut self, _ctx: &mut Context, key: &Key) -> ViewAction {
		if let Key::Character(character) = key {
			match character.to_string().as_str() {
				"w" => self.player.input.up = false,
				"a" => self.player.input.left = false,
				"s" => self.player.input.down = false,
				"d" => self.player.input.right = false,
				_ => (),
			}
		}

		ViewAction::None
	}

	fn transparent(&self) -> bool {
		false
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

fn player_sprite(ctx: &mut Context) -> Result<Mesh, GameError> {
	Mesh::new_rectangle(
		ctx,
		DrawMode::fill(),
		Rect::new(0.0, 0.0, 20.0, 20.0),
		Color::from_rgb(255, 0, 0),
	)
}

fn approach_zero(base: f32, delta: f32) -> f32 {
	if base > 0.0 {
		(base - delta.abs()).max(0.0)
	} else {
		(base + delta.abs()).min(0.0)
	}
}

pub fn main() -> GameResult {
	let cb = ggez::ContextBuilder::new("g", "Nqtural");
	let (ctx, event_loop) = cb.build()?;
	let state = ViewState::new(GameState::default());
	event::run(ctx, event_loop, state)
}
