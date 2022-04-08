const CELL_SIZE: f64 = 48.;

mod app_game_component;
mod app_game_menu;
mod app_game_menu_cf;
mod app_game_no_menu;
mod app_hello;
mod cf;
mod game;
mod game_component;
mod main_menu;
mod menu;

fn main() {
    use chargrid_wgpu::*;
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

    context.run(app_game_menu_cf::app());
}
