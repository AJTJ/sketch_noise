use nannou::noise::*;
use nannou::prelude::*;

const WIDTH: u32 = 1250;
const HEIGHT: u32 = 650;
const N_ELEMENTS: usize = 50;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1, 1)
        .run();
}

struct Element {
    positions: Vec<Vec2>,
}

impl Element {
    pub fn new(p: Vec2) -> Self {
        Element { positions: vec![p] }
    }
}

struct Model {
    elements: Vec<Element>,
    noise: Perlin,
}

fn model(_app: &App) -> Model {
    let mut noise = Perlin::new();
    noise = noise.set_seed(9000);
    let mut elements = Vec::new();
    for _ in 0..N_ELEMENTS {
        let elem = Element::new(vec2(
            (random::<f32>() - 0.5) * WIDTH as f32,
            (random::<f32>() - 0.5) * HEIGHT as f32,
        ));
        elements.push(elem);
    }

    Model { elements, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // move it to the corner of the screen
    app.main_window().set_outer_position_pixels(0, 0);
    // resize it so that it is now visible
    app.main_window().set_inner_size_pixels(WIDTH, HEIGHT);

    for el in model.elements.iter_mut() {
        let sn = 0.005;

        // USING NOISE
        el.positions.clear();
        el.positions.push(vec2(
            (random::<f32>() - 0.5) * WIDTH as f32,
            (random::<f32>() - 0.5) * HEIGHT as f32,
        ));

        // moving STEPS at a time
        for _ in 0..500 {
            let e = el.positions[0];
            let new = e + vec2(
                model.noise.get([sn * e.x as f64, sn * e.y as f64, 0.0]) as f32,
                model.noise.get([sn * e.x as f64, sn * e.y as f64, 2.0]) as f32,
            );
            el.positions.insert(0, new);
        }
    }
}

fn get_color(time: f32) -> Rgba {
    rgba(
        (time / 1000.0).sin() + 0.5,
        ((time + 200.0) / 800.0).sin() + 0.5,
        ((time + 150.0) / 700.0).sin() + 0.5,
        1.0,
    )
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // TIME AS FRAMES
    // let time = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(WIDTH as f32, HEIGHT as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.005));

    // TIME AS MILLISECONDS
    let time = app.duration.since_start.as_millis();

    for el in model.elements.iter() {
        // POLYLINE: A single line composed of many line segments
        draw.polyline()
            .points(el.positions.iter().cloned())
            .color(get_color(time as f32));
    }

    draw.to_frame(app, &frame).unwrap();
}

// DRAWING A CIRCLE OF CIRCLES
// for i in 0..10 {
//     let angle = i as f32 * 0.1 * TAU + time;
//     draw.ellipse()
//         .x_y(
//             (m_pos.x + 100.0) * angle.cos(),
//             (m_pos.y + 100.0) * angle.sin(),
//         )
//         .color(WHITE);
// }
