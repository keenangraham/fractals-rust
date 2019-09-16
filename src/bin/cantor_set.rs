use piston_window::*;


const WIDTH: f64 = 1500.0;
const HEIGHT: f64 = 1000.0;
const LEVEL: u32 = 10;
const LINECOLOR: [f32; 4] = [168.0 / 255.0, 177.0 / 255.0, 245.0 / 255.0, 1.0];
const LINEWIDTH: f64 = 30.0;
const LINESEP: f64 = 100.0;

// TODO: Make line struct and add draw method.

struct Params<'a, G: 'a>
where G: Graphics {
    linesep: f64,
    linewidth: f64,
    linecolor: [f32; 4],
    transform: [[f64; 3]; 2],
    graphics: &'a mut G
}


fn draw_line<G>(x: f64, y: f64, length: f64, params: &mut Params<G>)
where G: Graphics {
    line(
        params.linecolor,
        params.linewidth,
        [x, y, x + length, y],
        params.transform,
        params.graphics
    );
    
}


fn draw_cantor_set<G>(x: f64, y: f64, length: f64, level: u32, mut params: &mut Params<G>)
where G: Graphics {
    if level > 0 {
        draw_line(x, y, length, &mut params);
        draw_cantor_set(x, y + params.linesep, length / 3.0, level - 1, &mut params);
        draw_cantor_set(x + 2.0 * length / 3.0, y + params.linesep, length / 3.0, level - 1, &mut params);
    }
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new(
            "Fractal!",
            [WIDTH, HEIGHT]
        ).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            let mut params = Params {
                linesep: LINESEP,
                linewidth: LINEWIDTH,
                linecolor: LINECOLOR,
                transform: context.transform,
                graphics: graphics
            };
            draw_cantor_set(0.0, 0.0, WIDTH, LEVEL, &mut params);
        });
    }
}
