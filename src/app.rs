use chargrid_core::prelude::*;
use chargrid_runtime::app;

pub struct App;

impl Component for App {
    type Output = app::Output;
    type State = ();

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        fb.set_cell_relative_to_ctx(
            ctx,
            Coord::new(0, 0),
            0,
            RenderCell {
                character: Some('A'),
                style: Style::plain_text(),
            },
        );
    }

    fn update(&mut self, state: &mut Self::State, ctx: Ctx, event: Event) -> Self::Output {
        if event.is_escape() {
            return Some(app::Exit);
        }
        None
    }

    fn size(&self, state: &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
