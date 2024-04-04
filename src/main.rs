use macroquad::prelude::*;

#[macroquad::main("Daniels spel")]
async fn main() {
    loop {
        clear_background(GREEN);
        next_frame().await
    }
}
