use ggez::glam::Vec2;

pub struct Camera {
	pub position: Vec2,
	pub virtual_size: Vec2,
	pub smoothing: f32,
}

impl Camera {
	pub fn new(virtual_size: Vec2) -> Self {
		Self {
			position: Vec2::ZERO,
			virtual_size,
			smoothing: 10.0,
		}
	}

	pub fn update(&mut self, target: Vec2, dt: f32) {
		let t = 1.0 - (-self.smoothing * dt).exp();

		self.position += (target - self.position) * t;
	}

	pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
		self.virtual_size / 2.0 + (world - self.position)
	}
}
