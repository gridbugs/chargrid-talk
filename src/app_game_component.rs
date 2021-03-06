use crate::{
    game::Game,
    game_component::{GameComponent, GameOver},
};
use chargrid_core::prelude::*;

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
        fb.clear();
        GameComponent.render(&self.game, ctx, fb);
    }

    fn update(&mut self, &mut (): &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                input::keys::ETX => return Some(app::Exit),
                _ => (),
            }
        }
        if let Some(GameOver) = GameComponent.update(&mut self.game, ctx, event) {
            return Some(app::Exit);
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
