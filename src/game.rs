use chargrid_core::{Ctx, FrameBuffer, RenderCell, Rgba32, Style};
use direction::{CardinalDirection, Direction};
use grid_2d::{Coord, Grid, Size};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_isaac::Isaac64Rng;

#[derive(Clone, Copy)]
enum Cell {
    Floor,
    Wall,
    Stairs,
}

pub struct Game {
    rng: Isaac64Rng,
    world: Grid<Cell>,
    player_coord: Coord,
}

impl Game {
    pub fn new() -> Self {
        let size = Size::new(16, 16);
        let rng = Isaac64Rng::from_entropy();
        let world = Grid::new_copy(size, Cell::Floor);
        let player_coord = Coord::new(0, 0);
        let mut s = Self {
            rng,
            world,
            player_coord,
        };
        s.generate();
        s
    }

    pub fn render(&self, ctx: Ctx, fb: &mut FrameBuffer) {
        for (coord, cell) in self.world.enumerate() {
            let render_cell = if coord == self.player_coord {
                RenderCell {
                    character: Some('@'),
                    style: Style::plain_text().with_bold(true),
                }
            } else {
                let (ch, style) = match cell {
                    Cell::Floor => (' ', Style::plain_text()),
                    Cell::Wall => (
                        ' ',
                        Style::plain_text().with_background(Rgba32::new_grey(127)),
                    ),
                    Cell::Stairs => ('>', Style::plain_text().with_bold(true)),
                };
                RenderCell {
                    character: Some(ch),
                    style,
                }
            };
            fb.set_cell_relative_to_ctx(ctx, coord, 0, render_cell);
        }
    }

    pub fn player_walk(&mut self, direction: CardinalDirection) {
        let dest_coord = self.player_coord + direction.coord();
        if let Some(&dest_cell) = self.world.get(dest_coord) {
            match dest_cell {
                Cell::Wall => (),
                Cell::Floor => self.player_coord = dest_coord,
                Cell::Stairs => self.generate(),
            }
        }
    }

    fn generate(&mut self) {
        let conway_grid = loop {
            let candidate = self.genrate_candidate();
            let num_alive = candidate.iter().filter(|&&x| x).count();
            if num_alive > 64 {
                break candidate;
            }
        };
        let mut alive_coords = conway_grid
            .enumerate()
            .filter(|&(_, &alive)| alive)
            .map(&|(coord, _)| coord)
            .collect::<Vec<_>>();
        alive_coords.shuffle(&mut self.rng);
        self.player_coord = alive_coords.pop().unwrap();
        let stairs_coord = alive_coords.pop().unwrap();
        for (cell, &conway_cell) in self.world.iter_mut().zip(conway_grid.iter()) {
            if conway_cell {
                *cell = Cell::Floor;
            } else {
                *cell = Cell::Wall;
            }
        }
        *self.world.get_checked_mut(stairs_coord) = Cell::Stairs;
    }

    fn genrate_candidate(&mut self) -> Grid<bool> {
        let mut conway_grid = Grid::new_fn(self.world.size(), |_| self.rng.gen::<bool>());
        let mut conway_grid_tmp = conway_grid.clone();
        for _ in 0..6 {
            for ((coord, &cell), next_cell) in
                conway_grid.enumerate().zip(conway_grid_tmp.iter_mut())
            {
                let mut alive_count = 0;
                for direction in Direction::all() {
                    if let Some(&nei) = conway_grid.get(coord + direction.coord()) {
                        alive_count += nei as u8;
                    }
                }
                if cell {
                    *next_cell = alive_count >= 4 && alive_count <= 8;
                } else {
                    *next_cell = alive_count == 5;
                }
            }
            std::mem::swap(&mut conway_grid, &mut conway_grid_tmp);
        }
        for _ in 0..1 {
            for ((coord, &cell), next_cell) in
                conway_grid.enumerate().zip(conway_grid_tmp.iter_mut())
            {
                let mut alive_count = 0;
                for direction in Direction::all() {
                    if let Some(&nei) = conway_grid.get(coord + direction.coord()) {
                        alive_count += nei as u8;
                    }
                }
                if !cell {
                    *next_cell = alive_count > 4;
                }
            }
            std::mem::swap(&mut conway_grid, &mut conway_grid_tmp);
        }
        let mut seen = Grid::new_copy(self.world.size(), false);
        let mut biggest = Vec::new();
        for (coord, &cell) in conway_grid.enumerate() {
            if cell && !*seen.get_checked(coord) {
                let mut stack = vec![coord];
                let mut chunk = vec![coord];
                *seen.get_checked_mut(coord) = true;
                while let Some(coord) = stack.pop() {
                    for direction in CardinalDirection::all() {
                        let nei_coord = coord + direction.coord();
                        if let Some(&nei_cell) = conway_grid.get(nei_coord) {
                            if nei_cell && !*seen.get_checked(nei_coord) {
                                *seen.get_checked_mut(nei_coord) = true;
                                stack.push(nei_coord);
                                chunk.push(nei_coord);
                            }
                        }
                    }
                }
                if chunk.len() > biggest.len() {
                    biggest = chunk;
                }
            }
        }
        for cell in conway_grid.iter_mut() {
            *cell = false;
        }
        for coord in biggest {
            if coord.x > 0
                && coord.y > 0
                && (coord.x as u32) < conway_grid.width() - 1
                && (coord.y as u32) < conway_grid.height() - 1
            {
                *conway_grid.get_checked_mut(coord) = true;
            }
        }
        conway_grid
    }
}
