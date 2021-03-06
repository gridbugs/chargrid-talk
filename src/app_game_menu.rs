use crate::{
    game::Game,
    game_component::{GameComponent, GameOver},
    main_menu::*,
};
use chargrid_core::prelude::*;

/// Type for keeping track of whether the game or the menu is currently being displayed
enum CurrentComponent {
    Game(GameComponent),
    MainMenu(MainMenu),
}

pub struct App {
    current_component: CurrentComponent,
    game: Game,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_component: CurrentComponent::Game(GameComponent),
            game: Game::new(),
        }
    }
}

impl Component for App {
    type Output = app::Output;
    type State = ();

    fn render(&self, &(): &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        fb.clear();
        // Only render the current component
        match &self.current_component {
            CurrentComponent::MainMenu(menu) => menu.render(&(), ctx, fb),
            CurrentComponent::Game(game_component) => game_component.render(&self.game, ctx, fb),
        }
    }

    fn update(&mut self, &mut (): &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                input::keys::ETX => return Some(app::Exit),
                _ => (),
            }
        }
        // State-machine hell!
        match &mut self.current_component {
            CurrentComponent::MainMenu(menu) => {
                if event.is_escape() {
                    self.current_component = CurrentComponent::Game(GameComponent);
                    return None;
                }
                if let Some(choice) = menu.update(&mut (), ctx, event) {
                    match choice {
                        MenuItem::Resume => {
                            self.current_component = CurrentComponent::Game(GameComponent);
                        }
                        MenuItem::NewGame => {
                            self.game = Game::new();
                            self.current_component = CurrentComponent::Game(GameComponent);
                        }
                        MenuItem::Quit => return Some(app::Exit),
                    }
                }
            }
            CurrentComponent::Game(game_component) => {
                if event.is_escape() {
                    self.current_component = CurrentComponent::MainMenu(main_menu());
                    return None;
                }
                if let Some(GameOver) = game_component.update(&mut self.game, ctx, event) {
                    return Some(app::Exit);
                }
            }
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
