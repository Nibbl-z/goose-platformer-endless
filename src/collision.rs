use macroquad::prelude::*;

pub struct Rect {
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32
}

impl Rect {
    pub fn collides_with(&self, other : &Rect) -> bool {
        self.x < other.x + other.w && 
        other.x < self.x + self.w && 
        self.y < other.y + other.h &&
        other.y < self.y + self.h
    }

    pub fn in_camera_view(&self, camera : &Camera2D) -> bool {
        let camera_left = camera.target.x - (screen_width() / 2.0) / camera.zoom.x;
        let camera_right = camera.target.x + (screen_width() / 2.0) / camera.zoom.x;
        let camera_top = camera.target.y - (screen_height() / 2.0) / camera.zoom.y;
        let camera_bottom = camera.target.y + (screen_height() / 2.0) / camera.zoom.y;

        let camera_rect = Rect {
            x: camera_left,
            y: camera_top,
            w: camera_right - camera_left,
            h: camera_bottom - camera_top,
        };
        
        self.collides_with(&camera_rect)
    }
}