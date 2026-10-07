use ggez::graphics::{Canvas, Color, DrawMode, Mesh, Rect};
use ggez::winit::keyboard::{Key, NamedKey};
use ggez::{Context, GameResult};

use crate::game::tile::{Tile, TileMap};
use crate::game::view::{View, ViewAction};
use crate::player::Player;
use crate::ui::camera::Camera;
use crate::ui::menu::MenuView;

pub struct GameState {
	camera: Camera,
	map: TileMap,
	player: Player,
}

impl GameState {
	pub fn new() -> Self {
		let mut map = TileMap::new(50, 50, Tile::Grass);

		for x in 5..15 {
			map.set(x, 8, Tile::Grass);
		}

		let player = Player::default();

		Self {
			camera: Camera::new(player.position),
			map,
			player,
		}
	}
}

impl View for GameState {
	fn update(&mut self, ctx: &mut Context) -> GameResult<ViewAction> {
		let dt = ctx.time.delta().as_secs_f32();
		self.player.update(dt);
		self.camera.update(self.player.position, dt);

		Ok(ViewAction::None)
	}

	fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		self.map.draw(ctx, canvas, &self.camera)?;

		let player_screen = self.camera.world_to_screen(ctx, self.player.position);

		canvas.draw(&player_sprite(ctx)?, player_screen);

		Ok(())
	}

	fn key_down(&mut self, _ctx: &mut Context, key: &Key) -> ViewAction {
		match key {
			Key::Named(NamedKey::Escape) => {
				return ViewAction::Push(Box::new(MenuView::new()));
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

fn player_sprite(ctx: &mut Context) -> GameResult<Mesh> {
	Mesh::new_rectangle(
		ctx,
		DrawMode::fill(),
		Rect::new(-10.0, -10.0, 20.0, 20.0),
		Color::from_rgb(255, 0, 0),
	)
}
