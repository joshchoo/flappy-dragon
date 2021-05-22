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

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // TODO: 2021-05-21 implement
    fn play(&mut self, _ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    // TODO: 2021-05-21 implement
    fn dead(&self, _ctx: &mut BTerm) {}

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
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
