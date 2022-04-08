use crate::game::Game;
use chargrid_core::prelude::*;
use chargrid_runtime::app;
use direction::CardinalDirection;

pub struct App {
    game: Game,
}

impl App {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}

impl Component for App {
    type Output = app::Output;
    type State = ();

    fn render(&self, &(): &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        // Clear the frame buffer at the start of each frame
        fb.clear();
        self.game.render(ctx, fb);
    }

    fn update(&mut self, &mut (): &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                // Exit the program when a "close window" event is received
                input::keys::ETX => return Some(app::Exit),

                // Game controls
                KeyboardInput::Left => self.game.player_walk(CardinalDirection::West),
                KeyboardInput::Right => self.game.player_walk(CardinalDirection::East),
                KeyboardInput::Up => self.game.player_walk(CardinalDirection::North),
                KeyboardInput::Down => self.game.player_walk(CardinalDirection::South),
                _ => (),
            }
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
