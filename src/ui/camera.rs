use ggez::glam::Vec2;

pub struct Camera {
	pub position: Vec2,
	pub smoothing: f32,
}

impl Camera {
	pub fn new(position: Vec2) -> Self {
		Self {
			position,
			smoothing: 5.0,
		}
	}

	pub fn update(&mut self, target: Vec2, dt: f32) {
		let t = 1.0 - (-self.smoothing * dt).exp();

		self.position += (target - self.position) * t;
	}

	pub fn world_to_screen(&self, ctx: &ggez::Context, world: Vec2) -> Vec2 {
		let window_size = ctx.gfx.drawable_size();
		let screen_center = Vec2::new(window_size.0 / 2.0, window_size.1 / 2.0);

		screen_center + (world - self.position)
	}
}
