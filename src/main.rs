use macroquad::prelude::*;

#[macroquad::main("Daniels spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 400.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(GREEN);
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);
        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}
