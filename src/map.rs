use macroquad::prelude::*;
use crate::{collision::Rect, player::Player};

pub struct Platform {
    pub rect : Rect
}

impl Platform {
    pub fn new(x : f32, y : f32, w : f32, h : f32) -> Platform {
        Platform {
            rect : Rect {x, y, w, h}
        }
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }

    pub fn update(&self, player : &mut Player) {
        if self.rect.collides_with(&player.rect) {
            player.land(self);
        }
    }
}