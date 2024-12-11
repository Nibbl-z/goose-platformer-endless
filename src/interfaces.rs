use macroquad::prelude::*;
use crate::{collision::Rect, player::Player};

pub struct GameOver {
    text_fade : f32,
    bg_fade : f32,
    text_image : Texture2D,
    btn_tex : Texture2D,
    btn_tex_hover : Texture2D
}

impl GameOver {
    pub async fn init() -> GameOver { 
        GameOver {
            text_fade : 0.0,
            bg_fade : 0.0,
            text_image : load_texture("img/you_died.png").await.unwrap(),
            btn_tex : load_texture("img/btn_restart.png").await.unwrap(),
            btn_tex_hover : load_texture("img/btn_restart_hover.png").await.unwrap(),
        }
    }

    pub fn reset(&mut self) {
        self.text_fade = 0.0;
        self.bg_fade = 0.0;
    }
    
    pub fn draw(&self, player : &Player) {
        draw_rectangle(-5000.0,  -5000.0, 10000.0, 10000.0, Color::new(1.0, 0.0, 0.0, self.bg_fade));
        draw_texture(
            &self.text_image, 
            (screen_width() - &self.text_image.width()) / 2.0, 
            (screen_height() - &self.text_image.height()) / 2.0 - 150.0, 
            Color::new(1.0, 1.0, 1.0, self.text_fade)
        );
        
        let mouse_rect = Rect {x : mouse_position().0, y : mouse_position().1, w : 2.0, h : 2.0};
        let btn_rect = Rect {
            x : (screen_width() - &self.text_image.width()) / 2.0,
            y : (screen_height() - &self.text_image.height()) / 2.0 + 200.0,
            w : 500.0,
            h : 150.0
        };

        let texture = {
            if btn_rect.collides_with(&mouse_rect) {
                &self.btn_tex_hover
            } else {
                &self.btn_tex
            }
        };
        
        draw_texture(
            texture, 
            (screen_width() - &self.text_image.width()) / 2.0, 
            (screen_height() - &self.text_image.height()) / 2.0 + 200.0, 
            WHITE
        );
        
        draw_text(
            &format!("FINAL SCORE: {}", player.score), 
            (screen_width() - &self.text_image.width()) / 2.0, 
            (screen_height() - &self.text_image.height()) / 2.0 + 450.0,
            70.0,
            Color::new(1.0, 1.0, 1.0, self.text_fade)
        );

        
    }
    
    pub fn update(&mut self, start_time : f64) -> bool {
        let elapsed_time = macroquad::time::get_time() - start_time;
        let progress = (elapsed_time / 1.0).clamp(0.0, 1.0) as f32;

        self.text_fade = progress;
        self.bg_fade = progress;
        
        let mouse_rect = Rect {x : mouse_position().0, y : mouse_position().1, w : 2.0, h : 2.0};
        let btn_rect = Rect {
            x : (screen_width() - &self.text_image.width()) / 2.0,
            y : (screen_height() - &self.text_image.height()) / 2.0 + 200.0,
            w : 500.0,
            h : 150.0
        };

        if btn_rect.collides_with(&mouse_rect) {
            if is_mouse_button_pressed(MouseButton::Left) {
                return true;
            }
        }
        
        false
    }
}
