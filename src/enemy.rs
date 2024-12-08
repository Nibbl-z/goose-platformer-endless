use macroquad::prelude::*;
use crate::{collision::Rect, player::Player};

pub struct Enemy {
    pub rect: Rect,
    direction: bool,
    texture : Texture2D,
    recorded_positions: Vec<(f32, f32, bool)>,
    delay: usize
}

impl Enemy {
    pub async fn new() -> Enemy {
        Enemy {
            rect: Rect { x: 0.0, y: 0.0, w: 50.0, h: 50.0 },
            direction: false,
            texture: load_texture("img/enemy.png").await.unwrap(),
            recorded_positions: Vec::new(),
            delay: 100,
        }
    }
    
    pub fn update(&mut self, player: &Player) {
        self.recorded_positions.push((player.rect.x, player.rect.y, player.direction));

        // If we have enough recorded positions, update the ghost's position
        if self.recorded_positions.len() > self.delay {
            let (x, y, direction) = self.recorded_positions.remove(0);
            self.rect.x = x;
            self.rect.y = y;
            self.direction = direction
        }
    }
    
    pub fn draw(&self) {
        draw_texture_ex(&self.texture, self.rect.x, self.rect.y, WHITE, DrawTextureParams {
            flip_x: self.direction,
            flip_y: false,
            ..Default::default()
        });
    }
}