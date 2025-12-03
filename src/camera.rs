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

    pub fn world_to_screen(&self, pos: &Coord) -> (i32, i32) {
        let sx = (pos.x - self.x as i64) as f64 * self.zoom;
        let sy = (pos.y - self.y as i64) as f64 * self.zoom;
        (sx as i32, sy as i32)
    }

    pub fn screen_to_world(&self, pos: &Coord) -> Coord {
        // Convert screen coordinates back to world coordinates
        let wx = self.x + (pos.x as f64 / self.zoom);
        let wy = self.y + (pos.y as f64 / self.zoom);

        // Round down to integer cell coordinates
        Coord::new(
            wx.floor() as i64,
            wy.floor() as i64,
        )
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
        self.zoom_exp -= 1;
        self.zoom = 2.0f64.powi(self.zoom_exp);
    }

    #[inline]
    pub const fn get_zoom(&self) -> f64 {
        self.zoom
    }
}
