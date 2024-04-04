use macroquad::prelude::*;

#[macroquad::main("Daniels spel")]
async fn main() {
    loop {
        clear_background(YELLOW);
        next_frame().await
    }
}
