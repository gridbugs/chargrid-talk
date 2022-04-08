use crate::{
    cf_boxed::*,
    game::Game,
    game_component::{GameComponent, GameOver},
    main_menu::*,
};
use chargrid_core::prelude::*;

type AppCF<T> = BoxedCF<Option<T>, Game>;

fn game_component() -> AppCF<GameOver> {
    boxed_cf(GameComponent)
}

fn main_menu_component() -> AppCF<Option<app::Exit>> {
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
        game_component()
            .catch_escape()
            .and_then(|or_escape| match or_escape {
                Ok(GameOver) => val(LoopControl::Break(app::Exit)),
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
