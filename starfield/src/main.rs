use nannou::prelude::*;

const MAX_SPEED: f32 = 100.0;
const STAR_SIZE: f32 = 8.0;
const NUM: usize = 1000;

#[derive(Default, Debug, Copy, Clone)]
struct Star {
    pos: Point3,
    pz: f32,
}

impl Star {
    fn new(bounds: Rect) -> Star {
        let z = random_range(1.0, bounds.w());
        Star {
            pos: Point3 { 
                x: random_range(bounds.left(), bounds.right()),
                y: random_range(bounds.top(), bounds.bottom()),
                z,
            },
            pz: z,
        }
    }

    fn update(&mut self, speed: f32, bounds: Rect) {
        self.pz = self.pos.z;
        self.pos.z -= speed;
        if self.pos.z < 1.0 {
            self.pos.z = bounds.w();
            self.pz = self.pos.z;
            self.pos.x = random_range(bounds.left(), bounds.right());
            self.pos.y = random_range(bounds.top(), bounds.bottom());
        }
        //dbg!(self);
    }

    fn view(&self, draw_ellipse: bool, bounds: Rect, draw: &Draw) {
        let x = map_range(self.pos.x / self.pos.z, 0.0, 1.0, 0.0, bounds.w());
        let y = map_range(self.pos.y / self.pos.z, 0.0, 1.0, 0.0, bounds.h());
        let radius = map_range(self.pos.z, 0.0, bounds.w(), STAR_SIZE, 0.0);
        let brightness = 1.0 - map_range(self.pos.z, 0.0, bounds.w(), 0.0, 1.0);
        //dbg!(x, y, radius);
        if draw_ellipse {
            draw.ellipse()
                .x_y(x, y)
                .radius(radius)
                .hsv(0.0, 0.0, brightness)
                .resolution(8);
        }

        let px = map_range(self.pos.x / self.pz, 0.0, 1.0, 0.0, bounds.w());
        let py = map_range(self.pos.y / self.pz, 0.0, 1.0, 0.0, bounds.h());

        draw.line()
            .points(Point2 {x: px, y: py }, Point2 {x, y})
            .hsv(0.0, 0.0,  brightness);
    }
}

struct Model {
    stars: Vec<Star>,
}

fn model(app: &App) -> Model {
    let mut vec: Vec<Star> = Vec::with_capacity(NUM);
    (0..vec.capacity()).for_each(|_| vec.push(Star::new(app.window_rect())));
    Model { stars: vec }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    let speed = map_range(app.mouse.x, bounds.left(), bounds.right(), 0.0, MAX_SPEED);
    model.stars.iter_mut().for_each(|s| s.update(speed, bounds));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.stars.iter().for_each(|s| s.view(app.mouse.buttons.right().is_down(), app.window_rect(), &draw));
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
