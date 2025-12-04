mod grid;
mod coord;
mod camera;

use raylib::prelude::*;
use grid::Grid;
use camera::Camera;
use std::time::{Duration, Instant};

pub enum State {
    Editing,
    Running,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .resizable()
        .title("Game of Life")
        .build();

    let mut grid = Grid::new();

    let mut camera = Camera::new(0.0, 0.0);

    let mut last_step = Instant::now();
    let step_interval = Duration::from_millis(100);

    let mut state = State::Editing;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let dt = d.get_frame_time() as f64;

        // Camera controls
        let scroll = d.get_mouse_wheel_move_v();
        if d.is_key_pressed(KeyboardKey::KEY_W) || scroll.y > 0.0 { camera.inc_zoom(); }
        if d.is_key_pressed(KeyboardKey::KEY_S) || scroll.y < 0.0 { camera.dec_zoom(); }

        let cam_zoom = camera.get_zoom();
        let cam_offset = dt / cam_zoom * 1000.0;
        if d.is_key_down(KeyboardKey::KEY_RIGHT) { camera.offset(cam_offset, 0.0); }
        if d.is_key_down(KeyboardKey::KEY_LEFT)  { camera.offset(-cam_offset, 0.0); }
        if d.is_key_down(KeyboardKey::KEY_DOWN)  { camera.offset(0.0, cam_offset); }
        if d.is_key_down(KeyboardKey::KEY_UP)    { camera.offset(0.0, -cam_offset); }

        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let md = d.get_mouse_delta();
            camera.offset(
                -md.x as f64 / cam_zoom,
                -md.y as f64 / cam_zoom,
            );
        }

        let m_pos = {
            let x = d.get_mouse_position();
            (x.x as i32, x.y as i32)
        };

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            match state {
                State::Editing => state = State::Running,
                State::Running => state = State::Editing,
            }
        }

        d.clear_background(Color::WHITE);

        let screen_size = (d.get_screen_width(), d.get_screen_height());

        // Draw the live cells
        for pos in grid.get_live() {
            let screen_pos = camera.world_to_screen(*pos, screen_size);

            if screen_pos.0 < 0 || screen_pos.0 >= screen_size.0 { continue; }
            if screen_pos.1 < 0 || screen_pos.1 >= screen_size.1 { continue; }

            if grid.is_alive(&pos) {
                d.draw_rectangle(
                    screen_pos.0,
                    screen_pos.1,
                    cam_zoom as i32,
                    cam_zoom as i32,
                    Color::new(0, 0, 0, 255)
                );
            }
        }

        match state {
            State::Editing => {
                let w_pos = camera.screen_to_world(m_pos, screen_size);
                let s_pos = camera.world_to_screen(w_pos, screen_size);

                d.draw_rectangle(
                    s_pos.0,
                    s_pos.1,
                    cam_zoom as i32,
                    cam_zoom as i32,
                    Color::new(200, 200, 200, 255)
                );

                let cam_zoom_i = cam_zoom as i32;
                for i in 0..(screen_size.0 / cam_zoom_i) {
                    let x = i * cam_zoom_i;
                    d.draw_line(0, x, screen_size.0, x, Color::GRAY);
                    d.draw_line(x, 0, x, screen_size.1, Color::GRAY);
                }

                if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                    grid.spawn(w_pos);
                }
            }

            State::Running => if last_step.elapsed() >= step_interval {
                grid.step();
                last_step = Instant::now();
            }
        }
    }
}
