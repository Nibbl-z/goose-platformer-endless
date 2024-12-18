use macroquad::prelude::*;

use crate::player::Player;

const DEFAULT_SPEED : f32 = 100.0;

pub struct Lava {
    pub y : f32,
    speed : f32,
    texture : Texture2D
}

impl Lava {
    pub async fn new() -> Lava {
        Lava {
            y : 600.0,
            speed : DEFAULT_SPEED,
            texture : load_texture("img/lava.png").await.unwrap(),
        }
    }
    
    pub fn update(&mut self, dt : f32, player : &mut Player) {
        self.speed = if player.rect.y <= self.y - 600.0 {
            DEFAULT_SPEED * 7.0
        } else {
            DEFAULT_SPEED
        };
        
        if player.rect.y + 50.0 >= self.y {
            player.died = true;
            
            if player.died_time == -1.0 {
                player.just_died = true;
                player.died_time = macroquad::time::get_time();
            }
        }
        
        self.y -= self.speed * dt;
    }
    
    pub fn draw(&self, player : &Player) {
        for x in -8..8 {
            for y in 0..5 {
                let x_offset = (player.rect.x / 200.0).floor() * 200.0;
                
                draw_texture_ex(&self.texture, x as f32 * 200.0 + x_offset,(y as f32) * 200.0 + self.y , WHITE, DrawTextureParams {
                    dest_size: Some(vec2(200.0, 200.0)),
                    ..Default::default()
                });
            }
        }
    }
}