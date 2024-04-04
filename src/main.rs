use macroquad::prelude::*;

#[macroquad::main("Daniels spel")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
