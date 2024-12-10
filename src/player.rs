use macroquad::prelude::*;
use crate::collision::Rect;
use crate::map::Platform;

pub struct Player {
    pub rect : Rect,
    dx : f32,
    dy : f32,
    jump_power: f32,
    speed: f32,
    coyote_time : u32,
    pub on_ground: bool,
    pub touching_wall : i8,
    pub direction : bool,
    texture : Texture2D,
    current_platform : Option<(f32, f32)>,
    pub died : bool
}

const GRAVITY : f32 = 1000.0;
const DRAG : f32 = 0.999;
const PLATFORM_DRAG : f32 = 0.6;
const ACCEL : f32 = 0.1;

impl Player {
    pub async fn new() -> Player {
        Player {
            dx : 0.0,
            dy : 0.0,
            rect : Rect{x : 0.0, y : 0.0, w : 50.0, h : 50.0},
            jump_power : -700.0,
            speed : 100000.0,
            on_ground : false,
            coyote_time : 0,
            touching_wall : 0,
            direction : false,
            texture : load_texture("img/player.png").await.unwrap(),
            current_platform : None,
            died : false
        }
    }
    
    pub fn draw(&self) {
        draw_texture_ex(&self.texture, self.rect.x, self.rect.y, WHITE, DrawTextureParams {
            flip_x: self.direction,
            flip_y: false,
            ..Default::default()
        });
    }
    
    pub fn fixed_update(&mut self) {
        
        if self.coyote_time > 0 {
            self.coyote_time -= 1;
        }
        
    }
    
    pub fn update(&mut self, dt : f32) {
        if is_key_down(KeyCode::D) && self.touching_wall != -1 {
            if self.touching_wall == 1 {
                self.coyote_time = 5
            }
            self.touching_wall = 0;
            self.dx = self.speed * dt;
            self.direction = true;
        }

        if is_key_down(KeyCode::A) && self.touching_wall != 1 {
            if self.touching_wall == -1 {
                self.coyote_time = 5
            }
            self.touching_wall = 0;
            self.dx = -self.speed * dt;
            self.direction = false;
        }

        if is_key_pressed(KeyCode::Space) && (self.on_ground || self.touching_wall != 0 || self.coyote_time != 0) {
            self.dy = self.jump_power;
            self.on_ground = false;
        }

        if !self.on_ground {
            self.dy += GRAVITY * dt;
            self.dy *= DRAG; 
        } else {
            self.dy = 0.0;
        }
        
        self.dx *= PLATFORM_DRAG;
        
        if let Some(platform) = &self.current_platform {
            if self.rect.x < platform.0 || self.rect.x > platform.1 { self.on_ground = false }
        }
        self.rect.x += self.dx * dt;
        self.rect.y += self.dy * dt;
        
        self.touching_wall = 0;
    }
    
    
    pub fn land(&mut self, platform : &Platform) {
        if self.dy < 0.0 { return; }
        if self.rect.x <= platform.rect.x - 40.0 {return;}
        if self.rect.x >= platform.rect.x + platform.rect.w - 10.0 {return;}
        
        self.rect.y = platform.rect.y - 50.0;
        self.on_ground = true;
        self.touching_wall = 0;
        self.current_platform = Some((platform.rect.x, platform.rect.x + platform.rect.w));
    }
    
    pub fn hit_side_wall(&mut self, platform : &Platform) {
        if self.rect.x <= platform.rect.x {
            self.touching_wall = -1;
            self.dx = 0.0;
        } else {
            self.touching_wall = 1;
            self.dx = 0.0;
        }
    }
    
    pub fn hit_head(&mut self) {
        self.dy = 10.0;
    }
}