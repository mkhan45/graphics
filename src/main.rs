mod graphics;
use graphics::GraphicsContext;

mod main_state;

extern crate winit;

const RADIUS: f32 = 0.175;

struct Data {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

fn update(ctx: &mut GraphicsContext, data: &mut Data) {
    ctx.new_circle([data.x, data.y], 0.2);

    data.x += data.dx;
    data.y += data.dy;

    if data.x + 0.2 >= 1.0 || data.x - 0.2 <= -1.0 {
        data.dx *= -1.0;
    }
    if data.y + 0.2 >= 1.0 || data.y - 0.2 <= -1.0 {
        data.dy *= -1.0;
    }
}

fn main() {
    let mut ctx = GraphicsContext::new();
    let mut data = Data{
        x: 0.0,
        y: 0.0,
        dx: 0.025,
        dy: -0.01,
    };

    ctx.run::<Data>(&mut data, &update);
}
