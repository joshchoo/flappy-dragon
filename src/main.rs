use bracket_lib::prelude::*;

/// The game state
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    // TODO: 2021-05-21 implement
    fn main_menu(&self, ctx: &mut BTerm) {}

    // TODO: 2021-05-21 implement
    fn play(&self, ctx: &mut BTerm) {}

    // TODO: 2021-05-21 implement
    fn dead(&self, ctx: &mut BTerm) {}
}

/// The GameMode state machine
enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    /// ctx provides functions for interacting with the game display
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
