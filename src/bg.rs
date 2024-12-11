use macroquad::prelude::*;
use crate::player::Player;

pub struct BG {
    bg_texture : Texture2D
}

impl BG {
    pub async fn new() -> BG {
        BG {
            bg_texture : load_texture("img/bg.png").await.unwrap()
        }
    }

    pub fn draw(&self, player : &Player) {
        for x in -10..10 {
            for y in -10..10 {
                let x_offset = (player.rect.x / 400.0).floor() * 200.0;
                let y_offset = (player.rect.y / 400.0).floor() * 200.0;
                draw_texture(
                    &self.bg_texture, 
                    x as f32 * 200.0 + x_offset + player.rect.x * 0.5,
                    (y as f32) * 200.0 + y_offset + player.rect.y * 0.5, 
                    WHITE);
            }
        }
    }
}