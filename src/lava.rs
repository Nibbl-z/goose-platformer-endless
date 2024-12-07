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
    
    pub fn update(&mut self, dt : f32, player : &Player) {
        self.speed = if player.rect.y <= self.y - 600.0 {
            DEFAULT_SPEED * 7.0
        } else {
            DEFAULT_SPEED
        };
        

        self.y -= self.speed * dt;
    }
    
    pub fn draw(&self) {
        for x in -500..500 {
            for y in 0..20 {
                draw_texture_ex(&self.texture, x as f32 * 200.0,(y as f32) * 200.0 + self.y , WHITE, DrawTextureParams {
                    dest_size: Some(vec2(200.0, 200.0)),
                    ..Default::default()
                });
            }
        }
        
        draw_texture_ex(&self.texture, 200.0, 200.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(200.0, 200.0)),
            ..Default::default()
        });
    }
}