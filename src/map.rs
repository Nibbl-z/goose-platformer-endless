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
            if player.rect.y >= self.rect.y + self.rect.h - 10.0 && player.rect.x >= self.rect.x - 40.0 && player.rect.x <= self.rect.x + self.rect.w - 10.0 {
                player.hit_head(); 
            } else if player.rect.y >= self.rect.y && player.rect.y < self.rect.y + self.rect.h {
                player.hit_side_wall(self);
            } else if player.rect.y + 50.0 >= self.rect.y {
                player.land(self);
            }
        }
    }
    
    pub fn generate_next(&self, direction : bool, previous_batch : &[Platform]) -> Platform {
        let mut give_up = 0;

        loop {
            give_up += 1;
            let mut collides = false;
            let x = if direction == true { 
                rand::gen_range(100.0, 200.0) + self.rect.x + self.rect.w 
            } else {
                self.rect.x - self.rect.w - rand::gen_range(100.0, 200.0)
            };
            let y = rand::gen_range(-self.rect.h * 1.5, self.rect.h * -0.5) + self.rect.y + self.rect.h / 2.0;
        
            let w = rand::gen_range(150.0, 400.0);
            let h = rand::gen_range(150.0, 500.0);
            
            let created_platform = Platform::new(x, y, w, h);
            
            for platform in previous_batch {
                if (Rect {
                    x : x - 40.0, 
                    y : y - 40.0, 
                    w : w + 80.0,
                    h : h + 80.0
                }).collides_with(&platform.rect) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                break created_platform
            }

            if give_up >= 50 {
                break created_platform
            }
        }
        
    }
}