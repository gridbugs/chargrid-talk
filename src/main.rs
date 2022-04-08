use chargrid_wgpu::*;
use meap::Parser;

const CELL_SIZE: f64 = 48.;

mod app_game_component;
mod app_game_menu;
mod app_game_menu_cf;
mod app_game_menu_cf_boxed;
mod app_game_no_menu;
mod app_hello;
mod cf;
mod cf_boxed;
mod game;
mod game_component;
mod main_menu;
mod menu;

fn main() {
    let version = meap::prelude::pos_req::<u8>("which version to run")
        .with_help_default()
        .parse_env_or_exit();
    let context = Context::new(Config {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin-custom.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA-custom.ttf").to_vec(),
        },
        title: "Hello Chargrid".to_string(),
        window_dimensions_px: Dimensions {
            width: 768.,
            height: 768.,
        },
        cell_dimensions_px: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        font_scale: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        underline_width_cell_ratio: 0.1,
        underline_top_offset_cell_ratio: 0.8,
        resizable: false,
        force_secondary_adapter: false,
    });
    match version {
        0 => context.run(app_hello::App::new()),
        1 => context.run(app_game_no_menu::App::new()),
        2 => context.run(app_game_component::App::new()),
        3 => context.run(app_game_menu::App::new()),
        4 => context.run(app_game_menu_cf::app()),
        5 => context.run(app_game_menu_cf_boxed::app()),
        _ => panic!("unexpected version"),
    }
}
