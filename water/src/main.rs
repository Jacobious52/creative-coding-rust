use nannou::prelude::*;
use nannou::noise::{NoiseFn, OpenSimplex};

struct Model {
    noise: OpenSimplex,
    t: f64,
}

fn model(_app: &App) -> Model {
    Model {
        noise: OpenSimplex::new(),
        t: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //let bounds = app.window_rect();
    model.t += 0.01;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    let bounds = app.window_rect();
    
    draw.background().color(BLACK);

    let mut py = 0.0;
    for x in (bounds.left() as i32)..(bounds.right() as i32) {
        let xf: f64 = std::convert::From::from(x);
        let mut fy = 0.0;
        let mut range = 0.0;

        for yy in 1..3 {
            let yyf: f64 = std::convert::From::from(yy);
            if yy == 1 {
                fy += model.noise.get([(xf + model.t * 1000.0) * (0.01 * yyf), model.t]) * (1.0 / yyf);
            } else {
                fy += model.noise.get([xf * (0.01 * yyf), model.t]) * (1.0 / yyf);
            }
            range += 1.0 / yyf;
        }
        
        let pym = map_range(py, -range, range, -200.0, 200.0);
        let ym = map_range(fy, -range, range, -200.0, 200.0);

        draw.line()
            .points(Point2{ x: (xf - 1.0) as f32, y: pym as f32 }, Point2{ x: xf as f32, y: ym as f32 })
            .rgb(0.0, 0.0, 1.0);

        draw.line()
            .points(Point2{ x: xf as f32, y: bounds.bottom() }, Point2{ x: xf as f32, y: ym as f32 })
            .rgb(0.0, 0.0, 1.0);

        py = fy;
    }

    draw.to_frame(app, frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
