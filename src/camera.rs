use crate::coord::Coord;

#[derive(Debug)]
pub struct Camera {
    x: f64,
    y: f64,
    /// Size in pixels per cell
    zoom: f64,
    zoom_exp: i32,
}

impl Camera {
    pub fn new(x: f64, y: f64) -> Self {
        let zoom_exp = 5;
        Self {
            x,
            y,
            zoom: 2.0f64.powi(zoom_exp),
            zoom_exp,
        }
    }

    fn clamp_to_grid(&self, x: f64) -> f64 {
        (x / self.zoom).floor() * self.zoom
    }

    pub fn world_to_screen(&self, pos: Coord, screen_size: (i32, i32)) -> (i32, i32) {
        let zoom_i = self.zoom as i32;
        let cx = ((screen_size.0 / (2 * zoom_i)) * zoom_i) as f64;
        let cy = ((screen_size.1 / (2 * zoom_i)) * zoom_i) as f64;

        // Cell difference
        let dx = pos.x as f64 - self.x.floor();
        let dy = pos.y as f64 - self.y.floor();

        // Subtract half a cell to align the camera to cell corners
        let sx = cx + dx * self.zoom;
        let sy = cy + dy * self.zoom;

        (sx as i32, sy as i32)
    }

    pub fn screen_to_world(&self, pos: (i32, i32), screen_size: (i32, i32)) -> Coord {
        let zoom_i = self.zoom as i32;
        let cx = ((screen_size.0 / (2 * zoom_i)) * zoom_i) as f64;
        let cy = ((screen_size.1 / (2 * zoom_i)) * zoom_i) as f64;

        let dx = (pos.0 as f64 - cx) / self.zoom;
        let dy = (pos.1 as f64 - cy) / self.zoom;

        let wx = self.x.floor() + dx;
        let wy = self.y.floor() + dy;

        Coord {
            x: wx.floor() as i64,
            y: wy.floor() as i64,
        }
    }

    pub fn offset(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn inc_zoom(&mut self) {
        if self.zoom_exp >= 5 { return; }
        self.zoom_exp += 1;
        self.zoom = 2.0f64.powi(self.zoom_exp);
    }

    pub fn dec_zoom(&mut self) {
        if self.zoom_exp <= 0 { return; }
        self.zoom_exp -= 1;
        self.zoom = 2.0f64.powi(self.zoom_exp);
    }

    #[inline]
    pub const fn get_zoom(&self) -> f64 {
        self.zoom
    }
}
