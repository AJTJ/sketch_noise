use nannou::noise::*;
use nannou::prelude::*;

const WIDTH: u32 = 1250;
const HEIGHT: u32 = 650;
const N_ELEMENTS: usize = 500;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1, 1)
        .run();
}

struct Element {
    position: Vec2,
}

impl Element {
    pub fn new(p: Vec2) -> Self {
        Element { position: p }
    }
}

struct Model {
    elements: Vec<Element>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut elements = Vec::new();
    for el in 0..N_ELEMENTS {
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

    let m = app.mouse.position();
    // println!("pos: {}", m);

    if m.x != 0.0 && m.y != 0.0 {
        for el in model.elements.iter_mut() {
            let e = el.position;

            let main_distance =
                ((Pow::<f32>::pow(m.x - e.x, 2.0)) + (Pow::<f32>::pow(m.y - e.y, 2.0))).sqrt();
            let inc = 4.0;

            let x_new = e.x + ((inc / main_distance) * (m.x - e.x));
            let y_new = e.y + ((inc / main_distance) * (m.y - e.y));

            // el.position += vec2((random::<f32>() - 0.5), (random::<f32>() - 0.5));
            el.position = vec2(x_new, y_new);
            // println!("p: {}", el.position)
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(BLACK);

    let val = model.noise.get([42.4, 37.7, 2.8]);

    // let time = app.elapsed_frames() as f32 / 60.0;
    let time = app.duration.since_start.as_millis();

    for el in model.elements.iter() {
        draw.ellipse().xy(el.position).radius(4.0).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

// m_pos.x * angle.cos(),
// m_pos.y * angle.sin(),

// let (hei, wid) = app.main_window().outer_size_pixels();

// for i in 0..10 {
//     let angle = i as f32 * 0.1 * TAU + time;
//     draw.ellipse()
//         .x_y(
//             (m_pos.x + 100.0) * angle.cos(),
//             (m_pos.y + 100.0) * angle.sin(),
//         )
//         .color(WHITE);
// }
