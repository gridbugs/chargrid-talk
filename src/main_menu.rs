use crate::menu::Menu;

#[derive(Clone, Copy)]
pub enum MenuItem {
    Resume,
    NewGame,
    Quit,
}

pub type MainMenu = Menu<MenuItem>;

pub fn main_menu() -> Menu<MenuItem> {
    Menu::new(vec![
        ("Resume".to_string(), MenuItem::Resume),
        ("New Game".to_string(), MenuItem::NewGame),
        ("Quit".to_string(), MenuItem::Quit),
    ])
}
