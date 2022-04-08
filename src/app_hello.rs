use chargrid_core::prelude::*;

pub struct App {
    text: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            text: "Hello, FP-Syd!".to_string(),
        }
    }
}

impl Component for App {
    type Output = app::Output;
    type State = ();

    fn render(&self, &(): &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        // Move the cursor so the text will be centred
        let ctx = ctx.add_offset(Coord { x: 1, y: 7 });

        // Iterate over the characters in the string, writing each one to the frame buffer
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
                // Exit the program when a "close window" event is received
                input::keys::ETX => return Some(app::Exit),
                _ => (),
            }
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        // this component's size is the entire window
        ctx.bounding_box.size()
    }
}
