use bracket_lib::prelude::*;

/// The game state
struct State {}

impl GameState for State {
    /// ctx provides functions for interacting with the game display
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State {})
}
