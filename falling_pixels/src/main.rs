use nannou::image;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Clone)]
enum Atom {
    Air,
    Sand
}

struct Model {
    texture: wgpu::Texture,

    atoms: Vec<Atom>,

    world_size: Vector2<usize>
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

fn update(app: &App, model: &mut Model, _update: Update) {
    
    for y in (0..model.world_size.y).rev() {
        for x in 0..model.world_size.x {
            // let atom = model.atom_at(x, y);
            if app.mouse.x < 50.0 {
                model.set_atom(Atom::Air, x, y);
                continue;
            }

            model.set_atom(Atom::Sand, x, y);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();

    let image = image::ImageBuffer::from_fn(model.world_size.x as u32, model.world_size.y as u32, |x, y| {
        let px = x as usize;
        let py = y as usize;

        let atom = model.atom_at(px, py); 
        
        match atom {
            Atom::Air => nannou::image::Rgba([0, 0, 0, std::u16::MAX]),
            Atom::Sand => nannou::image::Rgba([std::u16::MAX, std::u16::MAX, std::u16::MAX, std::u16::MAX]),
        }
    });

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
