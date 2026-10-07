use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};

use crate::ui::camera::Camera;

pub const TILE_SIZE: f32 = 24.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
	GrassEven,
	GrassOdd,
}

impl Tile {
	pub fn color(self) -> ggez::graphics::Color {
		match self {
			Tile::GrassEven => ggez::graphics::Color::from_rgb(50, 160, 50),
			Tile::GrassOdd => ggez::graphics::Color::from_rgb(60, 180, 60),
		}
	}
}

pub struct TileMap {
	pub width: usize,
	pub height: usize,
	pub tiles: Vec<Tile>,
}

impl TileMap {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			tiles: (0..width * height)
				.map(|i| {
					if i % 2 == 0 {
						Tile::GrassEven
					} else {
						Tile::GrassOdd
					}
				})
				.collect(),
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
		let screen_size = Vec2::from(ctx.gfx.drawable_size());
		let half = screen_size / 2.0;

		let left = camera.position.x - half.x;
		let right = camera.position.x + half.x;
		let top = camera.position.y - half.y;
		let bottom = camera.position.y + half.y;

		let start_x = (left / TILE_SIZE).floor() as i32;
		let end_x = (right / TILE_SIZE).ceil() as i32;

		let start_y = (top / TILE_SIZE).floor() as i32;
		let end_y = (bottom / TILE_SIZE).ceil() as i32;

		let mut builder = MeshBuilder::new();

		for y in start_y..end_y {
			for x in start_x..end_x {
				let Some(tile) = self.get(x, y) else {
					continue;
				};

				let screen_x = x as f32 * TILE_SIZE - camera.position.x + half.x;

				let screen_y = y as f32 * TILE_SIZE - camera.position.y + half.y;

				builder.rectangle(
					DrawMode::fill(),
					Rect::new(screen_x, screen_y, TILE_SIZE, TILE_SIZE),
					tile.color(),
				)?;
			}
		}

		let mesh = Mesh::from_data(ctx, builder.build());

		canvas.draw(&mesh, Vec2::ZERO);

		Ok(())
	}
}
