use chargrid_core::prelude::*;

pub struct Menu<T: Clone> {
    items: Vec<(String, T)>,
    current_index: usize,
}

impl<T: Clone> Menu<T> {
    pub fn new(items: Vec<(String, T)>) -> Self {
        Self {
            items,
            current_index: 0,
        }
    }

    fn up(&mut self) {
        self.current_index = (self.current_index - 1).max(0);
    }

    fn down(&mut self) {
        self.current_index = (self.current_index + 1).min(self.items.len() - 1);
    }

    fn current_item(&self) -> T {
        self.items[self.current_index].1.clone()
    }
}

impl<T: Clone> Component for Menu<T> {
    type State = ();
    type Output = Option<T>;

    fn render(&self, &(): &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        let normal_style = Style::plain_text().with_bold(false);
        let selected_style = Style::plain_text().with_bold(true);
        for (i, entry) in self.items.iter().enumerate() {
            let style = if i == self.current_index {
                selected_style
            } else {
                normal_style
            };
            let start_coord = Coord { x: 0, y: i as i32 };
            if i == self.current_index {
                fb.set_cell_relative_to_ctx(
                    ctx,
                    start_coord,
                    0,
                    RenderCell {
                        character: Some('>'),
                        style,
                    },
                );
            }
            for (j, ch) in entry.0.chars().enumerate() {
                fb.set_cell_relative_to_ctx(
                    ctx,
                    start_coord
                        + Coord {
                            x: j as i32 + 2,
                            y: 0,
                        },
                    0,
                    RenderCell {
                        character: Some(ch),
                        style,
                    },
                );
            }
        }
    }

    fn update(&mut self, &mut (): &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(keyboard_input) = event.keyboard_input() {
            match keyboard_input {
                KeyboardInput::Up => self.up(),
                KeyboardInput::Down => self.down(),
                input::keys::RETURN => return Some(self.current_item()),
                _ => (),
            }
        }
        None
    }

    fn size(&self, &(): &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}
