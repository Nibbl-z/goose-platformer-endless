use macroquad::prelude::*;
use crate::collision::Rect;
use crate::map::Platform;

pub struct Player {
    pub rect : Rect,
    dx : f32,
    dy : f32,
    jump_power: f32,
    speed: f32,
    pub on_ground: bool,
    pub direction : bool,
    texture : Texture2D,
    current_platform : Option<Vec<f32>>
}

const GRAVITY : f32 = 6.0;
const DRAG : f32 = 0.999;
const PLATFORM_DRAG : f32 = 0.9;

impl Player {
    pub async fn new() -> Player {
        Player {
            dx : 0.0,
            dy : 0.0,
            rect : Rect{x : 0.0, y : 0.0, w : 50.0, h : 50.0},
            jump_power : -700.0,
            speed : 50.0,
            on_ground : false,
            direction : false,
            texture : load_texture("img/player.png").await.unwrap(),
            current_platform : None
        }
    }
    
    pub fn draw(&self) {
        draw_texture_ex(&self.texture, self.rect.x, self.rect.y, WHITE, DrawTextureParams {
            flip_x: self.direction,
            flip_y: false,
            ..Default::default()
        });
    }

    pub fn update(&mut self, dt : f32) {
        if is_key_down(KeyCode::D) {
            self.dx += self.speed;
            self.direction = true;
        }

        if is_key_down(KeyCode::A) {
            self.dx -= self.speed;
            self.direction = false;
        }

        if is_key_down(KeyCode::Space) && self.on_ground {
            self.dy = self.jump_power;
            self.on_ground = false;
        }

        if !self.on_ground {
            self.dy += GRAVITY;
            self.dy *= DRAG; 
        } else {
            self.dy = 0.0;
        }

        self.dx *= PLATFORM_DRAG;
        
        if let Some(platform) = &self.current_platform {
            if self.rect.x < platform[0] || self.rect.x > platform[1] { self.on_ground = false }
        }

        self.rect.y += self.dy * dt;
        self.rect.x += self.dx * dt;
    }
    
    pub fn land(&mut self, platform : &Platform) {
        self.rect.y = platform.rect.y - platform.rect.h;
        self.on_ground = true;
        self.current_platform = Some(vec![platform.rect.x, platform.rect.x + platform.rect.w]);
    }
}