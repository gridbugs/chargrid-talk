use crate::{
    cf::*,
    game::Game,
    game_component::{GameComponent, GameOver},
    main_menu::*,
};
use chargrid_core::prelude::*;

/// Run the main menu, applying the chosed menu item (if any) to the game state. Yields
/// `Some(Some(app::Exit))` if "Quit" was chosen.
fn main_menu_component() -> CF<impl Component<State = Game, Output = Option<Option<app::Exit>>>> {
    cf(main_menu())
        .ignore_state()
        .catch_escape()
        .and_then(|or_escape| match or_escape {
            Err(Escape) | Ok(MenuItem::Resume) => Either3::A(val(None)),
            Ok(MenuItem::NewGame) => Either3::B(on_state(|game: &mut Game| {
                *game = Game::new();
                None
            })),
            Ok(MenuItem::Quit) => Either3::C(val(Some(app::Exit))),
        })
}

pub fn app() -> impl Component<State = (), Output = app::Output> {
    loop_unit(|| {
        cf(GameComponent) // run the game...
            .catch_escape() // ...until escape is pressed
            .and_then(|or_escape| match or_escape {
                // Exit the program when the game is over
                Ok(GameOver) => Either2::A(val(LoopControl::Break(app::Exit))),

                // Open the menu when escape is pressed
                Err(Escape) => {
                    Either2::B(main_menu_component().map(|maybe_exit| match maybe_exit {
                        None => LoopControl::Continue(()),
                        Some(app::Exit) => LoopControl::Break(app::Exit),
                    }))
                }
            })
    })
    .with_state(Game::new())
    .clear_each_frame()
    .exit_on_close()
}
