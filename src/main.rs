use chargrid_core::prelude::*;
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

struct Args {
    version: u8,
    terminal: bool,
}

impl Args {
    fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                version = pos_req::<u8>("which version to run");
                terminal = flag("terminal");
            } in {
                Self { version, terminal }
            }
        }
    }
}

fn run_wgpu<C>(component: C)
where
    C: 'static + Component<State = (), Output = app::Output>,
{
    use chargrid_wgpu::*;
    let context = Context::new(Config {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin-custom.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA-custom.ttf").to_vec(),
        },
        title: "FP-Syd Talk".to_string(),
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
    context.run(component)
}

fn run_terminal<C>(component: C)
where
    C: 'static + Component<State = (), Output = app::Output>,
{
    use chargrid_ansi_terminal::*;
    let context = Context::new().unwrap();
    context.run(component, col_encode::FromTermInfoRgb)
}

fn main() {
    let Args { version, terminal } = Args::parser().with_help_default().parse_env_or_exit();
    if terminal {
        match version {
            0 => run_terminal(app_hello::App::new()),
            1 => run_terminal(app_game_no_menu::App::new()),
            2 => run_terminal(app_game_component::App::new()),
            3 => run_terminal(app_game_menu::App::new()),
            4 => run_terminal(app_game_menu_cf::app()),
            5 => run_terminal(app_game_menu_cf_boxed::app()),
            _ => panic!("unexpected version"),
        }
    } else {
        match version {
            0 => run_wgpu(app_hello::App::new()),
            1 => run_wgpu(app_game_no_menu::App::new()),
            2 => run_wgpu(app_game_component::App::new()),
            3 => run_wgpu(app_game_menu::App::new()),
            4 => run_wgpu(app_game_menu_cf::app()),
            5 => run_wgpu(app_game_menu_cf_boxed::app()),
            _ => panic!("unexpected version"),
        }
    }
}
