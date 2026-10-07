use ggez::glam::Vec2;

const ACCELERATION_RATIO: f32 = 5.0;
const MAX_BASE_VELOCITY: f32 = 100.0;

#[derive(Default)]
pub struct Player {
	pub position: Vec2,
	pub velocity: Vec2,
	pub input: PlayerInput,
	pub velocity_modifier: f32,
}

#[derive(Default)]
pub struct PlayerInput {
	pub up: bool,
	pub left: bool,
	pub down: bool,
	pub right: bool,
}

impl Player {
	pub fn update(&mut self, dt: f32) {
		let max_velocity = MAX_BASE_VELOCITY * (1.0 + self.velocity_modifier);

		let mut acceleration = Vec2::ZERO;

		if self.input.up {
			acceleration.y -= ACCELERATION_RATIO * max_velocity;
		}
		if self.input.down {
			acceleration.y += ACCELERATION_RATIO * max_velocity;
		}
		if self.input.left {
			acceleration.x -= ACCELERATION_RATIO * max_velocity;
		}
		if self.input.right {
			acceleration.x += ACCELERATION_RATIO * max_velocity;
		}

		self.velocity += acceleration * dt;

		if acceleration.x == 0.0 {
			self.velocity.x =
				approach_zero(self.velocity.x, ACCELERATION_RATIO * max_velocity * dt);
		}

		if acceleration.y == 0.0 {
			self.velocity.y =
				approach_zero(self.velocity.y, ACCELERATION_RATIO * max_velocity * dt);
		}

		if self.velocity.length() > max_velocity {
			self.velocity = self.velocity.normalize() * max_velocity;
		}

		self.position += self.velocity * dt;
	}
}

fn approach_zero(base: f32, delta: f32) -> f32 {
	if base > 0.0 {
		(base - delta.abs()).max(0.0)
	} else {
		(base + delta.abs()).min(0.0)
	}
}
