use crate::{
    cf_boxed::*,
    game::Game,
    game_component::{GameComponent, GameOver},
    main_menu::*,
};
use chargrid_core::prelude::*;

/// Run the main menu, applying the chosed menu item (if any) to the game state. Yields
/// `Some(Some(app::Exit))` if "Quit" was chosen.
fn main_menu_component() -> BoxedCF<Option<Option<app::Exit>>, Game> {
    boxed_cf(main_menu())
        .ignore_state()
        .catch_escape()
        .and_then(|or_escape| match or_escape {
            Err(Escape) | Ok(MenuItem::Resume) => val(None),
            Ok(MenuItem::NewGame) => on_state(|game: &mut Game| {
                *game = Game::new();
                None
            }),
            Ok(MenuItem::Quit) => val(Some(app::Exit)),
        })
}

pub fn app() -> BoxedCF<app::Output, ()> {
    loop_unit(|| {
        boxed_cf(GameComponent) // run the game...
            .catch_escape() // ...until escape is pressed
            .and_then(|or_escape| match or_escape {
                // Exit the program when the game is over
                Ok(GameOver) => val(LoopControl::Break(app::Exit)),

                // Open the menu when escape is pressed
                Err(Escape) => main_menu_component().map(|maybe_exit| match maybe_exit {
                    None => LoopControl::Continue(()),
                    Some(app::Exit) => LoopControl::Break(app::Exit),
                }),
            })
    })
    .with_state(Game::new())
    .clear_each_frame()
    .exit_on_close()
}
