use ggez::GameResult;
use ggez::event;

mod game;
mod player;
mod ui;

use crate::game::state::GameState;
use crate::game::view::ViewState;

pub fn main() -> GameResult {
	let cb = ggez::ContextBuilder::new("g", "Nqtural");
	let (ctx, event_loop) = cb.build()?;

	let state = ViewState::new(GameState::default());

	event::run(ctx, event_loop, state)
}
