use nannou::image;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Clone, PartialEq)]
enum Atom {
    Air,
    Sand,
    Water,
}

struct Model {
    texture: wgpu::Texture,

    atoms: Vec<Atom>,

    world_size: Vector2<usize>,
}

impl Model {
    fn atom_at(&self, x: usize, y: usize) -> &Atom {
        &self.atoms[x + y * self.world_size.x]
    }

    fn set_atom(&mut self, atom: Atom, x: usize, y: usize) {
        self.atoms[x + y * self.world_size.x] = atom;
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let window = app.main_window();
    let win = window.rect();

    let world_size = Vector2 {
        x: win.w() as usize,
        y: win.h() as usize,
    };

    let texture = wgpu::TextureBuilder::new()
        .size([world_size.x as u32, world_size.y as u32])
        .format(Frame::TEXTURE_FORMAT)
        .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
        .build(window.swap_chain_device());

    let atoms = vec![Atom::Air; world_size.x + world_size.y * world_size.x];

    dbg!(&world_size);

    Model {
        texture,
        atoms,
        world_size,
    }
}

fn inside_world(model: &Model, x: i32, y: i32, dx: i32, dy: i32) -> Option<(usize, usize)> {
    let nx = x + dx;
    let ny = y + dy;

    if nx >= 0 && nx < model.world_size.x as i32 && ny >= 0 && ny < model.world_size.y as i32 {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
}

fn check_next_positions(model: &Model, x: i32, y: i32, steps: &[(i32, i32)]) -> Vec<(usize, usize)> {
    steps
        .iter()
        .filter_map(|(dx, dy)| inside_world(&model, x, y, *dx, *dy))
        .collect()
}

fn try_move(model: &mut Model, atom: &Atom, x: usize, y: usize, spots: &[(usize, usize)], rand: u128) -> bool {
    let spots_ordered = if rand % 2 == 0 {
        spots.iter().rev().collect::<Vec<_>>()
    } else {
        spots.iter().collect()
    };

    for (nx, ny) in spots_ordered {
        if *model.atom_at(*nx, *ny) == Atom::Air {
            model.set_atom(atom.clone(), *nx, *ny);
            model.set_atom(Atom::Air, x, y);
            return true;
        }
        if *atom == Atom::Sand && *model.atom_at(*nx, *ny) == Atom::Water {
            model.set_atom(atom.clone(), *nx, *ny);
            model.set_atom(Atom::Water, x, y);
            return true;
        }
    }
    false
}

fn try_moves(mut model: &mut Model, atom: &Atom, x: usize, y: usize, spots_set: &[Vec<(usize, usize)>], rand: u128) {
    for spots in spots_set {
        if try_move(&mut model, &atom, x, y, &spots, rand) { return; }
    }
}

fn update(app: &App, mut model: &mut Model, update: Update) {

    // consts
    model.set_atom(Atom::Sand, model.world_size.x / 2, model.world_size.y / 2);
    model.set_atom(Atom::Water, model.world_size.x / 2 + 20, 20);
    model.set_atom(Atom::Water, model.world_size.x / 2 - 20, 20);


    // drawing
    let m_pos = app.mouse.position();
        let world_pos_x = map_range(
            m_pos.x,
            app.window_rect().left(),
            app.window_rect().right(),
            0,
            model.world_size.x,
        );
        let world_pos_y = map_range(
            m_pos.y,
            app.window_rect().top(),
            app.window_rect().bottom(),
            0,
            model.world_size.y,
        );

    if app.mouse.buttons.left().is_down() {
        if let Some((nx, ny)) = inside_world(model, world_pos_x as i32, world_pos_y as i32, 0, 0) {
            model.set_atom(Atom::Water, nx, ny);
        }
    } else if app.mouse.buttons.right().is_down() {
        if let Some((nx, ny)) = inside_world(model, world_pos_x as i32, world_pos_y as i32, 0, 0) {
            model.set_atom(Atom::Sand, nx, ny);
        }
    }

    let down = vec![(0i32, 1i32)];
    let diag = vec![(-1i32, 1i32), (1i32, 1i32)];
    let across = vec![(-1i32, 0i32), (1i32, 0i32)];

    let sand = vec![&down, &diag];
    let water = vec![&down, &diag, &across];

    // sim every pixel top to bottom (reverse direction of gravity)
    for y in (0..model.world_size.y).rev() {
        for x in 0..model.world_size.x {
            let current = model.atom_at(x, y);
            
            let move_set = match current {
                Atom::Air => continue,
                Atom::Sand => &sand,
                Atom::Water => &water,
            };

            let xy = (x as i32, y as i32);

            let spots_sets: Vec<_> = move_set.iter().map(|steps| {
                check_next_positions(&model, xy.0, xy.1, &steps)
            }).collect();

            let atom = current.clone();
            try_moves(&mut model, &atom, x, y, &spots_sets, random());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();

    let image = image::ImageBuffer::from_fn(
        model.world_size.x as u32,
        model.world_size.y as u32,
        |x, y| {
            let px = x as usize;
            let py = y as usize;

            let atom = model.atom_at(px, py);

            match atom {
                Atom::Air => nannou::image::Rgba([0, 0, 0, std::u16::MAX]),
                Atom::Sand => nannou::image::Rgba([
                    std::u16::MAX,
                    std::u16::MAX,
                    std::u16::MAX,
                    std::u16::MAX,
                ]),
                Atom::Water =>  nannou::image::Rgba([
                    0,
                    0,
                    std::u16::MAX,
                    std::u16::MAX,
                ])
            }
        },
    );

    let flat_samples = image.as_flat_samples();
    let img_bytes = slice_as_bytes(flat_samples.as_slice());
    model.texture.upload_data(
        app.main_window().swap_chain_device(),
        &mut *frame.command_encoder(),
        img_bytes,
    );

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.text(format!("{:.0}", app.fps()).as_str())
        .x_y(win.left() * 0.9, win.top() * 0.9)
        .color(RED)
        .font_size(16);

    draw.to_frame(app, &frame).unwrap();
}

fn slice_as_bytes(s: &[u16]) -> &[u8] {
    let len = s.len() * std::mem::size_of::<u16>();
    let ptr = s.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
