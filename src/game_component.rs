use crate::game::Game;
use chargrid_core::prelude::*;
use direction::CardinalDirection;

pub struct GameComponent;

impl Component for GameComponent {
    type Output = ();
    type State = Game;

    fn render(&self, game: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        game.render(ctx, fb);
    }

    fn update(&mut self, game: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                KeyboardInput::Left => game.player_walk(CardinalDirection::West),
                KeyboardInput::Right => game.player_walk(CardinalDirection::East),
                KeyboardInput::Up => game.player_walk(CardinalDirection::North),
                KeyboardInput::Down => game.player_walk(CardinalDirection::South),
                _ => (),
            }
        }
    }

    fn size(&self, _: &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
