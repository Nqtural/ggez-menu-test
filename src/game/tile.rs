use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::graphics::{Canvas, DrawMode, Mesh, Rect};
use ggez::{Context, GameResult};

use crate::ui::camera::Camera;

pub const TILE_SIZE: f32 = 32.0;

#[derive(Clone, Copy)]
pub enum Tile {
	Grass,
}

impl Tile {
	pub fn color(self) -> Color {
		match self {
			Tile::Grass => Color::from_rgb(60, 180, 60),
		}
	}
}

pub struct TileMap {
	pub width: usize,
	pub height: usize,
	pub tiles: Vec<Tile>,
}

impl TileMap {
	pub fn new(width: usize, height: usize, default: Tile) -> Self {
		Self {
			width,
			height,
			tiles: vec![default; width * height],
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
		if x < 0 || y < 0 {
			return None;
		}

		let x = x as usize;
		let y = y as usize;

		if x >= self.width || y >= self.height {
			return None;
		}

		Some(self.tiles[y * self.width + x])
	}

	pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
		if x < 0 || y < 0 {
			return;
		}

		let x = x as usize;
		let y = y as usize;

		if x >= self.width || y >= self.height {
			return;
		}

		self.tiles[y * self.width + x] = tile;
	}

	pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, camera: &Camera) -> GameResult {
		let half = Vec2::from(ctx.gfx.drawable_size()) / 2.0;

		let left = camera.position.x - half.x;
		let right = camera.position.x + half.x;
		let top = camera.position.y - half.y;
		let bottom = camera.position.y + half.y;

		let start_x = (left / TILE_SIZE).floor() as i32;
		let end_x = (right / TILE_SIZE).ceil() as i32;

		let start_y = (top / TILE_SIZE).floor() as i32;
		let end_y = (bottom / TILE_SIZE).ceil() as i32;

		for y in start_y..end_y {
			for x in start_x..end_x {
				let Some(tile) = self.get(x, y) else {
					continue;
				};

				let world_position = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

				let screen_position = camera.world_to_screen(ctx, world_position);

				let mesh = Mesh::new_rectangle(
					ctx,
					DrawMode::fill(),
					Rect::new(screen_position.x, screen_position.y, TILE_SIZE, TILE_SIZE),
					tile.color(),
				)?;

				canvas.draw(&mesh, Vec2::ZERO);
			}
		}

		Ok(())
	}
}
