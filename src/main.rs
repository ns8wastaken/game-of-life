mod grid;
mod coord;
mod camera;

use raylib::prelude::*;
use coord::Coord;
use grid::Grid;
use camera::Camera;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Game of Life")
        .build();

    let mut grid = Grid::new();
    grid.spawn(Coord::new(1,0));
    grid.spawn(Coord::new(2,0));
    grid.spawn(Coord::new(0,1));
    grid.spawn(Coord::new(1,1));
    grid.spawn(Coord::new(1,2));

    let mut camera = Camera::new(0.0, 0.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_pressed(KeyboardKey::KEY_W) { camera.inc_zoom(); }
        if d.is_key_pressed(KeyboardKey::KEY_S) { camera.dec_zoom(); }

        let dt = d.get_frame_time() as f64;

        let cam_offset = dt / camera.get_zoom() * 1000.0;
        if d.is_key_down(KeyboardKey::KEY_RIGHT) { camera.offset(cam_offset, 0.0); }
        if d.is_key_down(KeyboardKey::KEY_LEFT)  { camera.offset(-cam_offset, 0.0); }
        if d.is_key_down(KeyboardKey::KEY_DOWN)  { camera.offset(0.0, cam_offset); }
        if d.is_key_down(KeyboardKey::KEY_UP)    { camera.offset(0.0, -cam_offset); }

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let md = d.get_mouse_delta();
            camera.offset(
                -md.x as f64 / camera.get_zoom(),
                -md.y as f64 / camera.get_zoom(),
            );
        }

        d.clear_background(Color::WHITE);

        for pos in grid.get_live() {
            let screen_pos = camera.world_to_screen(&pos);

            if screen_pos.0 < 0 || screen_pos.0 >= d.get_screen_width() { continue; }
            if screen_pos.1 < 0 || screen_pos.1 >= d.get_screen_height() { continue; }

            if grid.is_alive(&pos) {
                d.draw_rectangle(
                    screen_pos.0,
                    screen_pos.1,
                    camera.get_zoom() as i32,
                    camera.get_zoom() as i32,
                    Color::new(0, 0, 0, 255)
                );
            }
        }

        grid.step();
    }
}
