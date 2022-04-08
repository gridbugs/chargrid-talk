use chargrid_core::prelude::*;
use chargrid_runtime::app;

pub struct App {
    text: String,
}

impl App {
    fn new() -> Self {
        Self {
            text: "Hello, FP-Syd!".to_string(),
        }
    }
}

pub fn app() -> App {
    App::new()
}

impl Component for App {
    type Output = app::Output;
    type State = ();

    fn render(&self, &(): &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        let ctx = ctx.add_offset(Coord { x: 1, y: 7 });
        for (i, ch) in self.text.chars().enumerate() {
            fb.set_cell_relative_to_ctx(
                ctx,
                Coord { x: i as i32, y: 0 },
                0,
                RenderCell::default().with_character(ch).with_bold(true),
            );
        }
    }

    fn update(&mut self, &mut (): &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                input::keys::ETX => return Some(app::Exit),
                _ => (),
            }
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
